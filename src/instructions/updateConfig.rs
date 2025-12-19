use core::convert::TryFrom;
use pinocchio::{account_info::AccountInfo, program_error::ProgramError, ProgramResult};
use pinocchio_log::log;

use crate::{check_pda, instructions::check_signer, parse_u64, writable};
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
    pub datalen: usize,
    pub num8a: u8,
    pub num8b: u8,
    pub num64a: u64,
    pub num64b: u64,
}
impl<'a> UpdateConfig<'a> {
    pub const DISCRIMINATOR: &'a u8 = &13;

    pub fn process(self) -> ProgramResult {
        let UpdateConfig {
            authority,
            pda1,
            pda2,
            datalen: u32,
            num8a,
            num8b,
            num64a,
            num64b,
        } = self;
        log!("UpdateConfig process()");
        check_signer(authority)?;
        writable(pda1)?;
        //writable(pda2)?;

        log!("UpdateConfig 2");
        check_pda(pda1)?;
        check_pda(pda2)?;
        log!("UpdateConfig 3");

        log!("UpdateConfig 4");

        log!("UpdateConfig 5");
        match self.datalen as usize {
            len if len == size_of::<UpdateConfigStatus>() => self.update_status()?,
            len if len == size_of::<UpdateConfigFee>() => self.update_fee()?,
            len if len == size_of::<UpdateConfigAuthority>() => self.update_authority()?,
            _ => return Err(ProgramError::InvalidInstructionData),
        }
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
        let datalen = data.len();

        let [authority, pda1, pda2] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };
        let num8a = data[0];
        let num8b = data[1];
        let num64a = parse_u64(&data[2..10])?;
        let num64b = parse_u64(&data[11..19])?;
        log!(
            "num8a: {}, num8b: {}, num64a: {}, num64b: {}",
            num8a,
            num8b,
            num64a,
            num64b
        );
        Ok(Self {
            authority,
            pda1,
            pda2,
            datalen,
            num8a,
            num8b,
            num64a,
            num64b,
        })
    }
}
