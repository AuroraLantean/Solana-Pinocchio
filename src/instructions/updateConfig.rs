use core::{convert::TryFrom, ops::Add};
use pinocchio::{account_info::AccountInfo, program_error::ProgramError, ProgramResult};
use pinocchio_log::log;

use crate::{
  instructions::check_signer, parse_u32, parse_u64, u8_slice_to_array, u8_to_bool, Config, MyError,
};
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
  pub config_pda: &'a AccountInfo,
  pub account1: &'a AccountInfo,
  pub bools: [bool; 4],
  pub u8s: [u8; 4],
  pub u32s: [u32; 4],
  pub u64s: [u64; 4],
  pub str_u8array: &'a [u8; 32], //pub str_u8: &'a [u8], //pub datalen: usize,
}
impl<'a> UpdateConfig<'a> {
  pub const DISCRIMINATOR: &'a u8 = &13;

  pub fn process(self) -> ProgramResult {
    let UpdateConfig {
      authority: _,
      config_pda: _,
      account1: _,
      bools: _,
      u8s: _,
      u32s: _,
      u64s: _,
      str_u8array: _, //datalen: _,
    } = self;
    log!("UpdateConfig process()");
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
    let config = Config::load(&self.config_pda)?;
    if config.authority != *self.authority.key() {
      return Err(ProgramError::IncorrectAuthority);
    }
    let fee = u64::from_le_bytes(config.fee);
    let amt = self.u64s[0];
    config.fee = amt.add(fee).to_be_bytes();
    Ok(())
  } /*let mut data = self.config_pda.data.borow_mut();
    let counter_data = Config {
      fee: ,
    }; */
  pub fn update_authority(self) -> ProgramResult {
    let config = Config::load(&self.config_pda)?;
    if config.authority != *self.authority.key() {
      return Err(ProgramError::IncorrectAuthority);
    }
    config.authority = *self.account1.key();
    Ok(())
  }
}
impl<'a> TryFrom<(&'a [u8], &'a [AccountInfo])> for UpdateConfig<'a> {
  type Error = ProgramError;

  fn try_from(value: (&'a [u8], &'a [AccountInfo])) -> Result<Self, Self::Error> {
    log!("UpdateConfig try_from");
    let (data, accounts) = value;
    log!("accounts len: {}, data len: {}", accounts.len(), data.len());

    let [authority, config_pda, account1] = accounts else {
      return Err(ProgramError::NotEnoughAccountKeys);
    };

    //TODO: check all data size in every function
    /*u32size = core::mem::size_of::<u32>();//4
    u64size = core::mem::size_of::<u64>();//8
    let expected_size = 8 + u32size * 4 + u64size * 4; // 8 + 4*4+ 8*4 = 56
    let expected_size: usize = 56;
    log!("expected_size: {}", expected_size);
    if data.len() != expected_size {
      return Err(MyError::InputDataLen.into());
    }*/
    let max_data_len = 88; //56+32
    if data.len() > max_data_len {
      return Err(MyError::InputDataLengthOverMax.into());
    }
    let b0 = u8_to_bool(data[0])?;
    let b1 = u8_to_bool(data[1])?;
    let b2 = u8_to_bool(data[2])?;
    let b3 = u8_to_bool(data[3])?;
    let bools = [b0, b1, b2, b3];

    let u8s = [data[4], data[5], data[6], data[7]];
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
    let str_u8: &[u8] = &data[56..max_data_len];
    log!("str_u8: {}", str_u8);
    let str_u8array = u8_slice_to_array(str_u8)?;
    check_signer(authority)?;

    Ok(Self {
      authority,
      config_pda,
      account1,
      u8s,
      bools,
      u32s,
      u64s,
      str_u8array,
    })
  }
}
