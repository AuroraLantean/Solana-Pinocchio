use pinocchio::{account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey};

use crate::MyError;

#[derive(Clone, Copy, Debug)]
#[repr(C)] //0..8 	Discriminator 	8 bytes
pub struct Config {
  pub authority: Pubkey,      //32
  pub fee: [u8; 8],           //u64,
  pub sol_balance: [u8; 8],   //8
  pub token_balance: [u8; 8], //8
  pub bump: u8,               // 1
}
impl Config {
  pub const LEN: usize = core::mem::size_of::<Config>();

  pub fn load(account: &AccountInfo) -> Result<&mut Self, ProgramError> {
    if account.data_len() != Config::LEN {
      return Err(MyError::ConfigDataLengh.into());
    }
    //assert_eq!(account.data_len(), Config::LEN);
    if account.owner() != &crate::ID {
      return Err(MyError::ForeignPDA.into());
    }
    //assert_eq!(account.owner(), &crate::ID);
    unsafe { Ok(&mut *(account.borrow_mut_data_unchecked().as_ptr() as *mut Self)) }
  }
}
