use core::convert::TryFrom;
use pinocchio::{
  account_info::AccountInfo,
  instruction::{Seed, Signer},
  program_error::ProgramError,
  sysvars::{rent::Rent, Sysvar},
  ProgramResult,
};
use pinocchio_log::log;
use pinocchio_system::instructions::CreateAccount;

use crate::{
  check_ata, check_decimals, check_from_balc, check_mint0a, check_pda, check_sysprog, data_len,
  derive_pda1, executable, instructions::check_signer, none_zero_u64, parse_u64, rent_exempt22,
  writable, Ee, ACCOUNT_DISCRIMINATOR_SIZE, VAULT_SEED,
};

/// TokLgc: Users to Deposit Tokens
pub struct TokLgcDeposit<'a> {
  pub user: &'a AccountInfo, //signer
  pub from_ata: &'a AccountInfo,
  pub to_ata: &'a AccountInfo,
  pub to_wallet: &'a AccountInfo,
  pub mint: &'a AccountInfo,
  pub token_program: &'a AccountInfo,
  pub system_program: &'a AccountInfo,
  pub atoken_program: &'a AccountInfo,
  pub decimals: u8,
  pub amount: u64,
}
impl<'a> TokLgcDeposit<'a> {
  pub const DISCRIMINATOR: &'a u8 = &5;

  pub fn process(self) -> ProgramResult {
    let TokLgcDeposit {
      user,
      from_ata,
      to_ata,
      to_wallet,
      mint,
      token_program,
      system_program,
      atoken_program: _,
      decimals,
      amount,
    } = self;
    log!("TokLgcDeposit process()");
    rent_exempt22(mint, 0)?;
    check_decimals(mint, decimals)?;
    check_mint0a(mint, token_program)?;

    log!("TokLgcDeposit 5");
    if to_wallet.lamports() == 0 {
      log!("TokLgcDeposit 6: make to_wallet");
      let (expected_vault_pda, bump) = derive_pda1(user, VAULT_SEED)?;
      if to_wallet.key() != &expected_vault_pda {
        return Ee::VaultPDA.e();
      }
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
        to: to_wallet,
        lamports: needed_lamports,
        space: VAULT_SIZE as u64,
        owner: &crate::ID,
      }
      .invoke_signed(&[signer])?;
      log!("TokLgcDeposit 6b");
    }
    check_pda(to_wallet)?;
    log!("TokLgcDeposit 7: to_wallet is verified");

    if to_ata.data_is_empty() {
      log!("Make to_ata");
      pinocchio_associated_token_account::instructions::Create {
        funding_account: user,
        account: to_ata,
        wallet: to_wallet,
        mint,
        system_program,
        token_program,
      }
      .invoke()?;
      //Please upgrade to SPL Token 2022 for immutable owner support
    } else {
      log!("to_ata has data");
      check_ata(to_ata, to_wallet, mint)?;
    }
    writable(to_ata)?;
    rent_exempt22(to_ata, 1)?;
    log!("ToATA is found/verified");

    log!("Transfer Tokens");
    pinocchio_token::instructions::TransferChecked {
      from: from_ata,
      mint,
      to: to_ata,
      authority: user,
      amount,
      decimals,
    }
    .invoke()?;
    /*  pinocchio_token::instructions::Transfer {
        from: vault,
        to: to_ata,
        authority: escrow,
        amount: vault_account.amount(),
    }.invoke_signed(&[seeds.clone()])?; */
    Ok(())
  }
}
impl<'a> TryFrom<(&'a [u8], &'a [AccountInfo])> for TokLgcDeposit<'a> {
  type Error = ProgramError;

  fn try_from(value: (&'a [u8], &'a [AccountInfo])) -> Result<Self, Self::Error> {
    log!("TokLgcDeposit try_from");
    let (data, accounts) = value;
    log!("accounts len: {}, data len: {}", accounts.len(), data.len());

    let [user, from_ata, to_ata, to_wallet, mint, token_program, system_program, atoken_program] =
      accounts
    else {
      return Err(ProgramError::NotEnoughAccountKeys);
    };
    check_signer(user)?;
    executable(token_program)?;
    check_sysprog(system_program)?;
    //TODO: check ATOKEN
    writable(from_ata)?;
    check_ata(from_ata, user, mint)?;

    //1+8: u8 takes 1, u64 takes 8 bytes
    data_len(data, 9)?;
    let decimals = data[0];
    let amount = parse_u64(&data[1..])?;
    log!("decimals: {}, amount: {}", decimals, amount);

    none_zero_u64(amount)?;
    check_from_balc(from_ata, amount)?;

    Ok(Self {
      user,
      from_ata,
      to_ata,
      to_wallet,
      mint,
      token_program,
      system_program,
      atoken_program,
      decimals,
      amount,
    })
  }
}
