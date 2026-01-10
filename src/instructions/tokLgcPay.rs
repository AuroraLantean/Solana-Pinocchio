use core::convert::TryFrom;
use pinocchio::{account_info::AccountInfo, program_error::ProgramError, ProgramResult};
use pinocchio_log::log;

use crate::{
  ata_balc, check_ata, check_atoken_gpvbd, check_decimals, check_mint0a, check_sysprog,
  check_vault, data_len, executable, instructions::check_signer, none_zero_u64, parse_u64,
  rent_exempt_mint, rent_exempt_tokacct, writable, Config, Ee,
};

/// TokLgc: Users to Pay Tokens to VaultAdmin
pub struct TokLgcPay<'a> {
  pub user: &'a AccountInfo, //signer
  pub from_ata: &'a AccountInfo,
  //pub config_pda: &'a AccountInfo,
  pub to_ata: &'a AccountInfo,
  pub vault: &'a AccountInfo,
  pub mint: &'a AccountInfo,
  pub config_pda: &'a AccountInfo,
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
      vault,
      mint,
      config_pda: _,
      token_program,
      system_program,
      atoken_program: _,
      decimals,
      amount,
    } = self;
    log!("TokLgcPay process()");

    if to_ata.data_is_empty() {
      log!("Make to_ata");
      pinocchio_associated_token_account::instructions::Create {
        funding_account: user,
        account: to_ata,
        wallet: vault,
        mint,
        system_program,
        token_program,
      }
      .invoke()?;
      //Please upgrade to SPL Token 2022 for immutable owner support
    } else {
      log!("to_ata has data");
      check_ata(to_ata, vault, mint)?;
    }
    writable(to_ata)?;
    rent_exempt_tokacct(to_ata)?;
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
    Ok(())
  }
}
impl<'a> TryFrom<(&'a [u8], &'a [AccountInfo])> for TokLgcPay<'a> {
  type Error = ProgramError;

  fn try_from(value: (&'a [u8], &'a [AccountInfo])) -> Result<Self, Self::Error> {
    log!("TokLgcPay try_from");
    let (data, accounts) = value;
    log!("accounts len: {}, data len: {}", accounts.len(), data.len());

    let [user, from_ata, to_ata, vault, mint, config_pda, token_program, system_program, atoken_program] =
      accounts
    else {
      return Err(ProgramError::NotEnoughAccountKeys);
    };
    check_signer(user)?;
    executable(token_program)?;
    check_sysprog(system_program)?;
    check_atoken_gpvbd(atoken_program)?;

    writable(from_ata)?;
    check_ata(from_ata, user, mint)?;
    log!("TokLgcPay try_from 5");
    //1+8: u8 takes 1, u64 takes 8 bytes
    data_len(data, 9)?;
    let decimals = data[0];
    let amount = parse_u64(&data[1..])?;
    log!("decimals: {}, amount: {}", decimals, amount);

    none_zero_u64(amount)?;
    ata_balc(from_ata, amount)?;

    log!("TokLgcPay try_from 9");
    config_pda.can_borrow_mut_data()?;
    let config: &mut Config = Config::from_account_info(&config_pda)?;

    if !config.mints().contains(&mint.key()) {
      return Err(Ee::MintNotAccepted.into());
    }
    check_vault(vault, config.vault())?;

    rent_exempt_mint(mint)?;
    check_decimals(mint, decimals)?;
    check_mint0a(mint, token_program)?;

    Ok(Self {
      user,
      from_ata,
      to_ata,
      vault,
      mint,
      config_pda,
      token_program,
      system_program,
      atoken_program,
      decimals,
      amount,
    })
  }
}
