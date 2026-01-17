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
  check_ata, check_ata_escrow, check_atoken_gpvbd, check_decimals, check_escrow_mints,
  check_mint0a, check_sysprog, data_len, executable, instructions::check_signer, none_zero_u64,
  parse_u64, rent_exempt_mint, rent_exempt_tokacct, writable, Config, Ee, Escrow,
};
//TODO: add Token2022 interface
/// Make Withdraw Escrow Token Y
pub struct EscrowTokWithdraw<'a> {
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
  pub id: u64,
}
impl<'a> EscrowTokWithdraw<'a> {
  pub const DISCRIMINATOR: &'a u8 = &17;

  pub fn process(self) -> ProgramResult {
    let EscrowTokWithdraw {
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
      id,
    } = self;
    log!("---------== process()");
    config_pda.can_borrow_mut_data()?;
    let _config: &mut Config = Config::from_account_info(&config_pda)?;

    escrow_pda.can_borrow_mut_data()?;
    let escrow: &mut Escrow = Escrow::from_account_info(&escrow_pda)?;

    log!("Check args against EscrowPDA fields");
    let bump = escrow.bump();
    if maker.key().ne(escrow.maker()) {
      return Err(ProgramError::IncorrectAuthority);
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

    Ok(())
  }
}
impl<'a> TryFrom<(&'a [u8], &'a [AccountInfo])> for EscrowTokWithdraw<'a> {
  type Error = ProgramError;

  fn try_from(value: (&'a [u8], &'a [AccountInfo])) -> Result<Self, Self::Error> {
    log!("EscrowTokWithdraw try_from");
    let (data, accounts) = value;
    log!("accounts len: {}, data len: {}", accounts.len(), data.len());

    let [maker, maker_ata_x, maker_ata_y, escrow_ata_x, escrow_ata_y, mint_x, mint_y, escrow_pda, config_pda, token_program, system_program, atoken_program] =
      accounts
    else {
      return Err(ProgramError::NotEnoughAccountKeys);
    };
    check_signer(maker)?;
    executable(token_program)?;
    check_sysprog(system_program)?;
    check_atoken_gpvbd(atoken_program)?;
    log!("EscrowTokWithdraw try_from 1");

    writable(maker_ata_y)?;
    check_ata(maker_ata_y, maker, mint_y)?;
    log!("EscrowTokWithdraw try_from 2");

    writable(escrow_ata_x)?;
    check_ata(escrow_ata_x, escrow_pda, mint_x)?;
    log!("EscrowTokWithdraw try_from 3");

    writable(escrow_pda)?;
    writable(config_pda)?;
    if escrow_pda.data_is_empty() {
      return Err(Ee::EscrowDataEmpty.into());
    }
    log!("EscrowTokWithdraw try_from 5");

    //2x u8 takes 2 + 2x u64 takes 16 bytes
    data_len(data, 8)?;
    let id = parse_u64(&data[0..8])?;
    log!("id: {}", id);

    log!("EscrowTokWithdraw try_from 5");
    //check_escrow_mints(mint_x, mint_y)?;
    rent_exempt_mint(mint_x)?;
    rent_exempt_mint(mint_y)?;
    //TODO: fee is part of exchange amount

    log!("EscrowTokWithdraw try_from 6");
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
      id,
    })
  }
}
