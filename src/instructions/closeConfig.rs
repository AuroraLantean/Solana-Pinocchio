use core::convert::TryFrom;
use pinocchio::{error::ProgramError, AccountView, ProgramResult};
use pinocchio_log::log;

use crate::{check_pda, data_len, instructions::check_signer, writable, Config};

/// Close PDA
pub struct CloseConfigPda<'a> {
  pub authority: &'a AccountView,
  pub config_pda: &'a AccountView,
  pub dest: &'a AccountView,
}
impl<'a> CloseConfigPda<'a> {
  pub const DISCRIMINATOR: &'a u8 = &14;

  pub fn process(self) -> ProgramResult {
    let CloseConfigPda {
      authority: _,
      config_pda,
      dest,
    } = self;
    log!("CloseConfigPda process()");
    //set the first byte to 255
    {
      let mut data = config_pda.try_borrow_mut()?;
      data[0] = 0xff;
    }
    log!("CloseConfigPda 1");
    let dest_lam = dest.lamports();
    let config_lam = config_pda.lamports();
    dest.set_lamports(dest_lam + config_lam);
    config_pda.set_lamports(0);
    //https://learn.blueshift.gg/en/courses/pinocchio-for-dummies/pinocchio-accounts
    //*dest.try_borrow_mut_lamports()? += *config_pda.try_borrow_lamports()?;

    log!("CloseConfigPda 2"); //resize the account to only the 1st byte
    config_pda.resize(1)?;
    config_pda.close()?;
    Ok(())
  }
}
impl<'a> TryFrom<(&'a [u8], &'a [AccountView])> for CloseConfigPda<'a> {
  type Error = ProgramError;

  fn try_from(value: (&'a [u8], &'a [AccountView])) -> Result<Self, Self::Error> {
    log!("CloseConfigPda try_from");
    let (data, accounts) = value;
    log!("accounts len: {}, data len: {}", accounts.len(), data.len());
    data_len(data, 0)?;

    let [authority, config_pda, dest] = accounts else {
      return Err(ProgramError::NotEnoughAccountKeys);
    };
    check_signer(authority)?;
    writable(config_pda)?;
    check_pda(config_pda)?;

    config_pda.check_borrow_mut()?;
    let config: &mut Config = Config::from_account_view(&config_pda)?;
    if config.admin().ne(authority.address()) && config.prog_owner().ne(authority.address()) {
      return Err(ProgramError::IncorrectAuthority);
    }
    Ok(Self {
      authority,
      config_pda,
      dest,
    })
  }
}
