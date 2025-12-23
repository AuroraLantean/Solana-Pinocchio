use core::convert::TryFrom;
use pinocchio::{account_info::AccountInfo, program_error::ProgramError, ProgramResult};
use pinocchio_log::log;

use crate::{instructions::check_signer, writable, Config};

/// Close PDA
pub struct CloseConfigPda<'a> {
  pub authority: &'a AccountInfo,
  pub config_pda: &'a AccountInfo,
  pub dest: &'a AccountInfo,
}
impl<'a> CloseConfigPda<'a> {
  pub const DISCRIMINATOR: &'a u8 = &14;

  pub fn process(self) -> ProgramResult {
    let CloseConfigPda {
      authority,
      config_pda,
      dest,
    } = self;
    log!("CloseConfigPda process()");
    check_signer(authority)?;
    writable(config_pda)?;

    log!("CloseConfigPda 2");
    let config = Config::load(&self.config_pda)?;
    log!("CloseConfigPda 3");
    if config.authority != *self.authority.key() {
      return Err(ProgramError::IncorrectAuthority);
    }
    log!("CloseConfigPda 4");
    //set the first byte to 255
    {
      let mut data = config_pda.try_borrow_mut_data()?;
      data[0] = 0xff;
    }
    //https://learn.blueshift.gg/en/courses/pinocchio-for-dummies/pinocchio-accounts
    *dest.try_borrow_mut_lamports()? += *config_pda.try_borrow_lamports()?;
    //resize the account to only the 1st byte
    config_pda.resize(1)?;
    config_pda.close()?;
    log!("CloseConfigPda 5");
    Ok(())
  }
}
impl<'a> TryFrom<(&'a [u8], &'a [AccountInfo])> for CloseConfigPda<'a> {
  type Error = ProgramError;

  fn try_from(value: (&'a [u8], &'a [AccountInfo])) -> Result<Self, Self::Error> {
    log!("CloseConfigPda try_from");
    let (data, accounts) = value;
    log!("accounts len: {}, data len: {}", accounts.len(), data.len());

    let [authority, config_pda, dest] = accounts else {
      return Err(ProgramError::NotEnoughAccountKeys);
    };

    Ok(Self {
      authority,
      config_pda,
      dest,
    })
  }
}
