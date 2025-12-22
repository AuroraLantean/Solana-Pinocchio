use core::convert::TryFrom;
use pinocchio::{
  account_info::AccountInfo,
  instruction::{Seed, Signer},
  program_error::ProgramError,
  sysvars::{rent::Rent, Sysvar},
  ProgramResult,
};
use pinocchio_log::log;

use crate::{
  check_pda, derive_pda1, instructions::check_signer, parse_u64, writable, MyError, VAULT_SEED,
};

/// Init Config PDA
pub struct InitConfig<'a> {
  pub authority: &'a AccountInfo,
  pub config_pda: &'a AccountInfo,
  pub system_program: &'a AccountInfo,
  //pub seeds: &'a [Seed<'a>],
  pub space: u64,
}
impl<'a> InitConfig<'a> {
  pub const DISCRIMINATOR: &'a u8 = &11;

  pub fn process(self) -> ProgramResult {
    let InitConfig {
      authority,
      config_pda,
      system_program: _,
      //seeds,
      space,
    } = self;
    log!("InitConfig process()");
    check_signer(authority)?;
    //writable(config_pda)?;

    log!("InitConfig 2");
    check_pda(config_pda)?;
    log!("InitConfig 3");

    log!("InitConfig 4");

    log!("InitConfig 5");
    let lamports = Rent::get()?.minimum_balance(space.try_into().unwrap());

    let (_expected_vault_pda, bump) = derive_pda1(authority, VAULT_SEED)?;
    let seeds = [
      Seed::from(VAULT_SEED),
      Seed::from(authority.key().as_ref()),
      Seed::from(core::slice::from_ref(&bump)),
    ];
    let signer = [Signer::from(&seeds)];

    pinocchio_system::instructions::CreateAccount {
      from: authority,
      to: config_pda,
      lamports,
      space: space as u64,
      owner: &crate::ID,
    }
    .invoke_signed(&signer)?;
    Ok(())
  }
}
impl<'a> TryFrom<(&'a [u8], &'a [AccountInfo])> for InitConfig<'a> {
  type Error = ProgramError;

  fn try_from(value: (&'a [u8], &'a [AccountInfo])) -> Result<Self, Self::Error> {
    log!("InitConfig try_from");
    let (data, accounts) = value;
    log!("accounts len: {}, data len: {}", accounts.len(), data.len());

    let [authority, config_pda, system_program] = accounts else {
      return Err(ProgramError::NotEnoughAccountKeys);
    };
    //let seeds: &'a [Seed<'a>] = &'a [Seed::from(b"vault".as_slice())];
    let space = parse_u64(data)?;
    Ok(Self {
      authority,
      config_pda,
      system_program,
      //seeds,
      space,
    })
  }
}
