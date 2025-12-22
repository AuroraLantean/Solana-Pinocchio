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

use crate::{
  instructions::{check_pda, check_signer, derive_pda1, parse_u64},
  MyError, ACCOUNT_DISCRIMINATOR_SIZE, VAULT_SEED,
};

// Deposit SOL to program PDA
// make and rent-funds the vault PDA
// check the PDA exists and is owned by the program
// transfer the SOL amount to the vault

//Deposit Accounts
pub struct DepositSol<'a> {
  pub user: &'a AccountInfo,
  pub vault: &'a AccountInfo,
  pub amount: u64,
}
impl<'a> DepositSol<'a> {
  pub const DISCRIMINATOR: &'a u8 = &0;

  pub fn process(self) -> ProgramResult {
    let DepositSol {
      user,
      vault,
      amount,
    } = self;

    ensure_deposit_accounts(user, vault)?;

    SystemTransfer {
      from: user,
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
    let [user, vault, _systemProgram] = accounts else {
      return Err(ProgramError::NotEnoughAccountKeys);
    };
    let amount = parse_u64(data)?;
    Ok(Self {
      user,
      vault,
      amount,
    })
  }
}

/// Ensure the vault exists; if not, create it with PDA seeds. user must be a signer, vault must be writable, and rent minimum must be respected for creation.
fn ensure_deposit_accounts(user: &AccountInfo, vault: &AccountInfo) -> ProgramResult {
  check_signer(user)?;

  // Create when empty and fund rent-exempt.
  if vault.lamports() == 0 {
    let (expected_vault_pda, bump) = derive_pda1(user, VAULT_SEED)?;
    if vault.key() != &expected_vault_pda {
      return Err(MyError::VaultPDA.into());
    }
    //assert_eq!(&expected_vault_pda, vault.key());

    let signer_seeds = [
      Seed::from(VAULT_SEED),
      Seed::from(user.key().as_ref()),
      Seed::from(core::slice::from_ref(&bump)),
    ];
    let signer = Signer::from(&signer_seeds);

    // Make the account rent-exempt.
    const VAULT_SIZE: usize = ACCOUNT_DISCRIMINATOR_SIZE + size_of::<u64>();
    let needed_lamports = Rent::get()?.minimum_balance(VAULT_SIZE);

    CreateAccount {
      from: user,
      to: vault,
      lamports: needed_lamports,
      space: VAULT_SIZE as u64,
      owner: &crate::ID,
    }
    .invoke_signed(&[signer])?;

    log!("Vault created");
  } else {
    // If vault already exists
    check_pda(vault)?;
    log!("Vault already exists");
  }
  Ok(())
}
