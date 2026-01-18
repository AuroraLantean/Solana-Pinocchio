use core::convert::TryFrom;
use pinocchio::{
  account_info::AccountInfo,
  instruction::{Seed, Signer},
  program_error::ProgramError,
  pubkey::find_program_address,
  sysvars::{rent::Rent, Sysvar},
  ProgramResult,
};
use pinocchio_log::log;

use crate::{
  ata_balc, check_ata, check_ata_escrow, check_atoken_gpvbd, check_decimals, check_escrow_mints,
  check_mint0a, check_sysprog, data_len, executable, instructions::check_signer, none_zero_u64,
  parse_u64, rent_exempt_mint, rent_exempt_tokacct, writable, Config, Ee, Escrow,
};

/// Make Escrow Token Offer
pub struct EscrowTokMake<'a> {
  pub maker: &'a AccountInfo, //signer
  pub maker_ata_x: &'a AccountInfo,
  pub escrow_ata_x: &'a AccountInfo, //as to_ata
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
impl<'a> EscrowTokMake<'a> {
  pub const DISCRIMINATOR: &'a u8 = &15;

  pub fn process(self) -> ProgramResult {
    let EscrowTokMake {
      maker,
      maker_ata_x,
      escrow_ata_x,
      mint_x,
      mint_y,
      escrow_pda,
      config_pda,
      token_program,
      system_program,
      atoken_program: _,
      decimal_x,
      decimal_y,
      amount_x,
      amount_y,
      id,
    } = self;
    log!("---------== process()");
    config_pda.can_borrow_mut_data()?;
    let _config: &mut Config = Config::from_account_info(&config_pda)?;

    /*let bump = unsafe { *(data.as_ptr() as *const u8) }.to_le_bytes();
    if bump.len() != 1 { return Err(..);  };   bump.as_ref()*/
    let seed = [Escrow::SEED, maker.key().as_slice(), &id.to_le_bytes()];
    let seeds = &seed[..];

    let (expected_escrow, bump) = find_program_address(seeds, &crate::ID); //TODO: may incur unknown cost
    if expected_escrow.ne(escrow_pda.key()) {
      return Ee::EscrowPDA.e();
    }
    //let expected_escrow = checked_create_program_address(seeds, &crate::ID)?;
    log!("EscrowTokMake EscrowPDA verified");

    if escrow_pda.data_is_empty() {
      log!("Make Escrow PDA 1");
      let lamports = Rent::get()?.minimum_balance(Escrow::LEN);

      log!("Make Escrow PDA 2");
      let id_bytes = &id.to_le_bytes();
      //let seed = [Escrow::SEED, maker.key().as_slice(), &id.to_le_bytes()];
      //let seeds = &seed[..];
      let seeds = [
        Seed::from(Escrow::SEED),
        Seed::from(maker.key().as_ref()),
        Seed::from(id_bytes),
        Seed::from(core::slice::from_ref(&bump)),
      ];

      let seed_signer = Signer::from(&seeds);

      pinocchio_system::instructions::CreateAccount {
        from: maker,
        to: escrow_pda,
        lamports,
        space: Escrow::LEN as u64,
        owner: &crate::ID,
      }
      .invoke_signed(&[seed_signer])?;
    } else {
      return Ee::EscrowExists.e();
    }
    log!("Escrow is made");

    if escrow_ata_x.data_is_empty() {
      log!("Make escrow_ata_x");
      pinocchio_associated_token_account::instructions::Create {
        funding_account: maker,
        account: escrow_ata_x,
        wallet: escrow_pda,
        mint: mint_x,
        system_program,
        token_program,
      }
      .invoke()?;
      //Please upgrade to SPL Token 2022 for immutable owner support
    } else {
      log!("escrow_ata_x has data");
      check_ata_escrow(escrow_ata_x, escrow_pda, mint_x)?;
    }
    writable(escrow_ata_x)?;
    rent_exempt_tokacct(escrow_ata_x)?;
    log!("Vault ATA is found/verified");

    pinocchio_token::instructions::TransferChecked {
      from: maker_ata_x,
      mint: mint_x,
      to: escrow_ata_x,
      authority: maker,
      amount: amount_x, // *(data.as_ptr().add(1 + 8) as *const u64)
      decimals: decimal_x,
    }
    .invoke()?;
    log!("tokens sent from maker_ata_x");

    let escrow: &mut Escrow = Escrow::from_account_info(&escrow_pda)?;
    escrow.set_maker(maker.key());
    escrow.set_mint_x(mint_x.key());
    escrow.set_mint_y(mint_y.key());
    escrow.set_id(id)?;
    escrow.set_amount_x(amount_x)?;
    escrow.set_amount_y(amount_y)?; // unsafe { *(data.as_ptr().add(1) as *const u64) };
    escrow.set_decimal_x(decimal_x);
    escrow.set_decimal_y(decimal_y);
    escrow.set_bump(bump); // unsafe { *data.as_ptr() };

    Ok(())
  }
}
impl<'a> TryFrom<(&'a [u8], &'a [AccountInfo])> for EscrowTokMake<'a> {
  type Error = ProgramError;

  fn try_from(value: (&'a [u8], &'a [AccountInfo])) -> Result<Self, Self::Error> {
    log!("EscrowTokMake try_from");
    let (data, accounts) = value;
    log!("accounts len: {}, data len: {}", accounts.len(), data.len());

    let [maker, maker_ata_x, escrow_ata_x, mint_x, mint_y, escrow_pda, config_pda, token_program, system_program, atoken_program] =
      accounts
    else {
      return Err(ProgramError::NotEnoughAccountKeys);
    };
    check_signer(maker)?;
    executable(token_program)?;
    check_sysprog(system_program)?;
    check_atoken_gpvbd(atoken_program)?;
    log!("EscrowTokMake try_from 3");

    writable(maker_ata_x)?;
    check_ata(maker_ata_x, maker, mint_x)?;
    writable(escrow_pda)?;
    writable(config_pda)?;
    log!("EscrowTokMake try_from 4");

    //2x u8 takes 2 + 2x u64 takes 16 bytes
    data_len(data, 26)?;
    let decimal_x = data[0];
    let amount_x = parse_u64(&data[1..9])?;
    log!("decimal_x: {}, amount_x: {}", decimal_x, amount_x);
    none_zero_u64(amount_x)?;
    ata_balc(maker_ata_x, amount_x)?;

    let decimal_y = data[9];
    let amount_y = parse_u64(&data[10..18])?;
    log!("decimal_y: {}, amount_y: {}", decimal_y, amount_y);
    none_zero_u64(amount_y)?;

    let id = parse_u64(&data[18..26])?;
    log!("id: {}", id);

    log!("EscrowTokMake try_from 5");
    check_escrow_mints(mint_x, mint_y)?;
    rent_exempt_mint(mint_x)?;
    rent_exempt_mint(mint_y)?;
    //TODO: fee is part of exchange amount

    log!("EscrowTokMake try_from 6");
    check_decimals(mint_x, decimal_x)?;
    check_decimals(mint_y, decimal_y)?;
    check_mint0a(mint_x, token_program)?;
    check_mint0a(mint_y, token_program)?; // Not needed since CPI since deposit will fail if not owned by token program

    Ok(Self {
      maker,
      maker_ata_x,
      escrow_ata_x,
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
