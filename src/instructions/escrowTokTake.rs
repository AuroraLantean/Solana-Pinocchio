use core::convert::TryFrom;
use pinocchio::{
  account_info::AccountInfo,
  instruction::{Seed, Signer},
  program_error::ProgramError,
  ProgramResult,
};
use pinocchio_log::log;

use crate::{
  ata_balc, check_ata, check_ata_escrow, check_atoken_gpvbd, check_decimals, check_escrow_mints,
  check_mint0a, check_sysprog, data_len, executable, instructions::check_signer, none_zero_u64,
  parse_u64, rent_exempt_mint, rent_exempt_tokacct, writable, Config, Ee, Escrow,
};
//TODO: add Token2022 interface
/// Take Escrow Token Offer
pub struct EscrowTokTake<'a> {
  pub taker: &'a AccountInfo, //signer
  pub taker_ata_x: &'a AccountInfo,
  pub taker_ata_y: &'a AccountInfo,
  pub escrow_ata_x: &'a AccountInfo,
  pub escrow_ata_y: &'a AccountInfo,
  pub mint_x: &'a AccountInfo,
  pub mint_y: &'a AccountInfo,
  pub escrow_pda: &'a AccountInfo,
  pub config_pda: &'a AccountInfo,
  pub token_program: &'a AccountInfo,
  pub system_program: &'a AccountInfo,
  pub atoken_program: &'a AccountInfo,
  pub amount_x: u64,
  pub amount_y: u64,
  pub id: u64,
  pub decimal_x: u8,
  pub decimal_y: u8,
}
impl<'a> EscrowTokTake<'a> {
  pub const DISCRIMINATOR: &'a u8 = &16;

  pub fn process(self) -> ProgramResult {
    let EscrowTokTake {
      taker,
      taker_ata_x,
      taker_ata_y,
      escrow_ata_x,
      escrow_ata_y,
      mint_x,
      mint_y,
      escrow_pda,
      config_pda,
      token_program,
      system_program,
      atoken_program: _,
      amount_x,
      amount_y,
      id,
      decimal_x,
      decimal_y,
    } = self;
    log!("EscrowTokTake process()");

    Ok(())
  }
}
impl<'a> TryFrom<(&'a [u8], &'a [AccountInfo])> for EscrowTokTake<'a> {
  type Error = ProgramError;

  fn try_from(value: (&'a [u8], &'a [AccountInfo])) -> Result<Self, Self::Error> {
    log!("EscrowTokTake try_from");
    let (data, accounts) = value;
    log!("accounts len: {}, data len: {}", accounts.len(), data.len());

    let [taker, taker_ata_x, taker_ata_y, escrow_ata_x, escrow_ata_y, mint_x, mint_y, escrow_pda, config_pda, token_program, system_program, atoken_program] =
      accounts
    else {
      return Err(ProgramError::NotEnoughAccountKeys);
    };
    check_signer(taker)?;
    executable(token_program)?;
    check_sysprog(system_program)?;
    check_atoken_gpvbd(atoken_program)?;
    log!("EscrowTokTake try_from 1");

    writable(taker_ata_y)?;
    check_ata(taker_ata_y, taker, mint_y)?;
    log!("EscrowTokTake try_from 2");

    writable(escrow_ata_x)?;
    check_ata(escrow_ata_x, escrow_pda, mint_x)?;
    log!("EscrowTokTake try_from 3");

    writable(escrow_pda)?;
    writable(config_pda)?;
    if escrow_pda.data_is_empty() {
      return Err(Ee::EscrowDataEmpty.into());
    }
    log!("EscrowTokTake try_from 5");

    //2x u8 takes 2 + 2x u64 takes 16 bytes
    data_len(data, 26)?;
    let decimal_x = data[0];
    let amount_x = parse_u64(&data[1..9])?;
    log!("decimal_x: {}, amount_x: {}", decimal_x, amount_x);
    none_zero_u64(amount_x)?;
    ata_balc(escrow_ata_x, amount_x)?;

    let decimal_y = data[9];
    let amount_y = parse_u64(&data[10..18])?;
    log!("decimal_y: {}, amount_y: {}", decimal_y, amount_y);
    none_zero_u64(amount_y)?;
    ata_balc(taker_ata_y, amount_y)?;

    let id = parse_u64(&data[18..26])?;
    log!("id: {}", id);

    log!("EscrowTokTake try_from 5");
    check_escrow_mints(mint_x, mint_y)?;
    rent_exempt_mint(mint_x)?;
    rent_exempt_mint(mint_y)?;
    //TODO: fee is part of exchange amount

    log!("EscrowTokTake try_from 6");
    check_decimals(mint_x, decimal_x)?;
    check_mint0a(mint_x, token_program)?;
    check_mint0a(mint_y, token_program)?; // Not needed since CPI since deposit will fail if not owned by token program

    Ok(Self {
      taker,
      taker_ata_x,
      taker_ata_y,
      escrow_ata_x,
      escrow_ata_y,
      mint_x,
      mint_y,
      escrow_pda,
      config_pda,
      token_program,
      system_program,
      atoken_program,
      amount_x,
      amount_y,
      id,
      decimal_x,
      decimal_y,
    })
  }
}
