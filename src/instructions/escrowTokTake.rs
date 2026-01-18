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
    log!("---------== process()");
    config_pda.can_borrow_mut_data()?;
    let _config: &mut Config = Config::from_account_info(&config_pda)?;

    escrow_pda.can_borrow_mut_data()?;
    let escrow: &mut Escrow = Escrow::from_account_info(&escrow_pda)?;

    log!("Check args against EscrowPDA fields");
    //cannot convert the maker in EscrowPDA from Pubkey to AccountInfo! Also hide the maker
    let bump = escrow.bump();
    let maker = escrow.maker();
    if escrow.mint_x().ne(mint_x.key()) {
      return Ee::EscrowMintX.e();
    }
    if escrow.mint_y().ne(mint_y.key()) {
      return Ee::EscrowMintY.e();
    }
    if escrow.amount_x() != amount_x {
      return Ee::InputAmountX.e();
    }
    if escrow.amount_y() != amount_y {
      return Ee::InputAmountY.e();
    }
    if escrow.id() != id {
      return Ee::EscrowId.e();
    }

    log!("Check Escrow ATA Y");
    if escrow_ata_y.data_is_empty() {
      log!("Make escrow_ata_y");
      pinocchio_associated_token_account::instructions::Create {
        funding_account: taker,
        account: escrow_ata_y,
        wallet: escrow_pda,
        mint: mint_y,
        system_program,
        token_program,
      }
      .invoke()?;
      //Please upgrade to SPL Token 2022 for immutable owner support
    } else {
      log!("escrow_ata_y has data");
      check_ata_escrow(escrow_ata_y, escrow_pda, mint_y)?;
    }
    writable(escrow_ata_y)?;
    rent_exempt_tokacct(escrow_ata_y)?;

    log!("Check Taker ATA X");
    if taker_ata_x.data_is_empty() {
      log!("Make taker_ata_x");
      pinocchio_associated_token_account::instructions::Create {
        funding_account: taker,
        account: taker_ata_x,
        wallet: taker,
        mint: mint_x,
        system_program,
        token_program,
      }
      .invoke()?;
      //Please upgrade to SPL Token 2022 for immutable owner support
    } else {
      log!("taker_ata_x has data");
      check_ata(taker_ata_x, taker, mint_x)?;
    }
    writable(taker_ata_x)?;
    rent_exempt_tokacct(taker_ata_x)?;

    log!("Transfer Token Y to Escrow ATA Y");
    pinocchio_token::instructions::TransferChecked {
      from: taker_ata_y,
      mint: mint_y,
      to: escrow_ata_y,
      authority: taker,
      amount: amount_y,
      decimals: decimal_y,
    }
    .invoke()?;

    log!("Make Seed Signer");
    let id_bytes = &id.to_le_bytes();
    let signer_seeds = [
      Seed::from(Escrow::SEED),
      Seed::from(maker.as_ref()),
      Seed::from(id_bytes),
      Seed::from(core::slice::from_ref(&bump)),
    ];
    let seed_signer = Signer::from(&signer_seeds);

    log!("Transfer Token X to Taker ATA X");
    pinocchio_token::instructions::TransferChecked {
      from: escrow_ata_x,
      mint: mint_x,
      to: taker_ata_x,
      authority: escrow_pda,
      amount: amount_x,
      decimals: decimal_x,
    }
    .invoke_signed(&[seed_signer])?;
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
    log!("EscrowTokTake try_from 4");

    //2x u8 takes 2 + 2x u64 takes 16 bytes
    data_len(data, 26)?;
    let decimal_x = data[0];
    let amount_x = parse_u64(&data[1..9])?;
    log!("decimal_x: {}, amount_x: {}", decimal_x, amount_x);
    none_zero_u64(amount_x)?;

    let escrow_ata_x_info = TokenAccount::from_account_info(escrow_ata_x)?;
    if escrow_ata_x_info.amount() < amount_x {
      return Err(Ee::EscrowInsuffTokenX.into());
    } //ata_balc(escrow_ata_x, amount_x)?;
      //TODO: unknown token received by Escrow

    let decimal_y = data[9];
    let amount_y = parse_u64(&data[10..18])?;
    log!("decimal_y: {}, amount_y: {}", decimal_y, amount_y);
    none_zero_u64(amount_y)?;
    let taker_ata_y_info = TokenAccount::from_account_info(taker_ata_y)?;
    if taker_ata_y_info.amount() < amount_y {
      return Err(Ee::TakerInsuffTokenY.into());
    } //ata_balc(taker_ata_y, amount_y)?;

    let id = parse_u64(&data[18..26])?;
    log!("id: {}", id);

    log!("EscrowTokTake try_from 5");
    check_escrow_mints(mint_x, mint_y)?;
    rent_exempt_mint(mint_x)?;
    rent_exempt_mint(mint_y)?;
    //TODO: fee is part of exchange amount

    log!("EscrowTokTake try_from 6");
    check_decimals(mint_x, decimal_x)?;
    check_decimals(mint_y, decimal_y)?;
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
