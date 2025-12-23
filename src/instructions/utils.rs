//use num_derive::FromPrimitive;
use pinocchio::{
  account_info::AccountInfo,
  program_error::{ProgramError, ToStr},
  pubkey::{try_find_program_address, Pubkey},
};
use thiserror::Error;

//TODO: put errors in error.rs ... https://learn.blueshift.gg/en/courses/pinocchio-for-dummies/pinocchio-errors
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
  #[error("PdaNotInitialized")]
  PdaNotInitialized,
  #[error("Parse u64")]
  ParseU64,
  #[error("MintOrTokenProgram")]
  MintOrTokenProgram,
  #[error("Tok22AcctDisciminatorOffset")]
  Tok22AcctDisciminatorOffset,
  #[error("InputDataLengthOverMax")]
  InputDataLengthOverMax,
  #[error("InputStrSliceOverMax")]
  InputStrSliceOverMax,
  #[error("InputU8InvalidForBool")]
  InputU8InvalidForBool,
  #[error("U64ByteSizeInvalid")]
  U64ByteSizeInvalid,
  #[error("U32ByteSizeInvalid")]
  U32ByteSizeInvalid,
  #[error("U16ByteSizeInvalid")]
  U16ByteSizeInvalid,
  #[error("U8ByteSizeInvalid")]
  U8ByteSizeInvalid,
  #[error("VaultPDA")]
  VaultPDA,
  #[error("ConfigDataLengh")]
  ConfigDataLengh,
  #[error("FunctionSelector")]
  FunctionSelector,
  #[error("ConfigPDA")]
  ConfigPDA,
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
      22 => Ok(MyError::PdaNotInitialized),
      23 => Ok(MyError::ParseU64),
      24 => Ok(MyError::MintOrTokenProgram),
      25 => Ok(MyError::Tok22AcctDisciminatorOffset),
      26 => Ok(MyError::InputDataLengthOverMax),
      27 => Ok(MyError::InputStrSliceOverMax),
      28 => Ok(MyError::InputU8InvalidForBool),
      29 => Ok(MyError::U64ByteSizeInvalid),
      30 => Ok(MyError::U32ByteSizeInvalid),
      31 => Ok(MyError::U16ByteSizeInvalid),
      32 => Ok(MyError::U8ByteSizeInvalid),
      33 => Ok(MyError::VaultPDA),
      34 => Ok(MyError::ConfigDataLengh),
      35 => Ok(MyError::FunctionSelector),
      36 => Ok(MyError::ConfigPDA),
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
      MyError::PdaNotInitialized => "PdaNotInitialized",
      MyError::ParseU64 => "ParseU64",
      MyError::MintOrTokenProgram => "MintOrTokenProgram",
      MyError::Tok22AcctDisciminatorOffset => "Tok22AcctDisciminatorOffset",
      MyError::InputDataLengthOverMax => "InputDataLengthOverMax",
      MyError::InputStrSliceOverMax => "InputStrSliceOverMax",
      MyError::InputU8InvalidForBool => "InputU8InvalidForBool",
      MyError::U64ByteSizeInvalid => "U64ByteSizeInvalid",
      MyError::U32ByteSizeInvalid => "U32ByteSizeInvalid",
      MyError::U16ByteSizeInvalid => "U16ByteSizeInvalid",
      MyError::U8ByteSizeInvalid => "U8ByteSizeInvalid",
      MyError::VaultPDA => "VaultPDA",
      MyError::ConfigDataLengh => "ConfigDataLengh",
      MyError::FunctionSelector => "FunctionSelector",
      MyError::ConfigPDA => "ConfigPDA",
    }
  }
}
//----------------==
//----------------==
/// Parse a u64 from u8 array
pub fn parse_u64(data: &[u8]) -> Result<u64, ProgramError> {
  let bytes: [u8; 8] = data
    .try_into()
    .or_else(|_e| Err(MyError::U64ByteSizeInvalid))?;

  let amt = u64::from_le_bytes(bytes);
  // let amount = u64::from_le_bytes([data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7]]);
  if amt == 0 {
    return Err(ProgramError::InvalidArgument);
  }
  Ok(amt)
}
pub fn parse_u32(data: &[u8]) -> Result<u32, ProgramError> {
  let bytes: [u8; 4] = data
    .try_into()
    .or_else(|_e| Err(MyError::U32ByteSizeInvalid))?;

  let amt = u32::from_le_bytes(bytes);
  // let amount = u64::from_le_bytes([data[0], data[1], data[2], data[3]]);
  if amt == 0 {
    return Err(MyError::ZeroAsInput.into());
  }
  Ok(amt)
}
pub fn u8_slice_to_array(str_u8: &[u8]) -> Result<&[u8; 32], ProgramError> {
  let str_u8array: &[u8; 32] = str_u8
    .try_into()
    .map_err(|_| MyError::InputStrSliceOverMax)?;
  return Ok(str_u8array);
}
pub fn u8_to_bool(v: u8) -> Result<bool, ProgramError> {
  match v {
    0 => Ok(false),
    1 => Ok(true),
    _ => Err(MyError::InputU8InvalidForBool.into()),
  }
}
//----------------==
pub fn derive_pda1(user: &AccountInfo, bstr: &[u8]) -> Result<(Pubkey, u8), ProgramError> {
  //find_program_address(&[b"vault", user.key().as_ref()], &crate::ID)
  // let (pda, _bump) =
  try_find_program_address(&[bstr, user.key().as_ref()], &crate::ID)
    .ok_or_else(|| ProgramError::InvalidSeeds)
}
/*let pda = pubkey::create_program_address(
    &[PDA_SEED, &[self.datas.bump as u8]],
    &crate::ID,
) */
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
