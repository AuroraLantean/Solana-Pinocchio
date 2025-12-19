//use num_derive::FromPrimitive;
use pinocchio::{
  account_info::AccountInfo,
  program_error::{ProgramError, ToStr},
};
use thiserror::Error;

//https://learn.blueshift.gg/en/courses/pinocchio-for-dummies/pinocchio-errors
#[derive(Clone, Debug, Eq, Error, PartialEq)] //FromPrimitive
pub enum MyError {
  #[error("InvalidDiscriminator")]
  InvalidDiscriminator,
  #[error("NotSigner")]
  NotSigner,
  #[error("NotWritable")]
  NotWritable,
  #[error("NotExecutable")]
  NotExecutable,
  #[error("ZeroAsInput")]
  ZeroAsInput,
  #[error("DecimalsValue")]
  DecimalsValue,
  #[error("MintDataLen")]
  MintDataLen,
  #[error("TokAcctDataLen")]
  TokAcctDataLen,
  #[error("Tok22AcctDataLen")]
  Tok22AcctDataLen,
  #[error("TokenProgramInvalid")]
  TokenProgramInvalid,
  #[error("SystemProgramInvalid")]
  SystemProgramInvalid,
  #[error("AtaOrOwner")]
  AtaOrOwner,
  #[error("AtaOrMint")]
  AtaOrMint,
  #[error("AtaCheckFailed")]
  AtaCheckFailed,
  #[error("AtaOwnerInvalid")]
  AtaOwnerInvalid,
  #[error("ForeignPDA")]
  ForeignPDA,
  #[error("EmptyLamport")]
  EmptyLamport,
  #[error("EmptyData")]
  EmptyData,
  #[error("AcctType")]
  AcctType,
  #[error("StrOverMax")]
  StrOverMax,
  #[error("StrUnderMin")]
  StrUnderMin,
  #[error("InputDataLen")]
  InputDataLen,
  #[error("PdaNoInitialized")]
  PdaNoInitialized,
  #[error("Parse u64")]
  ParseU64,
  #[error("MintOrTokenProgram")]
  MintOrTokenProgram,
  #[error("Tok22AcctDisciminatorOffset")]
  Tok22AcctDisciminatorOffset,
}
impl From<MyError> for ProgramError {
  fn from(e: MyError) -> Self {
    ProgramError::Custom(e as u32)
  }
}

