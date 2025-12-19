use core::convert::TryFrom;
use pinocchio::{account_info::AccountInfo, program_error::ProgramError, ProgramResult};
use pinocchio_log::log;

use crate::{check_pda, instructions::check_signer, parse_u32, parse_u64, writable, MyError};
pub struct UpdateConfigStatus<'a> {
  pub authority: &'a AccountInfo,
}
pub struct UpdateConfigFee<'a> {
  pub authority: &'a AccountInfo,
}
pub struct UpdateConfigAuthority<'a> {
  pub authority: &'a AccountInfo,
}

/// Update Config PDA
pub struct UpdateConfig<'a> {
  pub authority: &'a AccountInfo,
  pub pda1: &'a AccountInfo,
  pub pda2: &'a AccountInfo,
  pub u8s: [u8; 8],
  pub u32s: [u32; 4],
  pub u64s: [u64; 4],
  //pub datalen: usize,
}
impl<'a> UpdateConfig<'a> {
  pub const DISCRIMINATOR: &'a u8 = &13;

  pub fn process(self) -> ProgramResult {
    let UpdateConfig {
      authority,
      pda1: _,
      pda2: _,
      u8s: _,
      u32s: _,
      u64s: _,
      //datalen: _,
    } = self;
    log!("UpdateConfig process()");
    check_signer(authority)?;
    //writable(pda1)?;
    //writable(pda2)?;

    log!("UpdateConfig 2");
    //check_pda(pda1)?;
    //check_pda(pda2)?;
    log!("UpdateConfig 3");

    log!("UpdateConfig 4");

    log!("UpdateConfig 5");
    /*match self.datalen as usize {
      len if len == size_of::<UpdateConfigStatus>() => self.update_status()?,
      len if len == size_of::<UpdateConfigFee>() => self.update_fee()?,
      len if len == size_of::<UpdateConfigAuthority>() => self.update_authority()?,
      _ => return Err(..),
    }*/
    Ok(())
  }
  pub fn update_status(self) -> ProgramResult {
    Ok(())
  }
  pub fn update_fee(self) -> ProgramResult {
    Ok(())
  }
  pub fn update_authority(self) -> ProgramResult {
    Ok(())
  }
}
impl<'a> TryFrom<(&'a [u8], &'a [AccountInfo])> for UpdateConfig<'a> {
  type Error = ProgramError;

  fn try_from(value: (&'a [u8], &'a [AccountInfo])) -> Result<Self, Self::Error> {
    log!("UpdateConfig try_from");
    let (data, accounts) = value;
    log!("accounts len: {}, data len: {}", accounts.len(), data.len());

    let [authority, pda1, pda2] = accounts else {
      return Err(ProgramError::NotEnoughAccountKeys);
    };

    //TODO: check all data size in every function
    //u32size = core::mem::size_of::<u32>();//4
    // u64size = core::mem::size_of::<u64>();//8
    //let expected_size = 8 + u32size * 4 + u64size * 4; // 8 + 4*4+ 8*4 = 56
    let expected_size: usize = 56;
    log!("expected_size: {}", expected_size);
    if data.len() != expected_size {
      return Err(MyError::InputDataLen.into());
    }
    let u8s = [
      data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7],
    ];
    let u32a = parse_u32(&data[8..12])?;
    let u32b = parse_u32(&data[12..16])?;
    let u32c = parse_u32(&data[16..20])?;
    let u32d = parse_u32(&data[20..24])?;
    let u32s = [u32a, u32b, u32c, u32d];
    let num64a = parse_u64(&data[24..32])?;
    let num64b = parse_u64(&data[32..40])?;
    let num64c = parse_u64(&data[40..48])?;
    let num64d = parse_u64(&data[48..56])?;
    let u64s = [num64a, num64b, num64c, num64d];
    log!("u8s: {}, u32s: {}, u64s: {}", &u8s, &u32s, &u64s);
    Ok(Self {
      authority,
      pda1,
      pda2,
      u8s,
      u32s,
      u64s,
    })
  }
}
