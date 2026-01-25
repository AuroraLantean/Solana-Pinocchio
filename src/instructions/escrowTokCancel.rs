use core::convert::TryFrom;
use pinocchio::{
  cpi::{Seed, Signer},
  AccountView, ProgramResult,
};
use pinocchio_log::log;
use pinocchio_token::state::TokenAccount;

use crate::{
  check_ata, check_atoken_gpvbd, check_decimals, check_mint0a, check_sysprog, data_len, executable,
  instructions::check_signer, none_zero_u64, rent_exempt_mint, rent_exempt_tokacct, writable,
  Config, Ee, Escrow,
};
//TODO: add Token2022 interface
/// Make Cancel Escrow
pub struct EscrowTokCancel<'a> {
  pub maker: &'a AccountView, //signer
  pub maker_ata_x: &'a AccountView,
  pub maker_ata_y: &'a AccountView,
  pub escrow_ata_x: &'a AccountView,
  pub escrow_ata_y: &'a AccountView,
  pub mint_x: &'a AccountView,
  pub mint_y: &'a AccountView,
  pub escrow_pda: &'a AccountView,
  pub config_pda: &'a AccountView,
  pub token_program: &'a AccountView,
  pub system_program: &'a AccountView,
  pub atoken_program: &'a AccountView,
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
    let amount_x = escrow.amount_x();
    log!("decimal_x: {}, amount_x: {}", decimal_x, amount_x);
    let amount_y = escrow.amount_y();
    let decimal_y = escrow.decimal_y();
    log!("decimal_y: {}, amount_y: {}", decimal_y, amount_y);
    none_zero_u64(amount_y)?;
    check_decimals(mint_x, decimal_x)?;
    check_decimals(mint_y, decimal_y)?;

    log!("Check if Escrow ATA Y has value");
    if escrow_ata_y.data_len() > 0 {
      let escrow_ata_y_info = TokenAccount::from_account_info(escrow_ata_y)?;
      if escrow_ata_y_info.amount() >= amount_y {
        return Ee::MakerToWithdrawTokenY.e();
      }
      drop(escrow_ata_y_info);
    }

    log!("Check Maker ATA X");
    if maker_ata_x.data_is_empty() {
      log!("Make Maker_Ata_X");
      pinocchio_associated_token_account::instructions::Create {
        funding_account: maker,
        account: maker_ata_x,
        wallet: maker,
        mint: mint_x,
        system_program,
        token_program,
      }
      .invoke()?;
      //Please upgrade to SPL Token 2022 for immutable owner support
    } else {
      log!("Maker_Ata_Y has data");
      check_ata(maker_ata_x, maker, mint_x)?;
    }
    writable(maker_ata_x)?;
    rent_exempt_tokacct(maker_ata_x)?;

    log!("Make Seed Signer");
    let id_bytes = &id.to_le_bytes();
    let signer_seeds = [
      Seed::from(Escrow::SEED),
      Seed::from(maker.key().as_ref()),
      Seed::from(id_bytes),
      Seed::from(core::slice::from_ref(&bump)),
    ];
    let seed_signer = Signer::from(&signer_seeds);

    log!("Transfer Token X to Maker ATA X");
    //escrow_pda.can_borrow_mut_data()?;
    //escrow_ata_y.can_borrow_mut_data()?;
    pinocchio_token::instructions::TransferChecked {
      from: escrow_ata_x,
      mint: mint_x,
      to: maker_ata_x,
      authority: escrow_pda,
      amount: amount_x,
      decimals: decimal_x,
    }
    .invoke_signed(&[seed_signer.clone()])?;

    log!("Check Unknown token in Escrow ATA Y");
    if escrow_ata_y.data_len() > 0 {
      let escrow_ata_y_info = TokenAccount::from_account_info(escrow_ata_y)?;
      let unknown_amt_y = escrow_ata_y_info.amount();
      drop(escrow_ata_y_info);

      if unknown_amt_y > 0 {
        log!("Found unknown token in Escrow ATA Y");
        if maker_ata_y.data_is_empty() {
          log!("Make maker_ata_y");
          pinocchio_associated_token_account::instructions::Create {
            funding_account: maker,
            account: maker_ata_y,
            wallet: maker,
            mint: mint_y,
            system_program,
            token_program,
          }
          .invoke()?;
          //Please upgrade to SPL Token 2022 for immutable owner support
        } else {
          log!("Maker_Ata_Y has data");
          check_ata(maker_ata_y, maker, mint_y)?;
        }
        writable(maker_ata_y)?;
        rent_exempt_tokacct(maker_ata_y)?;

        log!("Send token y to maker_ata_y");
        pinocchio_token::instructions::TransferChecked {
          from: escrow_ata_y,
          mint: mint_y,
          to: maker_ata_y,
          authority: escrow_pda,
          amount: unknown_amt_y,
          decimals: decimal_y,
        }
        .invoke_signed(&[seed_signer.clone()])?;
      } else {
        log!("Escrow_ATA_Y has 0 token");
      }

      log!("Close Escrow ATA Y");
      //escrow_ata_y.can_borrow_mut_data()?;
      //escrow_pda.can_borrow_mut_data()?;
      pinocchio_token::instructions::CloseAccount {
        account: escrow_ata_y,
        authority: escrow_pda,
        destination: maker,
      }
      .invoke_signed(&[seed_signer.clone()])?;
    } else {
      log!("Escrow_ATA_Y has no data");
    }

    if escrow_ata_x.data_len() > 0 {
      log!("Close Escrow ATA X");
      //escrow_pda.can_borrow_mut_data()?;
      //escrow_ata_x.can_borrow_mut_data()?;
      pinocchio_token::instructions::CloseAccount {
        account: escrow_ata_x,
        authority: escrow_pda,
        destination: maker,
      }
      .invoke_signed(&[seed_signer.clone()])?;
    }

    log!("Close EscrowPDA 1");
    //set the first byte to 255
    {
      let mut data = escrow_pda.try_borrow_mut_data()?;
      data[0] = 0xff;
    }
    log!("Close EscrowPDA 2");
    //https://learn.blueshift.gg/en/courses/pinocchio-for-dummies/pinocchio-accounts
    *maker.try_borrow_mut_lamports()? += *escrow_pda.try_borrow_lamports()?;

    log!("Close EscrowPDA 3");
    //resize the account to only the 1st byte
    escrow_pda.resize(1)?;
    escrow_pda.close()?;
    Ok(())
  }
}
impl<'a> TryFrom<(&'a [u8], &'a [AccountView])> for EscrowTokCancel<'a> {
  type Error = ProgramError;

  fn try_from(value: (&'a [u8], &'a [AccountView])) -> Result<Self, Self::Error> {
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
    //check_ata(escrow_ata_y, escrow_pda, mint_y)?; ... escrow_ata_y does not yet exist

    writable(escrow_pda)?;
    writable(config_pda)?;
    if escrow_pda.data_is_empty() {
      return Err(Ee::EscrowDataEmpty.into());
    }
    log!("EscrowTokCancel try_from 5");
    rent_exempt_mint(mint_x)?;
    rent_exempt_mint(mint_y)?;

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
