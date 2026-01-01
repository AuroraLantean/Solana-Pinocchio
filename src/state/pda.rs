use pinocchio::{account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey};

use crate::MyError;

#[derive(Clone, Copy, Debug)]
#[repr(C)] //0..8 	Discriminator 	8 bytes
pub struct Config {
  pub authority: Pubkey,     // 32
  pub str_u8array: [u8; 32], // 32
  fee: [u8; 8],              // 8 for u64,
  sol_balance: [u8; 8],      // 8
  token_balance: [u8; 8],    // 8
  pub status: StatusEnum,    // 1
  pub bump: u8,              // 1
} // padding: [u8; 6] since the struct size needs to be aligned to 32 bytes.

impl Config {
  pub const LEN: usize = core::mem::size_of::<Self>();
  //Accessors: Safe Direct value copy, no reference created
  pub fn fee(&self) -> u64 {
    u64::from_le_bytes(self.fee)
  }
  pub fn sol_balance(&self) -> u64 {
    u64::from_le_bytes(self.sol_balance)
  }
  pub fn token_balance(&self) -> u64 {
    u64::from_le_bytes(self.token_balance)
  }

  pub fn check(pda: &AccountInfo) -> Result<(), ProgramError> {
    if pda.data_len() != Self::LEN {
      return Err(MyError::ConfigDataLengh.into());
    }
    if pda.owner() != &crate::ID {
      return Err(MyError::ForeignPDA.into());
    }
    // CHECK alignment for the most restrictive field (u64 in this case)... avoided by using byte arrays
    /*if (pda.borrow_mut_data_unchecked().as_ptr() as usize) % core::mem::align_of::<Self>() != 0 { return Err();  }*/
    Ok(())
  }
  pub fn load(pda: &AccountInfo) -> Result<&mut Self, ProgramError> {
    Self::check(pda)?;
    unsafe { Ok(&mut *(pda.borrow_mut_data_unchecked().as_ptr() as *mut Self)) }
  }
  pub fn load_unchecked(pda: &AccountInfo) -> Result<&mut Self, ProgramError> {
    unsafe { Ok(&mut *(pda.borrow_mut_data_unchecked().as_ptr() as *mut Self)) }
  }
  pub fn read(pda: &AccountInfo) -> Result<&Self, ProgramError> {
    Self::check(pda)?;
    Ok(unsafe { &*(pda.borrow_mut_data_unchecked().as_ptr() as *const Self) })
  }
}

#[repr(C)] //keeps the struct layout the same across different architectures
#[derive(Clone, Copy, Debug)]
pub enum StatusEnum {
  Waiting,
  Active,
  Expired,
  Paused,
}
impl From<u8> for StatusEnum {
  fn from(num: u8) -> Self {
    match num {
      0 => StatusEnum::Waiting,
      1 => StatusEnum::Active,
      2 => StatusEnum::Expired,
      3 => StatusEnum::Paused,
      _ => StatusEnum::Expired,
    }
  }
}

//------------==
#[derive(Clone, Copy, Debug)]
#[repr(C)] //0..8 	Discriminator 	8 bytes
pub struct Escrow {
  pub maker: Pubkey,      //32
  pub mint_maker: Pubkey, //32
  pub mint_taker: Pubkey, //32
  pub amount: [u8; 8],    //8
  pub bump: u8,           //1
}
impl Escrow {
  pub const LEN: usize = core::mem::size_of::<Escrow>();
  //pub const LEN: usize = 32 + 32 + 32 + 8 + 1;

  pub fn load(pda: &AccountInfo) -> Result<&mut Self, ProgramError> {
    if pda.data_len() != Escrow::LEN {
      return Err(MyError::PdaDataLen.into());
    }
    //assert_eq!(pda.data_len(), Escrow::LEN);
    if pda.owner() != &crate::ID {
      return Err(MyError::ForeignPDA.into());
    }
    //assert_eq!(pda.owner(), &crate::ID);
    unsafe { Ok(&mut *(pda.borrow_mut_data_unchecked().as_ptr() as *mut Self)) }
  }
  pub fn load_unchecked(pda: &AccountInfo) -> Result<&mut Self, ProgramError> {
    unsafe { Ok(&mut *(pda.borrow_mut_data_unchecked().as_ptr() as *mut Self)) }
  }
}
