use core::convert::TryFrom;
use pinocchio::{account_info::AccountInfo, program_error::ProgramError, ProgramResult};
use pinocchio_log::log;

use crate::{
  check_pda, instructions::check_signer, min_data_len, parse_u32, parse_u64, to32bytes, u8_to_bool,
  writable, Config, MyError, StatusEnum,
};

/// Update Config PDA
pub struct UpdateConfig<'a> {
  pub authority: &'a AccountInfo,
  pub config_pda: &'a AccountInfo,
  pub account1: &'a AccountInfo,
  pub account2: &'a AccountInfo,
  pub bools: [bool; 4],
  pub u8s: [u8; 4],
  pub u32s: [u32; 4],
  pub u64s: [u64; 4],
  pub str_u8array: &'a [u8; 32], //pub str_u8: &'a [u8], //pub datalen: usize,
  pub config: &'a mut Config,
}
//impl<'a> Copy for UpdateConfig<'a> {}
impl<'a> UpdateConfig<'a> {
  pub const DISCRIMINATOR: &'a u8 = &13;

  pub fn process(self) -> ProgramResult {
    log!("UpdateConfig process()");
    match self.u8s[0] {
      0 => self.update_status(),
      1 => self.update_fee(),
      2 => self.update_authority(),
      _ => Err(MyError::FunctionSelector.into()),
    }
  }

  pub fn add_tokens(self) -> ProgramResult {
    log!("UpdateConfig add_tokens()");
    let mutated_state = (self.config.token_balance())
      .checked_add(self.u64s[1])
      .ok_or_else(|| ProgramError::ArithmeticOverflow)?;
    //self.config.token_balance = mutated_state.to_le_bytes();
    Ok(())
  }

  pub fn update_status(self) -> ProgramResult {
    log!("UpdateConfig update_status()");
    Ok(())
  }
  //TODO: WHY do tests run twice??
  pub fn update_fee(self) -> ProgramResult {
    log!("UpdateConfig update_fee()");
    self.config.fee = self.u64s[0].to_le_bytes();
    //self.config.status = self.u8s[1];
    self.config.status = StatusEnum::from(self.u8s[1]);
    self.config.str_u8array = *self.str_u8array;
    self.add_tokens()?;
    //self.update_authority()?;
    Ok(())
  }
  pub fn update_authority(self) -> ProgramResult {
    self.config.authority = *self.account1.key();
    Ok(())
  }
}
impl<'a> TryFrom<(&'a [u8], &'a [AccountInfo])> for UpdateConfig<'a> {
  type Error = ProgramError;

  fn try_from(value: (&'a [u8], &'a [AccountInfo])) -> Result<Self, Self::Error> {
    log!("UpdateConfig try_from");
    let (data, accounts) = value;
    log!("accounts len: {}, data len: {}", accounts.len(), data.len());

    let [authority, config_pda, account1, account2] = accounts else {
      return Err(ProgramError::NotEnoughAccountKeys);
    };

    /* check minimum data size in try_from!
    u32size = core::mem::size_of::<u32>();//4
    u64size = core::mem::size_of::<u64>();//8
    let expected_size = 8 + u32size * 4 + u64size * 4; // 8 + 4*4+ 8*4 = 56
    let expected_size: usize = 56;
    log!("expected_size: {}", expected_size);
    if data.len() != expected_size {
      return Err(MyError::InputDataLen.into());
    }*/
    let max_data_size1 = 88;
    min_data_len(data, max_data_size1)?; //56+32

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
    let str_u8: &[u8] = &data[56..max_data_size1];
    log!("str_u8: {}", str_u8);
    let str_u8array = to32bytes(str_u8)?;

    //check Status input range
    if u8s[1] > 3 {
      return Err(MyError::InputStatus.into());
    }

    check_signer(authority)?;
    writable(config_pda)?;
    check_pda(config_pda)?;

    let config = Config::load(&config_pda)?;
    if config.authority != *authority.key() {
      return Err(MyError::PdaAuthority.into());
    }
    // cannot use self in "0 => Self.process(),
    Ok(Self {
      authority,
      config_pda,
      account1,
      account2,
      u8s,
      bools,
      u32s,
      u64s,
      str_u8array,
      config,
    })
  }
}
/*match self.datalen as usize {
  len if len == size_of::<UpdateConfigStatus>() => self.update_status()?,
  len if len == size_of::<UpdateConfigFee>() => self.update_fee()?,
  len if len == size_of::<UpdateConfigAuthority>() => self.update_authority()?,
  _ => return Err(..),
} */
