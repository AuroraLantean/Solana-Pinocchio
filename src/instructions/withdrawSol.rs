use core::convert::TryFrom;
use pinocchio::{
    account_info::AccountInfo,
    program_error::ProgramError,
    sysvars::{rent::Rent, Sysvar},
    ProgramResult,
};
use pinocchio_log::log;

use crate::instructions::{check_pda, check_signer, derive_pda1, parse_u64};

//  vault is owned by the program, matches the PDA derived from user. The withdrawn amount is everything above the rent minimum.
pub struct WithdrawSol<'a> {
    pub user: &'a AccountInfo,
    pub vault: &'a AccountInfo,
    pub amount: u64,
}
impl<'a> WithdrawSol<'a> {
    pub const DISCRIMINATOR: &'a u8 = &1;

    /// Transfer lamports from the vault PDA to the user, leaving the rent minimum in place.
    pub fn process(self) -> ProgramResult {
        let WithdrawSol {
            user,
            vault,
            amount,
        } = self;
        check_signer(user)?;

        // Validate the vault is owned by the program
        check_pda(vault)?;

        let (expected_vault_pda, _bump) = derive_pda1(user, b"vault")?;
        if vault.key() != &expected_vault_pda {
            return Err(ProgramError::InvalidAccountData);
        }

        // Compute how much can be withdrawn while keeping the account rent-exempt
        let data_len = vault.data_len();
        let min_balance = Rent::get()?.minimum_balance(data_len);

        log!("withdraw amt: {}", amount);
        let current = vault.lamports();
        log!("vault balc: {}", current);
        if current <= min_balance {
            return Err(ProgramError::AccountNotRentExempt);
        }
        if current
            <= min_balance
                .checked_add(amount)
                .ok_or(ProgramError::ArithmeticOverflow)?
        {
            return Err(ProgramError::InsufficientFunds);
        }

        // Transfer SOL from vault to user
        {
            let mut vault_lamports = vault.try_borrow_mut_lamports()?;

            *vault_lamports = vault_lamports
                .checked_sub(amount)
                .ok_or(ProgramError::InsufficientFunds)?;
        }

        {
            let mut admin_lamports = user.try_borrow_mut_lamports()?;

            *admin_lamports = admin_lamports
                .checked_add(amount)
                .ok_or(ProgramError::InsufficientFunds)?;
        }
        log!("{} lamports withdrawn from vault", amount);
        Ok(())
    }
}
impl<'a> TryFrom<(&'a [u8], &'a [AccountInfo])> for WithdrawSol<'a> {
    type Error = ProgramError;

    fn try_from(value: (&'a [u8], &'a [AccountInfo])) -> Result<Self, Self::Error> {
        let (data, accounts) = value;
        if accounts.len() < 2 {
            return Err(ProgramError::NotEnoughAccountKeys);
        }
        let user = &accounts[0];
        let vault = &accounts[1];

        let amount = parse_u64(data)?;
        Ok(Self {
            user,
            vault,
            amount,
        })
    }
}
