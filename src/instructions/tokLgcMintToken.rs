use core::convert::TryFrom;
use pinocchio::{account_info::AccountInfo, program_error::ProgramError, ProgramResult};
use pinocchio_log::log;

use crate::{
  check_ata, check_mint0b, check_sysprog, data_len, executable, instructions::check_signer,
  none_zero_u64, parse_u64, rent_exempt22, writable,
};

/// TokLgc Mint Tokens
pub struct TokLgcMintToken<'a> {
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
impl<'a> TokLgcMintToken<'a> {
  pub const DISCRIMINATOR: &'a u8 = &4;

  pub fn process(self) -> ProgramResult {
    let TokLgcMintToken {
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
    log!("TokLgcMintToken process()");
    rent_exempt22(mint, 0)?;
    writable(mint)?;
    check_mint0b(mint, mint_authority, token_program, decimals)?;

    log!("TokLgcMintToken 2");
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
      check_ata(ata, to_wallet, mint)?;
    }
    writable(ata)?;
    rent_exempt22(ata, 1)?;
    log!("Token Account found/verified");

    log!("Mint Tokens");
    pinocchio_token::instructions::MintToChecked {
      mint,
      account: ata,
      mint_authority,
      amount,
      decimals,
    }
    .invoke()?;
    Ok(())
  }
}
impl<'a> TryFrom<(&'a [u8], &'a [AccountInfo])> for TokLgcMintToken<'a> {
  type Error = ProgramError;

  fn try_from(value: (&'a [u8], &'a [AccountInfo])) -> Result<Self, Self::Error> {
    log!("TokLgcMintToken try_from");
    let (data, accounts) = value;
    log!("accounts len: {}, data len: {}", accounts.len(), data.len());

    let [mint_authority, to_wallet, mint, ata, token_program, system_program, atoken_program] =
      accounts
    else {
      return Err(ProgramError::NotEnoughAccountKeys);
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
/*Transfer mint_x from user ata to vault
      pinocchio_token::instructions::Transfer {
          from: maker_ata,
          to: vault,
          authority: maker,
          amount,
      }.invoke()?;

//----------==
  pinocchio_token::instructions::Transfer {
      from: vault,
      to: taker_ata_x,
      authority: escrow,
      amount: vault_account.amount(),
  }.invoke_signed(&[seeds.clone()])?;

  pinocchio_token::instructions::CloseAccount {
      account: vault,
      destination: maker,
      authority: escrow,
  }.invoke_signed(&[seeds])?;  */
