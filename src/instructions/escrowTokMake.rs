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
  ata_balc, check_ata, check_ata_escrow, check_atoken_gpvbd, check_decimals, check_mint0a,
  check_mint_xy, check_sysprog, data_len, executable, instructions::check_signer, none_zero_u64,
  parse_u64, rent_exempt_mint, rent_exempt_tokacct, writable, Config, Ee, Escrow,
};

/// Make Escrow Token Offer
pub struct EscrowTokMake<'a> {
  pub maker_x: &'a AccountInfo, //signer
  pub user_x_ata: &'a AccountInfo,
  pub escrow_ata: &'a AccountInfo, //as to_ata
  pub escrow: &'a AccountInfo,     //PDA as to_wallet
  pub mint_x: &'a AccountInfo,
  pub mint_y: &'a AccountInfo,
  pub token_program: &'a AccountInfo,
  pub system_program: &'a AccountInfo,
  pub atoken_program: &'a AccountInfo,
  pub decimal_x: u8,
  pub decimal_y: u8,
  pub amount_x: u64,
  pub amount_y: u64,
  pub id: u64,
}
impl<'a> EscrowTokMake<'a> {
  pub const DISCRIMINATOR: &'a u8 = &15;

  pub fn process(self) -> ProgramResult {
    let EscrowTokMake {
      maker_x,
      user_x_ata,
      escrow_ata,
      escrow,
      mint_x,
      mint_y,
      token_program,
      system_program,
      atoken_program: _,
      decimal_x,
      decimal_y: _,
      amount_x,
      amount_y,
      id,
    } = self;
    log!("EscrowTokMake process()");

    /*let bump = unsafe { *(data.as_ptr() as *const u8) }.to_le_bytes();
    if bump.len() != 1 { return Err(..);  };   bump.as_ref()*/
    let seed = [Escrow::SEED, maker_x.key().as_slice(), &id.to_le_bytes()];
    let seeds = &seed[..];

    let (expected_escrow, bump) = find_program_address(seeds, &crate::ID); //TODO: may incur unknown cost
    if expected_escrow.ne(escrow.key()) {
      return Ee::EscrowPDA.e();
    }
    //let expected_escrow = checked_create_program_address(seeds, &crate::ID)?;
    log!("EscrowTokMake EscrowPDA verified");

    if escrow.lamports() > 0 {
      //escrow.owner() != &crate::ID
      return Err(ProgramError::AccountAlreadyInitialized);
    } else {
      log!("Make Escrow PDA 1");
      let lamports = Rent::get()?.minimum_balance(Escrow::LEN);

      log!("Make Escrow PDA 2");
      let id_seed = &id.to_le_bytes();
      //let seed = [Escrow::SEED, maker_x.key().as_slice(), &id.to_le_bytes()];
      //let seeds = &seed[..];
      let seeds: [Seed<'_>; 4] = [
        Seed::from(Escrow::SEED),
        Seed::from(maker_x.key().as_ref()),
        Seed::from(id_seed),
        Seed::from(core::slice::from_ref(&bump)),
      ];
      let seed_signer = [Signer::from(&seeds)];
      log!("Make Escrow PDA 3");

      pinocchio_system::instructions::CreateAccount {
        from: maker_x,
        to: escrow,
        lamports,
        space: Escrow::LEN as u64,
        owner: &crate::ID,
      }
      .invoke_signed(&seed_signer)?;
    }

    log!("Escrow is made");
    if escrow_ata.data_is_empty() {
      log!("Make escrow_ata");
      pinocchio_associated_token_account::instructions::Create {
        funding_account: maker_x,
        account: escrow_ata,
        wallet: escrow,
        mint: mint_x,
        system_program,
        token_program,
      }
      .invoke()?;
      //Please upgrade to SPL Token 2022 for immutable owner support
    } else {
      log!("escrow_ata has data");
      check_ata_escrow(escrow_ata, escrow, mint_x)?;
    }
    writable(escrow_ata)?;
    rent_exempt_tokacct(escrow_ata)?;
    log!("Vault ATA is found/verified");

    log!("Transfer token_x from user_x_ata");
    pinocchio_token::instructions::TransferChecked {
      from: user_x_ata,
      mint: mint_x,
      to: escrow_ata,
      authority: maker_x,
      amount: amount_x, // *(data.as_ptr().add(1 + 8) as *const u64)
      decimals: decimal_x,
    }
    .invoke()?;
    /*  pinocchio_token::instructions::Transfer {
        from: escrow,
        to: escrow_ata,
        authority: escrow,
        amount: vault_account.amount(),
    }.invoke_signed(&[seeds.clone()])?; */

    log!("Fill Escrow PDA AFTER payment");
    let escrow: &mut Escrow = Escrow::from_account_info(&escrow)?;
    escrow.set_maker_x(maker_x.key());
    escrow.set_mint_x(mint_x.key());
    escrow.set_mint_y(mint_y.key());
    escrow.set_id(id)?;
    escrow.set_amount_y(amount_y)?; // unsafe { *(data.as_ptr().add(1) as *const u64) };
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

    let [maker_x, user_x_ata, escrow_ata, escrow, mint_x, mint_y, config_pda, token_program, system_program, atoken_program] =
      accounts
    else {
      return Err(ProgramError::NotEnoughAccountKeys);
    };
    check_signer(maker_x)?;
    executable(token_program)?;
    check_sysprog(system_program)?;
    check_atoken_gpvbd(atoken_program)?;

    writable(user_x_ata)?;
    check_ata(user_x_ata, maker_x, mint_x)?;
    writable(escrow)?;
    log!("EscrowTokMake try_from 5");

    //2x u8 takes 2 + 2x u64 takes 16 bytes
    data_len(data, 26)?;
    let decimal_x = data[0];
    let amount_x = parse_u64(&data[1..9])?;
    log!("decimal_x: {}, amount_x: {}", decimal_x, amount_x);
    none_zero_u64(amount_x)?;
    ata_balc(user_x_ata, amount_x)?;

    let decimal_y = data[9];
    let amount_y = parse_u64(&data[10..18])?;
    log!("decimal_y: {}, amount_y: {}", decimal_y, amount_y);
    none_zero_u64(amount_y)?;

    let id = parse_u64(&data[18..26])?;
    log!("id: {}", id);

    log!("EscrowTokMake try_from: config");
    config_pda.can_borrow_mut_data()?;
    let _config: &mut Config = Config::from_account_info(&config_pda)?;

    log!("EscrowTokMake try_from 5");
    check_mint_xy(mint_x, mint_y)?;
    rent_exempt_mint(mint_x)?;
    rent_exempt_mint(mint_y)?;
    //TODO: fee is part of exchange amount

    log!("EscrowTokMake try_from 6");
    check_decimals(mint_x, decimal_x)?;
    check_mint0a(mint_x, token_program)?;
    check_mint0a(mint_y, token_program)?; // Not needed since CPI since deposit will fail if not owned by token program

    Ok(Self {
      maker_x,
      user_x_ata,
      escrow_ata,
      escrow,
      mint_x,
      mint_y,
      token_program,
      system_program,
      atoken_program,
      decimal_x,
      decimal_y,
      amount_x,
      amount_y,
      id,
    })
  }
}
