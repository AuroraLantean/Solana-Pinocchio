use core::convert::TryFrom;
use pinocchio::{
  cpi::{Seed, Signer},
  error::ProgramError,
  AccountView, ProgramResult,
};
use pinocchio_log::log;
use pinocchio_system::instructions::Allocate;

use crate::{
  check_pda, data_len, instructions::check_signer, none_zero_u64, parse_u64, writable, Config,
};

/// Close PDA
pub struct ConfigResize<'a> {
  pub authority: &'a AccountView,
  pub config_pda: &'a AccountView,
  pub first_prog_owner: &'a AccountView,
  pub system_program: &'a AccountView,
  pub new_size: u64,
  pub bump: u8,
}
impl<'a> ConfigResize<'a> {
  pub const DISCRIMINATOR: &'a u8 = &19;

  pub fn process(self) -> ProgramResult {
    let ConfigResize {
      authority: _,
      config_pda,
      first_prog_owner,
      system_program: _,
      new_size,
      bump,
    } = self;
    log!("ConfigResize process()");
    let seeds = [
      Seed::from(Config::SEED),
      Seed::from(first_prog_owner.address().as_ref()),
      Seed::from(core::slice::from_ref(&bump)),
    ];
    let seed_signer = Signer::from(&seeds);
    log!("ConfigResize allocate()");
    Allocate {
      account: config_pda,
      space: new_size,
    }
    .invoke_signed(&[seed_signer])?;

    // AllocateWithSeed {
    //   account: config_pda,
    //   base: authority,
    //   seed: "config",
    //   space: new_size,
    //   owner: &PROG_ADDR,
    // }
    // .invoke()?;
    // Realloc {
    //   account: config_pda,
    //   space: new_size,
    //   payer: authority,
    //   system_program: system_program,
    // }
    // .invoke()?;
    Ok(())
  }
}
impl<'a> TryFrom<(&'a [u8], &'a [AccountView])> for ConfigResize<'a> {
  type Error = ProgramError;

  fn try_from(value: (&'a [u8], &'a [AccountView])) -> Result<Self, Self::Error> {
    log!("ConfigResize try_from");
    let (data, accounts) = value;
    log!("accounts len: {}, data len: {}", accounts.len(), data.len());
    data_len(data, 8)?;

    let [authority, config_pda, first_prog_owner, system_program] = accounts else {
      return Err(ProgramError::NotEnoughAccountKeys);
    };
    check_signer(authority)?;
    writable(config_pda)?;
    check_pda(config_pda)?;

    config_pda.check_borrow_mut()?;
    let config: &mut Config = Config::from_account_view(&config_pda)?;
    // if config.admin().ne(authority.key()) && config.prog_owner().ne(authority.key()) {
    //   return Err(ProgramError::IncorrectAuthority);
    // }
    let new_size = parse_u64(&data[0..8])?;
    let bump = config.bump(); //data[9];
    log!("new_size: {}", new_size);
    none_zero_u64(new_size)?;

    Ok(Self {
      authority,
      config_pda,
      first_prog_owner,
      system_program,
      new_size,
      bump,
    })
  }
}
