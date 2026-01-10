use core::convert::TryFrom;
use pinocchio::{account_info::AccountInfo, program_error::ProgramError, ProgramResult};
use pinocchio_log::log;

use crate::{
  check_mint22a, check_sysprog, executable, initialized, instructions::check_signer,
  not_initialized, rent_exempt_mint22, writable,
};

/// Token2022 Init ATA(Associated Token Account)
pub struct Token2022InitAta<'a> {
  pub payer: &'a AccountInfo,
  pub to_wallet: &'a AccountInfo,
  pub mint: &'a AccountInfo,
  pub ata: &'a AccountInfo,
  pub token_program: &'a AccountInfo,
  pub system_program: &'a AccountInfo,
  pub atoken_program: &'a AccountInfo,
}
impl<'a> Token2022InitAta<'a> {
  pub const DISCRIMINATOR: &'a u8 = &10;

  pub fn process(self) -> ProgramResult {
    let Token2022InitAta {
      payer,
      to_wallet,
      mint,
      ata,
      token_program,
      system_program,
      atoken_program: _,
    } = self;
    log!("Token2022InitAta process()");
    pinocchio_associated_token_account::instructions::Create {
      funding_account: payer, // Keypair
      account: ata,
      wallet: to_wallet,
      mint: mint,
      system_program: system_program,
      token_program: token_program,
    }
    .invoke()?;
    /*pinocchio_token_2022::instructions::InitializeAccount3 {
        account: ata,
        mint: mint,
        owner: to_wallet.key(),
        token_program: token_program.key(),
    }
    .invoke()?;//invalid account data for instruction*/
    Ok(())
  }
  pub fn init_if_needed(self) -> ProgramResult {
    if self.ata.lamports() == 0 {
      Self::process(self)?;
    }
    Ok(())
  }
}
impl<'a> TryFrom<(&'a [u8], &'a [AccountInfo])> for Token2022InitAta<'a> {
  type Error = ProgramError;

  fn try_from(value: (&'a [u8], &'a [AccountInfo])) -> Result<Self, Self::Error> {
    log!("Token2022InitAta try_from");
    let (data, accounts) = value;
    log!("accounts len: {}, data len: {}", accounts.len(), data.len());

    let [payer, to_wallet, mint, ata, token_program, system_program, atoken_program] = accounts
    else {
      return Err(ProgramError::NotEnoughAccountKeys);
    };
    check_signer(payer)?;
    executable(token_program)?;
    check_sysprog(system_program)?;
    //check_pda(config_pda)?;
    not_initialized(ata)?;
    writable(ata)?;
    initialized(to_wallet)?;
    log!("Token2022InitAta try_from 3");
    rent_exempt_mint22(mint)?;
    check_mint22a(mint, token_program)?;

    Ok(Self {
      payer,
      to_wallet,
      mint,
      ata,
      token_program,
      system_program,
      atoken_program,
    })
  }
}
