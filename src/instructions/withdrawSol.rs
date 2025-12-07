use core::convert::TryFrom;
use pinocchio::{
    account_info::AccountInfo,
    program_error::ProgramError,
    sysvars::{rent::Rent, Sysvar},
    ProgramResult,
};
use pinocchio_log::log;

use crate::instructions::{check_pda, check_signer, derive_vault_pda, parse_amount_u64};

//  vault is owned by the program, matches the PDA derived from the owner, and the owner is the signer of the withdraw transaction. The withdrawn amount is everything above the rent minimum.
pub struct WithdrawSol<'a> {
    pub owner: &'a AccountInfo,
    pub vault: &'a AccountInfo,
    pub amount: u64,
}
impl<'a> WithdrawSol<'a> {
    pub const DISCRIMINATOR: &'a u8 = &1;

    /// Transfer lamports from the vault PDA to the owner, leaving the rent minimum in place.
    pub fn process(self) -> ProgramResult {
        let WithdrawSol {
            owner,
            vault,
            amount,
        } = self;
        // Validate owner is signer
        check_signer(owner)?;

        // Validate the vault is owned by the program
        check_pda(vault)?;

        // Validate the vault is the correct PDA for this owner
        let (expected_vault_pda, _bump) = derive_vault_pda(owner)?;
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

        // Transfer SOL from vault to owner
        {
            let mut vault_lamports = vault.try_borrow_mut_lamports()?;

            *vault_lamports = vault_lamports
                .checked_sub(amount)
                .ok_or(ProgramError::InsufficientFunds)?;
        }

        {
            let mut owner_lamports = owner.try_borrow_mut_lamports()?;

            *owner_lamports = owner_lamports
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
        let owner = &accounts[0];
        let vault = &accounts[1];

        let amount = parse_amount_u64(data)?;
        Ok(Self {
            owner,
            vault,
            amount,
        })
    }
}
