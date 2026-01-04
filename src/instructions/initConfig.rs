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
  check_sysprog, derive_pda1, empty_lamport, get_time, instructions::check_signer, min_data_len,
  parse_u64, to32bytes, u8_to_bool, u8_to_status, Config, Ee, Status, CONFIG_SEED,
};

/// Init Config PDA
pub struct InitConfig<'a> {
  pub signer: &'a AccountInfo,
  pub config_pda: &'a AccountInfo,
  pub prog_owner: &'a AccountInfo,
  pub prog_admin: &'a AccountInfo,
  pub system_program: &'a AccountInfo,
  pub fee: u64,
  pub is_authorized: bool,
  pub status: Status,
  pub str_u8array: [u8; 32],
}
impl<'a> InitConfig<'a> {
  pub const DISCRIMINATOR: &'a u8 = &12;

  pub fn process(self) -> ProgramResult {
    let InitConfig {
      signer,
      config_pda,
      prog_owner,
      prog_admin,
      system_program: _,
      fee,
      is_authorized,
      status,
      str_u8array,
    } = self;
    log!("InitConfig process()");
    empty_lamport(config_pda)?;

    log!("InitConfig 3");
    let lamports = Rent::get()?.minimum_balance(Config::LEN); //space.try_into().unwrap()
    let space = Config::LEN as u64;

    log!("InitConfig 4");
    let (expected_config_pda, bump) = derive_pda1(prog_owner, CONFIG_SEED)?;

    log!("InitConfig 5");
    if expected_config_pda != *config_pda.key() {
      return Err(Ee::ConfigPDA.into());
    }

    log!("InitConfig 6");
    let seeds = [
      Seed::from(CONFIG_SEED),
      Seed::from(prog_owner.key().as_ref()),
      Seed::from(core::slice::from_ref(&bump)),
    ];
    let seed_signer = [Signer::from(&seeds)];

    log!("InitConfig 7");
    pinocchio_system::instructions::CreateAccount {
      from: signer,
      to: config_pda,
      lamports,
      space,
      owner: &crate::ID,
    }
    .invoke_signed(&seed_signer)?;

    log!("InitConfig after initialization");
    let time = get_time()?;
    self.config_pda.can_borrow_mut_data()?;
    let config = Config::load(&config_pda)?;
    config.set_prog_owner(*prog_owner.key());
    config.set_admin(*prog_admin.key());
    config.set_str_u8array(str_u8array);
    config.set_fee(fee);
    config.set_updated_at(time);
    config.set_is_authorized(is_authorized);
    config.set_status(status);
    config.set_bump(bump);
    Ok(())
  }
}
impl<'a> TryFrom<(&'a [u8], &'a [AccountInfo])> for InitConfig<'a> {
  type Error = ProgramError;

  fn try_from(value: (&'a [u8], &'a [AccountInfo])) -> Result<Self, Self::Error> {
    log!("InitConfig try_from");
    let (data, accounts) = value;
    log!("accounts len: {}, data len: {}", accounts.len(), data.len());

    let [signer, config_pda, prog_owner, prog_admin, system_program] = accounts else {
      return Err(ProgramError::NotEnoughAccountKeys);
    };
    check_signer(signer)?;
    check_sysprog(system_program)?;
    //writable(config_pda)?;

    let data_size1 = 42; //1+1+8+32
    min_data_len(data, data_size1)?;

    let is_authorized = u8_to_bool(data[0])?;
    let status = u8_to_status(data[1])?;
    let fee = parse_u64(&data[2..10])?;
    let str_u8array = *to32bytes(&data[10..data_size1])?;

    Ok(Self {
      signer,
      config_pda,
      prog_owner,
      prog_admin,
      system_program,
      fee,
      is_authorized,
      status,
      str_u8array,
    })
  }
}
