use pinocchio::{account_info::AccountInfo, Address, ProgramResult};

use crate::{none_zero_u64, Ee, ID};

//Vault to hold SOL and control Tokens, and has no struct to be declared
pub const VAULT_SEED: &[u8] = b"vault";
pub const ACCOUNT_DISCRIMINATOR_SIZE: usize = 8;
pub const VAULT_SIZE: usize = ACCOUNT_DISCRIMINATOR_SIZE + size_of::<u64>(); //SOL amount

//TODO: Bytemuck is a great library that makes it easy to read and write byte arrays as structs.
#[derive(Clone, Copy, Debug)]
#[repr(C)] //0..8 	Discriminator 	8 bytes
pub struct Config {
  mint0: Address,         // 32
  mint1: Address,         // 32
  mint2: Address,         // 32
  mint3: Address,         // 32
  vault: Address,         // 32
  prog_owner: Address,    // 32
  admin: Address,         // 32
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
  pub const SEED: &[u8] = b"config";
  //Getters or Accessors: Safe Direct value copy, no reference created
  pub fn mint0(&self) -> &Address {
    &self.mint0
  }
  pub fn mint1(&self) -> &Address {
    &self.mint1
  }
  pub fn mint2(&self) -> &Address {
    &self.mint2
  }
  pub fn mint3(&self) -> &Address {
    &self.mint3
  }
  pub fn mints(&self) -> [&Address; 4] {
    [&self.mint0, &self.mint1, &self.mint2, &self.mint3]
  }
  pub fn vault(&self) -> &Address {
    &self.vault
  }
  pub fn prog_owner(&self) -> &Address {
    &self.prog_owner
  }
  pub fn admin(&self) -> &Address {
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
  /* pub fn close_authority(&self) -> Option<&Address> {
      if self.has_close_authority() {
          Some(self.close_authority_unchecked())
      } else {
          None
      }
  }*/
  //----------== read
  pub fn check(pda: &AccountInfo) -> ProgramResult {
    if pda.data_len() != Self::LEN {
      return Ee::ConfigDataLengh.e();
    }
    if pda.owner() != &ID {
      return Ee::ConfigIsForeign.e();
    }
    // CHECK alignment for the most restrictive field (u64 in this case)... Alignment requirement checking can be removed ONLY IF you know all numbers are using u8 arrays
    /*if (pda.borrow_mut_data_unchecked().as_ptr() as usize) % core::mem::align_of::<Self>() != 0 { return Err();  }*/
    Ok(())
  }
  //better to use setters below
  pub fn from_account_info(pda: &AccountInfo) -> Result<&mut Self, ProgramResult> {
    Self::check(pda)?;
    unsafe { Ok(&mut *(pda.borrow_mut_data_unchecked().as_ptr() as *mut Self)) }
    /*Ok(Ref::map(account_info.try_borrow_data()?, |data| unsafe {
        Self::from_bytes_unchecked(data)
    })) */
  }
  //Must: there are no mutable borrows of the account data
  #[inline]
  pub unsafe fn from_account_info_unchecked(pda: &AccountInfo) -> Result<&Self, ProgramResult> {
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
  pub fn set_mint0(&mut self, pkey: &Address) {
    self.mint0 = *pkey;
  }
  pub fn set_mint1(&mut self, pkey: &Address) {
    self.mint1 = *pkey;
  }
  pub fn set_mint2(&mut self, pkey: &Address) {
    self.mint2 = *pkey;
  }
  pub fn set_mint3(&mut self, pkey: &Address) {
    self.mint3 = *pkey;
  }
  pub fn set_mints(&mut self, mints: [&Address; 4]) {
    self.mint0 = *mints[0];
    self.mint1 = *mints[1];
    self.mint2 = *mints[2];
    self.mint3 = *mints[3];
  }
  pub fn set_vault(&mut self, pkey: &Address) {
    self.vault = *pkey;
  }
  pub fn set_prog_owner(&mut self, pkey: &Address) {
    self.prog_owner = *pkey;
  }
  pub fn set_admin(&mut self, pkey: &Address) {
    self.admin = *pkey;
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
  maker: Address, //32; PDA needs at least 1 Address to keep PDA addresses from being exhausted by all users using u64. This also gives each user his own Escrow id.
  //taker: Address,   //32 hidden from maker
  mint_x: Address,   //32
  mint_y: Address,   //32
  amount_x: [u8; 8], //8 the offered amount from maker. This field gives taker easier way to view
  amount_y: [u8; 8], //8 the wanted amount to maker. The token_y price in mint_x = this Escrow PDA ATA_X amount / amount_y
  id: [u8; 8],       //8
  decimal_x: u8,     //1
  decimal_y: u8,     //1
  bump: u8,          //1
}
impl Escrow {
  pub const LEN: usize = core::mem::size_of::<Escrow>();
  //pub const LEN: usize = 32 + 32 + 32 + 8 +8+ 1;

  pub const SEED: &[u8] = b"escrow";

  pub fn maker(&self) -> &Address {
    &self.maker
  }
  pub fn mint_x(&self) -> &Address {
    &self.mint_x
  }
  pub fn mint_y(&self) -> &Address {
    &self.mint_y
  }
  pub fn id(&self) -> u64 {
    u64::from_le_bytes(self.id)
  }
  pub fn amount_x(&self) -> u64 {
    u64::from_le_bytes(self.amount_x)
  }
  pub fn amount_y(&self) -> u64 {
    u64::from_le_bytes(self.amount_y)
  }
  pub fn decimal_x(&self) -> u8 {
    self.decimal_x
  }
  pub fn decimal_y(&self) -> u8 {
    self.decimal_y
  }
  pub fn bump(&self) -> u8 {
    self.bump
  }
  pub fn set_maker(&mut self, pkey: &Address) {
    self.maker = *pkey;
  }
  pub fn set_mint_x(&mut self, pkey: &Address) {
    self.mint_x = *pkey;
  }
  pub fn set_mint_y(&mut self, pkey: &Address) {
    self.mint_y = *pkey;
  }
  pub fn set_id(&mut self, amt: u64) -> ProgramResult {
    self.id = amt.to_le_bytes();
    Ok(())
  }
  pub fn set_amount_x(&mut self, amt: u64) -> ProgramResult {
    none_zero_u64(amt)?;
    self.amount_x = amt.to_le_bytes();
    Ok(())
  }
  pub fn set_amount_y(&mut self, amt: u64) -> ProgramResult {
    none_zero_u64(amt)?;
    self.amount_y = amt.to_le_bytes();
    Ok(())
  }
  pub fn set_decimal_x(&mut self, amt: u8) {
    self.decimal_x = amt;
  }
  pub fn set_decimal_y(&mut self, amt: u8) {
    self.decimal_y = amt;
  }
  pub fn set_bump(&mut self, amt: u8) {
    self.bump = amt;
  }
  pub fn check(pda: &AccountInfo) -> ProgramResult {
    if pda.data_len() != Self::LEN {
      return Ee::EscrowDataLengh.e();
    }
    if pda.owner() != &ID {
      return Ee::EscrowIsForeign.e();
    }
    Ok(())
  }
  pub fn from_account_info(pda: &AccountInfo) -> Result<&mut Self, ProgramResult> {
    Self::check(pda)?;
    unsafe { Ok(&mut *(pda.borrow_mut_data_unchecked().as_ptr() as *mut Self)) }
  }
  pub unsafe fn from_bytes_unchecked(bytes: &[u8]) -> &Self {
    &*(bytes.as_ptr() as *const &&Escrow)
  }
  #[inline]
  pub unsafe fn from_account_info_unchecked(pda: &AccountInfo) -> Result<&Self, ProgramResult> {
    Self::check(pda)?;
    Ok(Self::from_bytes_unchecked(pda.borrow_data_unchecked()))
    //unsafe { Ok(&mut *(pda.borrow_mut_data_unchecked().as_ptr() as *mut Self)) }
  }
}