//Deserialize Errors from Raw Values
impl TryFrom<u32> for MyError {
  type Error = ProgramError;
  fn try_from(error: u32) -> Result<Self, Self::Error> {
    match error {
      0 => Ok(MyError::InvalidDiscriminator),
      1 => Ok(MyError::NotSigner),
      2 => Ok(MyError::NotWritable),
      3 => Ok(MyError::NotExecutable),
      4 => Ok(MyError::ZeroAsInput),
      5 => Ok(MyError::DecimalsValue),
      6 => Ok(MyError::MintDataLen),
      7 => Ok(MyError::TokAcctDataLen),
      8 => Ok(MyError::Tok22AcctDataLen),
      9 => Ok(MyError::TokenProgramInvalid),
      10 => Ok(MyError::SystemProgramInvalid),
      11 => Ok(MyError::AtaOrOwner),
      12 => Ok(MyError::AtaOrMint),
      13 => Ok(MyError::AtaCheckFailed),
      14 => Ok(MyError::AtaOwnerInvalid),
      15 => Ok(MyError::ForeignPDA),
      16 => Ok(MyError::EmptyLamport),
      17 => Ok(MyError::EmptyData),
      18 => Ok(MyError::AcctType),
      19 => Ok(MyError::StrOverMax),
      20 => Ok(MyError::StrUnderMin),
      21 => Ok(MyError::InputDataLen),
      22 => Ok(MyError::PdaNoInitialized),
      23 => Ok(MyError::ParseU64),
      24 => Ok(MyError::MintOrTokenProgram),
      25 => Ok(MyError::Tok22AcctDisciminatorOffset),
      _ => Err(ProgramError::InvalidArgument),
    }
  }
}
//Human Readable Errors
impl ToStr for MyError {
  fn to_str<E>(&self) -> &'static str {
    match self {
      MyError::InvalidDiscriminator => "InvalidDiscriminator",
      MyError::NotSigner => "NotSigner",
      MyError::NotWritable => "NotWritable",
      MyError::NotExecutable => "NotExecutable",
      MyError::ZeroAsInput => "ZeroAsInput",
      MyError::DecimalsValue => "DecimalsValue",
      MyError::MintDataLen => "MintDataLen",
      MyError::TokAcctDataLen => "TokAcctDataLen",
      MyError::Tok22AcctDataLen => "Tok22AcctDataLen",
      MyError::TokenProgramInvalid => "TokenProgramInvalid",
      MyError::SystemProgramInvalid => "SystemProgramInvalid",
      MyError::AtaOrOwner => "AtaOrOwner",
      MyError::AtaOrMint => "AtaOrMint",
      MyError::AtaCheckFailed => "AtaCheckFailed",
      MyError::AtaOwnerInvalid => "AtaOwnerInvalid",
      MyError::ForeignPDA => "ForeignPDA",
      MyError::EmptyLamport => "EmptyLamport",
      MyError::EmptyData => "EmptyData",
      MyError::AcctType => "AcctType",
      MyError::StrOverMax => "StrOverMax",
      MyError::StrUnderMin => "StrUnderMin",
      MyError::InputDataLen => "InputDataLen",
      MyError::PdaNoInitialized => "PdaNoInitialized",
      MyError::ParseU64 => "ParseU64",
      MyError::MintOrTokenProgram => "MintOrTokenProgram",
      MyError::Tok22AcctDisciminatorOffset => "Tok22AcctDisciminatorOffset",
    }
  }
}
//----------------==
//----------------==
/// Parse a u64 from instruction data.
/// amount must be non-zero,
pub fn parse_u64(data: &[u8]) -> Result<u64, ProgramError> {
  let bytes: [u8; 8] = data
    .try_into()
    .or_else(|_e| Err(ProgramError::Custom(501)))?;
  //TODO: fix Custom 501
  // Convert the byte slice to a u64
  let amt = u64::from_le_bytes(bytes);
  // let amount = u64::from_le_bytes([data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7]]);

  // Validate the amount (e.g., not zero)
  if amt == 0 {
    return Err(ProgramError::InvalidArgument);
  }
  Ok(amt)
}
pub fn parse_u32(data: &[u8]) -> Result<u32, ProgramError> {
  let bytes: [u8; 4] = data
    .try_into()
    .or_else(|_e| Err(ProgramError::Custom(502)))?;

  // Convert the byte slice to a u64
  let amt = u32::from_le_bytes(bytes);
  // let amount = u64::from_le_bytes([data[0], data[1], data[2], data[3]]);

  // Validate the amount (e.g., not zero)
  if amt == 0 {
    return Err(MyError::ZeroAsInput.into());
  }
  Ok(amt)
}
//----------------==
//----------------==
const TOKEN_2022_ACCOUNT_DISCRIMINATOR_OFFSET: usize = 165;
pub const TOKEN_2022_MINT_DISCRIMINATOR: u8 = 0x01;
pub const TOKEN_2022_TOKEN_ACCOUNT_DISCRIMINATOR: u8 = 0x02;

pub fn check_mint_interface(mint: &AccountInfo) -> Result<(), ProgramError> {
  if !mint.is_owned_by(&pinocchio_token_2022::ID) {
    //legacy token
    if !mint.is_owned_by(&pinocchio_token::ID) {
      return Err(MyError::MintOrTokenProgram.into());
    } else {
      if mint.data_len().ne(&pinocchio_token::state::Mint::LEN) {
        return Err(MyError::MintDataLen.into());
      }
    }
  } else {
    //Token2022
    let data = mint.try_borrow_data()?;

    if data.len().ne(&pinocchio_token::state::Mint::LEN) {
      if data.len().le(&TOKEN_2022_ACCOUNT_DISCRIMINATOR_OFFSET) {
        return Err(MyError::Tok22AcctDataLen.into());
      }
      if data[TOKEN_2022_ACCOUNT_DISCRIMINATOR_OFFSET].ne(&TOKEN_2022_MINT_DISCRIMINATOR) {
        return Err(MyError::Tok22AcctDisciminatorOffset.into());
      }
    }
  }
  Ok(())
}

pub fn check_tokacct_interface(ata: &AccountInfo) -> Result<(), ProgramError> {
  if !ata.is_owned_by(&pinocchio_token_2022::ID) {
    //Legacy ATA
    if !ata.is_owned_by(&pinocchio_token::ID) {
      return Err(MyError::AtaOwnerInvalid.into());
    } else {
      if ata
        .data_len()
        .ne(&pinocchio_token::state::TokenAccount::LEN)
      {
        return Err(MyError::TokAcctDataLen.into());
      }
    }
  } else {
    //Token2022 ATA
    let data = ata.try_borrow_data()?;

    if data.len().ne(&pinocchio_token::state::TokenAccount::LEN) {
      if data.len().le(&TOKEN_2022_ACCOUNT_DISCRIMINATOR_OFFSET) {
        return Err(MyError::Tok22AcctDataLen.into());
      }
      if data[TOKEN_2022_ACCOUNT_DISCRIMINATOR_OFFSET].ne(&TOKEN_2022_TOKEN_ACCOUNT_DISCRIMINATOR) {
        return Err(MyError::Tok22AcctDisciminatorOffset.into());
      }
    }
  }
  Ok(())
}
