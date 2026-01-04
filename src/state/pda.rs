use pinocchio::{account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey};

use crate::Ee;

#[derive(Clone, Copy, Debug)]
#[repr(C)] //0..8 	Discriminator 	8 bytes
pub struct Config {
  pub prog_owner: Pubkey,    // 32
  pub admin: Pubkey,         // 32
  pub str_u8array: [u8; 32], // 32
  fee: [u8; 8],              // 8 for u64,
  sol_balance: [u8; 8],      // 8
  token_balance: [u8; 8],    // 8
  updated_at: [u8; 4],       // 4 for u32
  pub is_authorized: bool,   // 1
  pub status: Status,        // 1
  pub bump: u8,              // 1
} // padding: [u8; 6] since the struct size needs to be aligned to 32 bytes.

impl Config {
  pub const LEN: usize = core::mem::size_of::<Self>();
  //Getters or Accessors: Safe Direct value copy, no reference created
  pub fn fee(&self) -> u64 {
    u64::from_le_bytes(self.fee)
  }
  pub fn sol_balance(&self) -> u64 {
    u64::from_le_bytes(self.sol_balance)
  }
  pub fn token_balance(&self) -> u64 {
    u64::from_le_bytes(self.token_balance)
  }
  pub fn updated_at(&self) -> u32 {
    u32::from_le_bytes(self.updated_at)
  }
  pub fn read(pda: &AccountInfo) -> Result<&Self, ProgramError> {
    Self::check(pda)?;
    Ok(unsafe { &*(pda.borrow_mut_data_unchecked().as_ptr() as *const Self) })
  }
  pub fn check(pda: &AccountInfo) -> Result<(), ProgramError> {
    if pda.data_len() != Self::LEN {
      return Err(Ee::ConfigDataLengh.into());
    }
    if pda.owner() != &crate::ID {
      return Err(Ee::ForeignPDA.into());
    }
    // CHECK alignment for the most restrictive field (u64 in this case)... Alignment requirement checking can be removed ONLY IF you know all numbers are using u8 arrays
    /*if (pda.borrow_mut_data_unchecked().as_ptr() as usize) % core::mem::align_of::<Self>() != 0 { return Err();  }*/
    Ok(())
  }
  //better to use setters below
  pub fn load(pda: &AccountInfo) -> Result<&mut Self, ProgramError> {
    Self::check(pda)?;
    unsafe { Ok(&mut *(pda.borrow_mut_data_unchecked().as_ptr() as *mut Self)) }
  }
  pub fn load_unchecked(pda: &AccountInfo) -> Result<&mut Self, ProgramError> {
    unsafe { Ok(&mut *(pda.borrow_mut_data_unchecked().as_ptr() as *mut Self)) }
  }
  //Setters
  pub fn set_prog_owner(&mut self, prog_owner: Pubkey) {
    self.prog_owner = prog_owner;
  }
  pub fn set_admin(&mut self, admin: Pubkey) {
    self.admin = admin;
  }
  pub fn set_str_u8array(&mut self, str_u8array: [u8; 32]) {
    self.str_u8array = str_u8array;
  }
  pub fn set_fee(&mut self, amt: u64) {
    self.fee = amt.to_le_bytes();
  }
  pub fn set_sol_balance(&mut self, amt: u64) {
    self.sol_balance = amt.to_le_bytes();
  }
  pub fn set_token_balance(&mut self, amt: u64) {
    self.token_balance = amt.to_le_bytes();
  }
  pub fn set_updated_at(&mut self, amt: u32) {
    self.updated_at = amt.to_le_bytes();
  }
  pub fn set_bump(&mut self, amt: u8) {
    self.bump = amt;
  }
  pub fn set_status(&mut self, status: Status) {
    self.status = status;
  }
  pub fn set_is_authorized(&mut self, boo: bool) {
    self.is_authorized = boo;
  }
}

//#[repr(C)] keeps the struct layout the same across different architectures
#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum Status {
  Waiting = 0,
  Active = 1,
  Expired = 2,
  Paused = 3,
  Canceled = 4,
}
impl From<u8> for Status {
  fn from(num: u8) -> Self {
    match num {
      0 => Status::Waiting,
      1 => Status::Active,
      2 => Status::Expired,
      3 => Status::Paused,
      4 => Status::Canceled,
      _ => Status::Expired,
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
      return Err(Ee::PdaDataLen.into());
    }
    //assert_eq!(pda.data_len(), Escrow::LEN);
    if pda.owner() != &crate::ID {
      return Err(Ee::ForeignPDA.into());
    }
    //assert_eq!(pda.owner(), &crate::ID);
    unsafe { Ok(&mut *(pda.borrow_mut_data_unchecked().as_ptr() as *mut Self)) }
  }
  pub fn load_unchecked(pda: &AccountInfo) -> Result<&mut Self, ProgramError> {
    unsafe { Ok(&mut *(pda.borrow_mut_data_unchecked().as_ptr() as *mut Self)) }
  }
}
