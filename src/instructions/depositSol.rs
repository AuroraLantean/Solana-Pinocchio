use core::convert::TryFrom;
use core::mem::size_of;
use pinocchio::{
    account_info::AccountInfo,
    instruction::{Seed, Signer},
    program_error::ProgramError,
    sysvars::{rent::Rent, Sysvar},
    ProgramResult,
};
use pinocchio_log::log;
use pinocchio_system::instructions::{CreateAccount, Transfer as SystemTransfer};

use crate::instructions::{check_pda, check_signer, derive_vault_pda, parse_amount_u64};

// Deposit SOL to program PDA
// make and rent-funds the vault PDA
// check the PDA exists and is owned by the program
// transfer the SOL amount to the vault

//Deposit Accounts
pub struct DepositSol<'a> {
    pub owner: &'a AccountInfo,
    pub vault: &'a AccountInfo,
    pub amount: u64,
}
impl<'a> DepositSol<'a> {
    pub const DISCRIMINATOR: &'a u8 = &0;

    pub fn process(self) -> ProgramResult {
        let DepositSol {
            owner,
            vault,
            amount,
        } = self;

        ensure_deposit_accounts(owner, vault)?;

        SystemTransfer {
            from: owner,
            to: vault,
            lamports: amount,
        }
        .invoke()?;
        log!("{} Lamports deposited to vault", amount);
        Ok(())
    }
}
impl<'a> TryFrom<(&'a [u8], &'a [AccountInfo])> for DepositSol<'a> {
    type Error = ProgramError;

    fn try_from(value: (&'a [u8], &'a [AccountInfo])) -> Result<Self, Self::Error> {
        let (data, accounts) = value;
        if accounts.len() < 2 {
            return Err(ProgramError::NotEnoughAccountKeys);
        }
        let owner = &accounts[0];
        let vault = &accounts[1];
        //let [owner, vault, _system_program, _] = accounts else { return Err(ProgramError::NotEnoughAccountKeys);}

        let amount = parse_amount_u64(data)?;
        Ok(Self {
            owner,
            vault,
            amount,
        })
    }
}

/// Ensure the vault exists; if not, create it with PDA seeds. owner must be a signer, vault must be writable, and rent minimum must be respected for creation.
fn ensure_deposit_accounts(owner: &AccountInfo, vault: &AccountInfo) -> ProgramResult {
    check_signer(owner)?;

    // Create when empty and fund rent-exempt.
    if vault.lamports() == 0 {
        const ACCOUNT_DISCRIMINATOR_SIZE: usize = 8;

        let (expected_vault_pda, bump) = derive_vault_pda(owner)?;
        if vault.key() != &expected_vault_pda {
            return Err(ProgramError::InvalidAccountData);
        }
        //assert_eq!(&expected_vault_pda, vault.key());

        let signer_seeds = [
            Seed::from(b"vault".as_slice()),
            Seed::from(owner.key().as_ref()),
            Seed::from(core::slice::from_ref(&bump)),
        ];
        let signer = Signer::from(&signer_seeds);

        // Make the account rent-exempt.
        const VAULT_SIZE: usize = ACCOUNT_DISCRIMINATOR_SIZE + size_of::<u64>();
        let needed_lamports = Rent::get()?.minimum_balance(VAULT_SIZE);

        CreateAccount {
            from: owner,
            to: vault,
            lamports: needed_lamports,
            space: VAULT_SIZE as u64,
            owner: &crate::ID,
        }
        .invoke_signed(&[signer])?;

        log!("Vault created");
    } else {
        // If vault already exists, validate owner matches the program.
        check_pda(vault)?;
        log!("Vault already exists");
    }
    Ok(())
}
