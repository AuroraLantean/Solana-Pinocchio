use core::convert::TryFrom;
use pinocchio::{account_info::AccountInfo, program_error::ProgramError, ProgramResult};
use pinocchio_log::log;

use crate::{
  ata_balc, check_ata, check_decimals, check_mint0a, check_pda, check_sysprog, data_len,
  executable, instructions::check_signer, none_zero_u64, parse_u64, rent_exempt22, writable,
  Config, Ee,
};

/// TokLgc: Users to Pay Tokens to VaultAdmin
pub struct TokLgcPay<'a> {
  pub user: &'a AccountInfo, //signer
  pub from_ata: &'a AccountInfo,
  //pub config_pda: &'a AccountInfo,
  pub to_ata: &'a AccountInfo,
  pub to_wallet: &'a AccountInfo,
  pub mint: &'a AccountInfo,
  pub token_program: &'a AccountInfo,
  pub system_program: &'a AccountInfo,
  pub atoken_program: &'a AccountInfo,
  pub decimals: u8,
  pub amount: u64,
}
impl<'a> TokLgcPay<'a> {
  pub const DISCRIMINATOR: &'a u8 = &7;

  pub fn process(self) -> ProgramResult {
    let TokLgcPay {
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
    log!("TokLgcPay process()");
    rent_exempt22(mint, 0)?;
    check_decimals(mint, decimals)?;
    check_mint0a(mint, token_program)?;

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
impl<'a> TryFrom<(&'a [u8], &'a [AccountInfo])> for TokLgcPay<'a> {
  type Error = ProgramError;

  fn try_from(value: (&'a [u8], &'a [AccountInfo])) -> Result<Self, Self::Error> {
    log!("TokLgcPay try_from");
    let (data, accounts) = value;
    log!("accounts len: {}, data len: {}", accounts.len(), data.len());

    let [user, from_ata, to_ata, to_wallet, mint, token_program, system_program, atoken_program] =
      accounts
    else {
      return Err(ProgramError::NotEnoughAccountKeys);
    };
    check_signer(user)?;
    check_sysprog(system_program)?;
    executable(token_program)?;
    //check_pda(config_pda)?;
    writable(from_ata)?;
    check_ata(from_ata, user, mint)?;

    //1+8: u8 takes 1, u64 takes 8 bytes
    data_len(data, 9)?;
    let decimals = data[0];
    let amount = parse_u64(&data[1..])?;
    log!("decimals: {}, amount: {}", decimals, amount);

    none_zero_u64(amount)?;
    ata_balc(from_ata, amount)?;

    log!("fetch from ConfigPDA");
    //writable(config_pda)?;
    //TODO:config_pda.can_borrow_data()?;
    //let config: &Config = Config::read(&config_pda)?;

    if to_wallet.lamports() == 0 {
      return Err(Ee::ToWallet.into());
    }
    log!("to_wallet exists");
    check_pda(to_wallet)?;

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
