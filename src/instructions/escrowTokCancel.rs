use core::convert::TryFrom;
use pinocchio::{
  account_info::AccountInfo,
  instruction::{Seed, Signer},
  program_error::ProgramError,
  ProgramResult,
};
use pinocchio_log::log;
use pinocchio_token::state::TokenAccount;

use crate::{
  check_ata, check_atoken_gpvbd, check_decimals, check_mint0a, check_sysprog, data_len, executable,
  instructions::check_signer, none_zero_u64, rent_exempt_mint, rent_exempt_tokacct, writable,
  Config, Ee, Escrow,
};
//TODO: add Token2022 interface
/// Make Withdraw Escrow Token Y
pub struct EscrowTokCancel<'a> {
  pub maker: &'a AccountInfo, //signer
  pub maker_ata_x: &'a AccountInfo,
  pub maker_ata_y: &'a AccountInfo,
  pub escrow_ata_x: &'a AccountInfo,
  pub escrow_ata_y: &'a AccountInfo,
  pub mint_x: &'a AccountInfo,
  pub mint_y: &'a AccountInfo,
  pub escrow_pda: &'a AccountInfo,
  pub config_pda: &'a AccountInfo,
  pub token_program: &'a AccountInfo,
  pub system_program: &'a AccountInfo,
  pub atoken_program: &'a AccountInfo,
}
impl<'a> EscrowTokCancel<'a> {
  pub const DISCRIMINATOR: &'a u8 = &18;

  pub fn process(self) -> ProgramResult {
    let EscrowTokCancel {
      maker,
      maker_ata_x,
      maker_ata_y,
      escrow_ata_x,
      escrow_ata_y,
      mint_x,
      mint_y,
      escrow_pda,
      config_pda,
      token_program,
      system_program,
      atoken_program: _,
    } = self;
    log!("---------== process()");
    config_pda.can_borrow_mut_data()?;
    let _config: &mut Config = Config::from_account_info(&config_pda)?;

    escrow_pda.can_borrow_mut_data()?;
    let escrow: &mut Escrow = Escrow::from_account_info(&escrow_pda)?;

    log!("Check args against EscrowPDA fields");
    let id = escrow.id();
    let bump = escrow.bump();
    if maker.key().ne(escrow.maker()) {
      return Ee::OnlyMaker.e();
    }
    if escrow.mint_x().ne(mint_x.key()) {
      return Ee::EscrowMintX.e();
    }
    if escrow.mint_y().ne(mint_y.key()) {
      return Ee::EscrowMintY.e();
    }

    let decimal_x = escrow.decimal_x();
    log!("decimal_: {}", decimal_x);
    let amount_y = escrow.amount_y();
    let decimal_y = escrow.decimal_y();
    log!("decimal_y: {}, amount_y: {}", decimal_y, amount_y);
    none_zero_u64(amount_y)?;
    check_decimals(mint_x, decimal_x)?;
    check_decimals(mint_y, decimal_y)?;

    let escrow_ata_y_info = TokenAccount::from_account_info(escrow_ata_y)?;
    if escrow_ata_y_info.amount() >= amount_y {
      return Ee::MakerToWithdrawTokenY.e();
    }
    drop(escrow_ata_y_info);

    //TODO: withdraw both token X and token Y if exists
    Ok(())
  }
}
impl<'a> TryFrom<(&'a [u8], &'a [AccountInfo])> for EscrowTokCancel<'a> {
  type Error = ProgramError;

  fn try_from(value: (&'a [u8], &'a [AccountInfo])) -> Result<Self, Self::Error> {
    log!("EscrowTokCancel try_from");
    let (data, accounts) = value;
    log!("accounts len: {}, data len: {}", accounts.len(), data.len());
    data_len(data, 0)?;

    let [maker, maker_ata_x, maker_ata_y, escrow_ata_x, escrow_ata_y, mint_x, mint_y, escrow_pda, config_pda, token_program, system_program, atoken_program] =
      accounts
    else {
      return Err(ProgramError::NotEnoughAccountKeys);
    };
    check_signer(maker)?;
    executable(token_program)?;
    check_sysprog(system_program)?;
    check_atoken_gpvbd(atoken_program)?;
    log!("EscrowTokCancel try_from 1");

    writable(escrow_ata_x)?;
    check_ata(escrow_ata_x, escrow_pda, mint_x)?;

    log!("EscrowTokCancel try_from 2");
    writable(escrow_ata_y)?;
    check_ata(escrow_ata_y, escrow_pda, mint_y)?;
    log!("EscrowTokCancel try_from 3");

    writable(escrow_pda)?;
    writable(config_pda)?;
    if escrow_pda.data_is_empty() {
      return Err(Ee::EscrowDataEmpty.into());
    }
    log!("EscrowTokCancel try_from 5");
    rent_exempt_mint(mint_x)?;
    rent_exempt_mint(mint_y)?;
    //TODO: fee is part of exchange amount

    log!("EscrowTokCancel try_from 6");
    check_mint0a(mint_x, token_program)?;
    check_mint0a(mint_y, token_program)?; // Not needed since CPI since deposit will fail if not owned by token program

    Ok(Self {
      maker,
      maker_ata_x,
      maker_ata_y,
      escrow_ata_x,
      escrow_ata_y,
      mint_x,
      mint_y,
      escrow_pda,
      config_pda,
      token_program,
      system_program,
      atoken_program,
    })
  }
}
