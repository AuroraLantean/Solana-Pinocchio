use core::convert::TryFrom;
use pinocchio::{
  cpi::{Seed, Signer},
  sysvars::{rent::Rent, Sysvar},
  AccountView, Address, ProgramResult,
};
use pinocchio_log::log;
use pinocchio_system::instructions::Transfer as SystemTransfer;

use crate::{
  check_sysprog,
  instructions::{check_pda, check_signer, derive_pda1, parse_u64},
  none_zero_u64, sol_balc, Ee, ID, PROG_ADDR, VAULT_SEED, VAULT_SIZE,
};

// Deposit SOL to program PDA
// make and rent-funds the vault PDA
// check the PDA exists and is owned by the program
// transfer the SOL amount to the vault

//Deposit Accounts
pub struct DepositSol<'a> {
  pub user: &'a AccountView,
  pub vault: &'a AccountView,
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
    log!("DepositSol process");
    ensure_deposit_accounts(user, vault)?;

    log!("DepositSol 2");
    SystemTransfer {
      from: user,
      to: vault,
      lamports: amount,
    }
    .invoke()?;
    log!("success: {} Lamports deposited to vault", amount);
    Ok(())
  }
}
impl<'a> TryFrom<(&'a [u8], &'a [AccountView])> for DepositSol<'a> {
  type Error = ProgramError;

  fn try_from(value: (&'a [u8], &'a [AccountView])) -> Result<Self, Self::Error> {
    log!("DepositSol try_from");
    let (data, accounts) = value;
    log!("accounts len: {}, data len: {}", accounts.len(), data.len());

    let [user, vault, system_program] = accounts else {
      return Err(ProgramError::NotEnoughAccountKeys);
    };
    check_signer(user)?;
    check_sysprog(system_program)?;

    log!("DepositSol data: {}", data);
    let amount = parse_u64(data)?;
    log!("amount: {}", amount);
    none_zero_u64(amount)?;
    sol_balc(user, amount)?;

    Ok(Self {
      user,
      vault,
      amount,
    })
  }
}

/// Ensure the vault exists; if not, create it with PDA seeds. user must be a signer, vault must be writable, and rent minimum must be respected for creation.
fn ensure_deposit_accounts(user: &AccountView, vault: &AccountView) -> ProgramResult {
  log!("ensure_deposit_accounts");
  // Create when empty and fund rent-exempt.
  if vault.lamports() == 0 {
    let (expected_vault_pda, bump) = derive_pda1(user.key(), VAULT_SEED)?;
    if vault.key() != &expected_vault_pda {
      return Ee::VaultPDA.e();
    }
    let signer_seeds = [
      Seed::from(VAULT_SEED),
      Seed::from(user.key().as_ref()),
      Seed::from(core::slice::from_ref(&bump)),
    ];
    let signer = Signer::from(&signer_seeds);

    // Make the account rent-exempt.
    let needed_lamports = Rent::get()?.minimum_balance(VAULT_SIZE);
    log!("needed_lamports: {}", needed_lamports);
    log!("VAULT_SIZE: {}", VAULT_SIZE);

    pinocchio_system::instructions::CreateAccount {
      from: user,
      to: vault,
      lamports: needed_lamports,
      space: VAULT_SIZE as u64,
      owner: &PROG_ADDR,
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
