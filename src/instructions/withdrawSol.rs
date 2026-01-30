use core::convert::TryFrom;
use pinocchio::{error::ProgramError, AccountView, ProgramResult};
use pinocchio_log::log;

use crate::{
  instructions::{check_pda, check_signer, derive_pda1, parse_u64},
  none_zero_u64, rent_exempt, Ee, VAULT_SEED,
};

//  vault is owned by the program, matches the PDA derived from user. The withdrawn amount is everything above the rent minimum.
pub struct WithdrawSol<'a> {
  pub user: &'a AccountView,
  pub vault: &'a AccountView,
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
    log!("withdrawSol process()");

    // Transfer SOL from vault to user
    {
      let from_lam = vault
        .lamports()
        .checked_sub(amount)
        .ok_or_else(|| ProgramError::InsufficientFunds)?;
      vault.set_lamports(from_lam);

      let sum_lam = user
        .lamports()
        .checked_add(amount)
        .ok_or_else(|| ProgramError::ArithmeticOverflow)?;
      user.set_lamports(sum_lam);
    }
    log!("{} lamports withdrawn from vault", amount);
    Ok(())
  }
}
impl<'a> TryFrom<(&'a [u8], &'a [AccountView])> for WithdrawSol<'a> {
  type Error = ProgramError;

  fn try_from(value: (&'a [u8], &'a [AccountView])) -> Result<Self, Self::Error> {
    log!("WithdrawSol try_from");
    let (data, accounts) = value;
    log!("accounts len: {}, data len: {}", accounts.len(), data.len());

    let [user, vault] = accounts else {
      return Err(ProgramError::NotEnoughAccountKeys);
    };
    check_signer(user)?;
    check_pda(vault)?;

    let amount = parse_u64(data)?;
    none_zero_u64(amount)?;

    let (expected_vault_pda, _bump) = derive_pda1(user.address(), VAULT_SEED)?;
    if vault.address() != &expected_vault_pda {
      return Err(Ee::VaultPDA.into());
    }

    // Compute how much can be withdrawn while keeping the account rent-exempt
    let (vault_balc, vault_min_balc) = rent_exempt(vault)?;
    log!("withdraw amt: {}", amount);
    log!("vault balc: {}", vault_balc);
    if vault_balc < amount {
      return Err(ProgramError::InsufficientFunds);
    }

    log!("check vault balc 2");
    if vault_balc
      <= vault_min_balc
        .checked_add(amount)
        .ok_or_else(|| ProgramError::ArithmeticOverflow)?
    {
      return Err(Ee::PdaToBeBelowRentExempt.into());
    }
    Ok(Self {
      user,
      vault,
      amount,
    })
  }
}
