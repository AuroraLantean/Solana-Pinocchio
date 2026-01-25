use core::convert::TryFrom;
use pinocchio::{
  account_info::AccountInfo,
  instruction::{Seed, Signer},
  Address, ProgramResult,
};
use pinocchio_log::log;

use crate::{
  ata_balc, check_ata, check_decimals, check_mint0a, check_sysprog, check_vault, data_len,
  executable, instructions::check_signer, none_zero_u64, parse_u64, rent_exempt_mint,
  rent_exempt_tokacct, writable, Config, Ee, VAULT_SEED,
};

/// TokLgc: Users to Redeem Tokens from VaultPDA
pub struct TokLgcRedeem<'a> {
  pub user: &'a AccountInfo, //signer
  pub from_ata: &'a AccountInfo,
  pub to_ata: &'a AccountInfo,
  pub vault: &'a AccountInfo,
  pub prog_owner: &'a Address,
  pub mint: &'a AccountInfo,
  pub token_program: &'a AccountInfo,
  pub system_program: &'a AccountInfo,
  pub atoken_program: &'a AccountInfo,
  pub vault_bump: u8,
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
      vault,
      prog_owner,
      mint,
      token_program,
      system_program,
      atoken_program: _,
      vault_bump,
      decimals,
      amount,
    } = self;
    log!("TokLgcRedeem process()");

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
    log!("ToATA is found/verified");

    let signer_seeds = [
      Seed::from(VAULT_SEED),
      Seed::from(prog_owner.as_ref()),
      Seed::from(core::slice::from_ref(&vault_bump)),
    ];
    log!("TokLgcRedeem 7c");
    let seed_signer = Signer::from(&signer_seeds);

    log!("TokLgcRedeem 8 Transfer Tokens");
    pinocchio_token::instructions::TransferChecked {
      from: from_ata,
      mint,
      to: to_ata,
      authority: vault,
      amount,
      decimals,
    }
    .invoke_signed(&[seed_signer])?;
    Ok(())
  }
}
impl<'a> TryFrom<(&'a [u8], &'a [AccountInfo])> for TokLgcRedeem<'a> {
  type Error = ProgramResult;

  fn try_from(value: (&'a [u8], &'a [AccountInfo])) -> Result<Self, Self::Error> {
    log!("TokLgcRedeem try_from");
    let (data, accounts) = value;
    log!("accounts len: {}, data len: {}", accounts.len(), data.len());

    let [user, from_ata, to_ata, vault, config_pda, mint, token_program, system_program, atoken_program] =
      accounts
    else {
      return Err(ProgramResult::NotEnoughAccountKeys);
    };
    check_signer(user)?;
    executable(token_program)?;
    check_sysprog(system_program)?;

    writable(from_ata)?;
    check_ata(from_ata, vault, mint)?;

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
    /*let (expected_vault, vault_bump) = derive_pda1(config.prog_owner(), VAULT_SEED)?;
    log!("TokLgcPay try_from 9");
    if vault.key() != &expected_vault {
      return Err(Ee::VaultPDA.into());
    }*/

    log!("TokLgcRedeem try_from 12");
    rent_exempt_mint(mint)?;
    check_decimals(mint, decimals)?;
    check_mint0a(mint, token_program)?;

    Ok(Self {
      user,
      from_ata,
      to_ata,
      vault,
      prog_owner: config.prog_owner(),
      mint,
      token_program,
      system_program,
      atoken_program,
      vault_bump: config.vault_bump(),
      decimals,
      amount,
    })
  }
}
