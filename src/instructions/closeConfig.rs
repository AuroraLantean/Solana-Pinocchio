use core::convert::TryFrom;
use pinocchio::{account_info::AccountInfo, program_error::ProgramError, ProgramResult};
use pinocchio_log::log;

use crate::{instructions::check_signer, rent_exempt, writable};

/// Close PDA
pub struct CloseConfigPda<'a> {
    pub authority: &'a AccountInfo,
    pub pda: &'a AccountInfo,
    pub dest: &'a AccountInfo,
}
impl<'a> CloseConfigPda<'a> {
    pub const DISCRIMINATOR: &'a u8 = &12;

    pub fn process(self) -> ProgramResult {
        let CloseConfigPda {
            authority,
            pda,
            dest,
        } = self;
        log!("CloseConfigPda process()");
        check_signer(authority)?;
        writable(pda)?;

        log!("CloseConfigPda 2");
        if !pda.is_owned_by(&crate::ID) {
            return Err(ProgramError::IllegalOwner);
        }
        log!("CloseConfigPda 3");
        // if pda.data_len().ne(&crate::state::LEN) {
        //     return Err(ProgramError::InvalidAccountData);
        // }
        log!("CloseConfigPda 4");
        //set the first byte to 255
        {
            let mut data = pda.try_borrow_mut_data()?;
            data[0] = 0xff;
        }
        //https://learn.blueshift.gg/en/courses/pinocchio-for-dummies/pinocchio-accounts
        *dest.try_borrow_mut_lamports()? += *pda.try_borrow_lamports()?;
        //resize the account to only the 1st byte
        pda.resize(1)?;
        pda.close()?;
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

        let [authority, pda, dest] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };
        Ok(Self {
            authority,
            pda,
            dest,
        })
    }
}
