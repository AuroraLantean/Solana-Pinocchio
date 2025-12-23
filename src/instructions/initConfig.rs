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
  derive_pda1, empty_lamport, instructions::check_signer, parse_u64, Config, MyError, CONFIG_SEED,
};

/// Init Config PDA
pub struct InitConfig<'a> {
  pub authority: &'a AccountInfo,
  pub config_pda: &'a AccountInfo,
  pub original_owner: &'a AccountInfo,
  pub system_program: &'a AccountInfo,
  pub fee: u64,
}
impl<'a> InitConfig<'a> {
  pub const DISCRIMINATOR: &'a u8 = &12;

  pub fn process(self) -> ProgramResult {
    let InitConfig {
      authority,
      config_pda,
      original_owner,
      fee,
      system_program: _,
    } = self;
    log!("InitConfig process()");
    check_signer(authority)?;
    //writable(config_pda)?;

    log!("InitConfig 2");
    empty_lamport(config_pda)?;

    log!("InitConfig 3");
    let lamports = Rent::get()?.minimum_balance(Config::LEN); //space.try_into().unwrap()
    let space = Config::LEN as u64;

    log!("InitConfig 4");
    let (expected_config_pda, bump) = derive_pda1(original_owner, CONFIG_SEED)?;

    log!("InitConfig 5");
    if expected_config_pda != *config_pda.key() {
      return Err(MyError::ConfigPDA.into());
    }

    log!("InitConfig 6");
    let seeds = [
      Seed::from(CONFIG_SEED),
      Seed::from(original_owner.key().as_ref()),
      Seed::from(core::slice::from_ref(&bump)),
    ];
    let signer = [Signer::from(&seeds)];

    log!("InitConfig 7");
    pinocchio_system::instructions::CreateAccount {
      from: authority,
      to: config_pda,
      lamports,
      space,
      owner: &crate::ID,
    }
    .invoke_signed(&signer)?;

    log!("InitConfig after initialization");
    self.config_pda.can_borrow_mut_data()?;
    let config = Config::load(&config_pda)?;
    config.authority = *original_owner.key();
    config.fee = fee.to_be_bytes();
    config.bump = bump;
    Ok(())
  }
}
impl<'a> TryFrom<(&'a [u8], &'a [AccountInfo])> for InitConfig<'a> {
  type Error = ProgramError;

  fn try_from(value: (&'a [u8], &'a [AccountInfo])) -> Result<Self, Self::Error> {
    log!("InitConfig try_from");
    let (data, accounts) = value;
    log!("accounts len: {}, data len: {}", accounts.len(), data.len());

    let [authority, config_pda, original_owner, system_program] = accounts else {
      return Err(ProgramError::NotEnoughAccountKeys);
    };
    //let seeds: &'a [Seed<'a>] = &'a [Seed::from(b"vault".as_slice())];
    let fee = parse_u64(data)?;
    Ok(Self {
      authority,
      config_pda,
      original_owner,
      system_program,
      fee,
    })
  }
}
