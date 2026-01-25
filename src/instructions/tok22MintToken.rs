use core::convert::TryFrom;
use pinocchio::{account_info::AccountInfo,  ProgramResult};
use pinocchio_log::log;

use crate::{
  check_ata22, check_mint22b, check_sysprog, data_len, executable, instructions::check_signer,
  none_zero_u64, parse_u64, rent_exempt_mint22, rent_exempt_tokacct22, writable,
};

/// Token2022 Mint Tokens
pub struct Token2022MintToken<'a> {
  pub mint_authority: &'a AccountInfo, //signer
  pub to_wallet: &'a AccountInfo,
  pub mint: &'a AccountInfo,
  pub ata: &'a AccountInfo,
  pub token_program: &'a AccountInfo,
  pub system_program: &'a AccountInfo,
  pub atoken_program: &'a AccountInfo,
  pub decimals: u8,
  pub amount: u64,
}
impl<'a> Token2022MintToken<'a> {
  pub const DISCRIMINATOR: &'a u8 = &11;

  pub fn process(self) -> ProgramResult {
    let Token2022MintToken {
      mint_authority,
      to_wallet,
      mint,
      ata,
      token_program,
      system_program,
      atoken_program: _,
      decimals,
      amount,
    } = self;
    log!("Token2022MintToken process()");
    rent_exempt_mint22(mint)?;
    writable(mint)?;
    check_mint22b(mint, mint_authority, token_program, decimals)?;

    log!("Token2022MintToken 5");
    if ata.data_is_empty() {
      log!("Make ata");
      pinocchio_associated_token_account::instructions::Create {
        funding_account: mint_authority,
        account: ata,
        wallet: to_wallet,
        mint,
        system_program,
        token_program,
      }
      .invoke()?;
      //Please upgrade to SPL Token 2022 for immutable owner support
    } else {
      log!("ata has data");
      check_ata22(ata, to_wallet, mint)?;
    }
    log!("Token2022MintToken 7");
    writable(ata)?;
    rent_exempt_tokacct22(ata)?;
    log!("Token Account found/verified");

    log!("Mint Tokens");
    pinocchio_token_2022::instructions::MintToChecked {
      mint,
      account: ata,
      mint_authority,
      amount,
      decimals,
      token_program: token_program.key(),
    }
    .invoke()?;
    Ok(())
  }
}
impl<'a> TryFrom<(&'a [u8], &'a [AccountInfo])> for Token2022MintToken<'a> {
  type Error = ProgramResult;

  fn try_from(value: (&'a [u8], &'a [AccountInfo])) -> Result<Self, Self::Error> {
    log!("Token2022MintToken try_from");
    let (data, accounts) = value;
    log!("accounts len: {}, data len: {}", accounts.len(), data.len());

    let [mint_authority, to_wallet, mint, ata, token_program, system_program, atoken_program] =
      accounts
    else {
      return Err(ProgramResult::NotEnoughAccountKeys);
    };
    check_signer(mint_authority)?;
    executable(token_program)?;
    check_sysprog(system_program)?;
    //check_pda(config_pda)?;

    //1+8: u8 takes 1, u64 takes 8 bytes
    data_len(data, 9)?;
    let decimals = data[0];
    let amount = parse_u64(&data[1..])?;
    log!("decimals: {}, amount: {}", decimals, amount);
    none_zero_u64(amount)?;
    Ok(Self {
      mint_authority,
      to_wallet,
      mint,
      ata,
      token_program,
      system_program,
      atoken_program,
      decimals,
      amount,
    })
  }
}
