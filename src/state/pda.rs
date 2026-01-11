use pinocchio::{
  account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey, ProgramResult,
};

use crate::{none_zero_u64, Ee};
//TODO: Bytemuck is a great library that makes it easy to read and write byte arrays as structs.
#[derive(Clone, Copy, Debug)]
#[repr(C)] //0..8 	Discriminator 	8 bytes
pub struct Config {
  mint0: Pubkey,          // 32
  mint1: Pubkey,          // 32
  mint2: Pubkey,          // 32
  mint3: Pubkey,          // 32
  vault: Pubkey,          // 32
  prog_owner: Pubkey,     // 32
  admin: Pubkey,          // 32
  str_u8array: [u8; 32],  // 32
  fee: [u8; 8],           // 8 for u64,
  sol_balance: [u8; 8],   // 8
  token_balance: [u8; 8], // 8
  updated_at: [u8; 4],    // 4 for u32
  is_authorized: bool,    // 1
  status: u8,             // 1
  vault_bump: u8,         // 1
  bump: u8,               // 1
} // padding: [u8; 6] since the struct size needs to be aligned to 32 bytes.

impl Config {
  pub const LEN: usize = core::mem::size_of::<Self>();
  //Getters or Accessors: Safe Direct value copy, no reference created
  pub fn mint0(&self) -> &Pubkey {
    &self.mint0
  }
  pub fn mint1(&self) -> &Pubkey {
    &self.mint1
  }
  pub fn mint2(&self) -> &Pubkey {
    &self.mint2
  }
  pub fn mint3(&self) -> &Pubkey {
    &self.mint3
  }
  pub fn mints(&self) -> [&Pubkey; 4] {
    [&self.mint0, &self.mint1, &self.mint2, &self.mint3]
  }
  pub fn vault(&self) -> &Pubkey {
    &self.vault
  }
  pub fn prog_owner(&self) -> &Pubkey {
    &self.prog_owner
  }
  pub fn admin(&self) -> &Pubkey {
    &self.admin
  }
  pub fn str_u8array(&self) -> &[u8; 32] {
    &self.str_u8array
  }
  pub fn status(&self) -> Status {
    self.status.into()
  }
  pub fn is_authorized(&self) -> bool {
    self.is_authorized
  }
  pub fn vault_bump(&self) -> u8 {
    self.vault_bump
  }
  pub fn bump(&self) -> u8 {
    self.bump
  }
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
  /* pub fn close_authority(&self) -> Option<&Pubkey> {
      if self.has_close_authority() {
          Some(self.close_authority_unchecked())
      } else {
          None
      }
  }*/
  //----------== read
  pub fn check(pda: &AccountInfo) -> Result<(), ProgramError> {
    if pda.data_len() != Self::LEN {
      return Ee::ConfigDataLengh.e();
    }
    if pda.owner() != &crate::ID {
      return Ee::ForeignPDA.e();
    }
    // CHECK alignment for the most restrictive field (u64 in this case)... Alignment requirement checking can be removed ONLY IF you know all numbers are using u8 arrays
    /*if (pda.borrow_mut_data_unchecked().as_ptr() as usize) % core::mem::align_of::<Self>() != 0 { return Err();  }*/
    Ok(())
  }
  //better to use setters below
  pub fn from_account_info(pda: &AccountInfo) -> Result<&mut Self, ProgramError> {
    Self::check(pda)?;
    unsafe { Ok(&mut *(pda.borrow_mut_data_unchecked().as_ptr() as *mut Self)) }
    /*Ok(Ref::map(account_info.try_borrow_data()?, |data| unsafe {
        Self::from_bytes_unchecked(data)
    })) */
  }
  //Must: there are no mutable borrows of the account data
  #[inline]
  pub unsafe fn from_account_info_unchecked(pda: &AccountInfo) -> Result<&Self, ProgramError> {
    Self::check(pda)?;
    Ok(Self::from_bytes_unchecked(pda.borrow_data_unchecked()))
    //Ok(&mut *(pda.borrow_mut_data_unchecked().as_ptr() as *mut Self))
  }
  /// The caller must ensure that `bytes` contains a valid representation of `Account`, and
  /// it is properly aligned to be interpreted as an instance of `Account`.
  /// At the moment `Account` has an alignment of 1 byte.
  /// This method does not perform a length validation.
  pub unsafe fn from_bytes_unchecked(bytes: &[u8]) -> &Self {
    &*(bytes.as_ptr() as *const &Config)
  }
  //----------== Setters
  pub fn set_mint0(&mut self, mint0: Pubkey) {
    self.mint0 = mint0;
  }
  pub fn set_mint1(&mut self, mint1: Pubkey) {
    self.mint1 = mint1;
  }
  pub fn set_mint2(&mut self, mint2: Pubkey) {
    self.mint2 = mint2;
  }
  pub fn set_mint3(&mut self, mint3: Pubkey) {
    self.mint3 = mint3;
  }
  pub fn set_mints(&mut self, mints: [Pubkey; 4]) {
    self.mint0 = mints[0];
    self.mint1 = mints[1];
    self.mint2 = mints[2];
    self.mint3 = mints[3];
  }
  pub fn set_vault(&mut self, vault: Pubkey) {
    self.vault = vault;
  }
  pub fn set_prog_owner(&mut self, prog_owner: Pubkey) {
    self.prog_owner = prog_owner;
  }
  pub fn set_admin(&mut self, admin: Pubkey) {
    self.admin = admin;
  }
  pub fn set_str_u8array(&mut self, str_u8array: [u8; 32]) {
    self.str_u8array = str_u8array;
  }
  pub fn set_fee(&mut self, amt: u64) -> ProgramResult {
    none_zero_u64(amt)?;
    self.fee = amt.to_le_bytes();
    Ok(())
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
  pub fn set_vault_bump(&mut self, amt: u8) {
    self.vault_bump = amt;
  }
  pub fn set_bump(&mut self, amt: u8) {
    self.bump = amt;
  }
  pub fn set_status(&mut self, status: u8) {
    self.status = status;
  }
  pub fn set_is_authorized(&mut self, boo: bool) {
    self.is_authorized = boo;
  }
}

//#[repr(C)] keeps the struct layout the same across different architectures
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
      _ => Status::Canceled,
    }
  }
} //Status::Uninitialized as u8
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
      return Err(Ee::EscrowDataLengh.into());
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
