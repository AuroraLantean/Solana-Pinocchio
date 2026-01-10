use core::convert::TryFrom;
use pinocchio::{
  account_info::AccountInfo,
  instruction::{Seed, Signer},
  program_error::ProgramError,
  ProgramResult,
};
use pinocchio_log::log;

use crate::{
  ata_balc, check_ata, check_decimals, check_mint0a, check_sysprog, data_len, derive_pda1,
  executable, instructions::check_signer, none_zero_u64, parse_u64, rent_exempt_mint,
  rent_exempt_tokacct, writable, Ee, VAULT_SEED,
};

/// TokLgc: Users to Redeem Tokens from VaultPDA
pub struct TokLgcRedeem<'a> {
  pub user: &'a AccountInfo, //signer
  pub from_ata: &'a AccountInfo,
  pub to_ata: &'a AccountInfo,
  pub from_pda: &'a AccountInfo,
  pub from_pda_owner: &'a AccountInfo,
  pub mint: &'a AccountInfo,
  pub token_program: &'a AccountInfo,
  pub system_program: &'a AccountInfo,
  pub atoken_program: &'a AccountInfo,
  pub decimals: u8,
  pub amount: u64,
}
impl<'a> TokLgcRedeem<'a> {
  pub const DISCRIMINATOR: &'a u8 = &8;

  pub fn process(self) -> ProgramResult {
    let TokLgcRedeem {
      user,
      from_ata,
      to_ata,
      from_pda,
      from_pda_owner,
      mint,
      token_program,
      system_program,
      atoken_program: _,
      decimals,
      amount,
    } = self;
    log!("TokLgcRedeem process()");
    rent_exempt_mint(mint)?;
    check_decimals(mint, decimals)?;
    check_mint0a(mint, token_program)?;

    log!("TokLgcRedeem 2");
    if to_ata.data_is_empty() {
      log!("Make to_ata");
      pinocchio_associated_token_account::instructions::Create {
        funding_account: user,
        account: to_ata,
        wallet: user,
        mint,
        system_program,
        token_program,
      }
      .invoke()?;
      //Please upgrade to SPL Token 2022 for immutable owner support
    } else {
      log!("to_ata has data");
      check_ata(to_ata, user, mint)?;
    }
    writable(to_ata)?;
    rent_exempt_tokacct(to_ata)?;
    log!("TokLgcRedeem 6 ToATA is found/verified");

    let (expected_vault_pda, bump) = derive_pda1(from_pda_owner, VAULT_SEED)?;
    log!("TokLgcRedeem 7a");
    if from_pda.key() != &expected_vault_pda {
      return Ee::VaultPDA.e();
    }
    log!("TokLgcRedeem 7b");
    let signer_seeds = [
      Seed::from(VAULT_SEED),
      Seed::from(from_pda_owner.key().as_ref()),
      Seed::from(core::slice::from_ref(&bump)),
    ];
    log!("TokLgcRedeem 7c");
    let signer = Signer::from(&signer_seeds);

    log!("TokLgcRedeem 8 Transfer Tokens");
    pinocchio_token::instructions::TransferChecked {
      from: from_ata,
      mint,
      to: to_ata,
      authority: from_pda,
      amount,
      decimals,
    }
    .invoke_signed(&[signer])?;
    /*  pinocchio_token::instructions::Transfer {
        from: vault,
        to: to_ata,
        authority: escrow,
        amount: vault_account.amount(),
    }.invoke_signed(&[seeds.clone()])?; */
    Ok(())
  }
}
impl<'a> TryFrom<(&'a [u8], &'a [AccountInfo])> for TokLgcRedeem<'a> {
  type Error = ProgramError;

  fn try_from(value: (&'a [u8], &'a [AccountInfo])) -> Result<Self, Self::Error> {
    log!("TokLgcRedeem try_from");
    let (data, accounts) = value;
    log!("accounts len: {}, data len: {}", accounts.len(), data.len());

    let [user, from_ata, to_ata, from_pda, from_pda_owner, mint, token_program, system_program, atoken_program] =
      accounts
    else {
      return Err(ProgramError::NotEnoughAccountKeys);
    };
    check_signer(user)?;
    executable(token_program)?;
    check_sysprog(system_program)?;
    //check_pda(config_pda)?;
    writable(from_ata)?;
    check_ata(from_ata, from_pda, mint)?;

    //1+8: u8 takes 1, u64 takes 8 bytes
    data_len(data, 9)?;
    let decimals = data[0];
    let amount = parse_u64(&data[1..])?;
    log!("decimals: {}, amount: {}", decimals, amount);

    none_zero_u64(amount)?;
    ata_balc(from_ata, amount)?;

    Ok(Self {
      user,
      from_ata,
      to_ata,
      from_pda,
      from_pda_owner,
      mint,
      token_program,
      system_program,
      atoken_program,
      decimals,
      amount,
    })
  }
}
