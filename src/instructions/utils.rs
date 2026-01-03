//use num_derive::FromPrimitive;
use pinocchio::{
  account_info::AccountInfo,
  program_error::{ProgramError, ToStr},
  pubkey::{find_program_address, try_find_program_address, Pubkey},
  sysvars::{clock::Clock, rent::Rent, Sysvar},
};
use pinocchio_log::log;
use pinocchio_token_2022::state::{Mint as Mint22, TokenAccount as TokenAccount22};
use thiserror::Error;

use crate::Status;

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
  #[error("TokenProgram")]
  TokenProgram,
  #[error("SystemProgram")]
  SystemProgram,
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
  #[error("InputDataUnderMin")]
  InputDataUnderMin,
  #[error("InputDataOverMax")]
  InputDataOverMax,
  #[error("PdaNotInitialized")]
  PdaNotInitialized,
  #[error("Parse u64")]
  ParseU64,
  #[error("Tok22AcctDisciminatorOffset")]
  Tok22AcctDisciminatorOffset,
  #[error("InputU8InvalidForStatus")]
  InputU8InvalidForStatus,
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
  #[error("InputStatus")]
  InputStatus,
  #[error("MathOverflow")]
  MathOverflow,
  #[error("MathUnderflow")]
  MathUnderflow,
  #[error("NotRentExamptMint22")]
  NotRentExamptMint22,
  #[error("NotRentExamptTokAcct22")]
  NotRentExamptTokAcct22,
  #[error("NotRentExamptPDA")]
  NotRentExamptPDA,
  #[error("MintOrMintAuthority")]
  MintOrMintAuthority,
  #[error("MintOrTokenProgram")]
  MintOrTokenProgram,
  #[error("ErrorValue")]
  ErrorValue,
  #[error("PdaAuthority")]
  PdaAuthority,
  #[error("InsufficientFundNominal")]
  InsufficientFundNominal,
  #[error("ToWallet")]
  ToWallet,
  #[error("PdaDataLen")]
  PdaDataLen,
  #[error("ByteSliceSize32")]
  ByteSliceSize32,
  #[error("ByteSliceSize10")]
  ByteSliceSize10,
  #[error("ByteSliceSize6")]
  ByteSliceSize6,
  #[error("AtokenGPvbd")]
  AtokenGPvbd,
  #[error("ClockGet")]
  ClockGet,
  #[error("OnlyOwner")]
  OnlyOwner,
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
      9 => Ok(MyError::TokenProgram),
      10 => Ok(MyError::SystemProgram),
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
      21 => Ok(MyError::InputDataUnderMin),
      22 => Ok(MyError::InputDataOverMax),
      23 => Ok(MyError::PdaNotInitialized),
      24 => Ok(MyError::ParseU64),
      25 => Ok(MyError::Tok22AcctDisciminatorOffset),
      26 => Ok(MyError::InputU8InvalidForStatus),
      27 => Ok(MyError::InputU8InvalidForBool),
      28 => Ok(MyError::U64ByteSizeInvalid),
      29 => Ok(MyError::U32ByteSizeInvalid),
      30 => Ok(MyError::U16ByteSizeInvalid),
      31 => Ok(MyError::U8ByteSizeInvalid),
      32 => Ok(MyError::VaultPDA),
      33 => Ok(MyError::ConfigDataLengh),
      34 => Ok(MyError::FunctionSelector),
      35 => Ok(MyError::ConfigPDA),
      36 => Ok(MyError::InputStatus),
      37 => Ok(MyError::MathOverflow),
      38 => Ok(MyError::MathUnderflow),
      39 => Ok(MyError::NotRentExamptMint22),
      40 => Ok(MyError::NotRentExamptTokAcct22),
      41 => Ok(MyError::NotRentExamptPDA),
      42 => Ok(MyError::MintOrMintAuthority),
      43 => Ok(MyError::MintOrTokenProgram),
      44 => Ok(MyError::PdaAuthority),
      45 => Ok(MyError::InsufficientFundNominal),
      46 => Ok(MyError::ToWallet),
      47 => Ok(MyError::PdaDataLen),
      48 => Ok(MyError::ByteSliceSize32),
      49 => Ok(MyError::ByteSliceSize10),
      50 => Ok(MyError::ByteSliceSize6),
      51 => Ok(MyError::AtokenGPvbd),
      52 => Ok(MyError::ClockGet),
      53 => Ok(MyError::OnlyOwner),
      _ => Err(MyError::ErrorValue.into()),
    }
  }
}
//Human Readable Errors
impl ToStr for MyError {
  fn to_str<E>(&self) -> &'static str {
    match self {
      MyError::ErrorValue => "ErrorValue",
      MyError::InvalidDiscriminator => "InvalidDiscriminator",
      MyError::NotSigner => "NotSigner",
      MyError::NotWritable => "NotWritable",
      MyError::NotExecutable => "NotExecutable",
      MyError::ZeroAsInput => "ZeroAsInput",
      MyError::DecimalsValue => "DecimalsValue",
      MyError::MintDataLen => "MintDataLen",
      MyError::TokAcctDataLen => "TokAcctDataLen",
      MyError::Tok22AcctDataLen => "Tok22AcctDataLen",
      MyError::TokenProgram => "TokenProgram",
      MyError::SystemProgram => "SystemProgram",
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
      MyError::InputDataUnderMin => "InputDataUnderMin",
      MyError::InputDataOverMax => "InputDataOverMax",
      MyError::PdaNotInitialized => "PdaNotInitialized",
      MyError::ParseU64 => "ParseU64",
      MyError::Tok22AcctDisciminatorOffset => "Tok22AcctDisciminatorOffset",
      MyError::InputU8InvalidForStatus => "InputU8InvalidForStatus",
      MyError::InputU8InvalidForBool => "InputU8InvalidForBool",
      MyError::U64ByteSizeInvalid => "U64ByteSizeInvalid",
      MyError::U32ByteSizeInvalid => "U32ByteSizeInvalid",
      MyError::U16ByteSizeInvalid => "U16ByteSizeInvalid",
      MyError::U8ByteSizeInvalid => "U8ByteSizeInvalid",
      MyError::VaultPDA => "VaultPDA",
      MyError::ConfigDataLengh => "ConfigDataLengh",
      MyError::FunctionSelector => "FunctionSelector",
      MyError::ConfigPDA => "ConfigPDA",
      MyError::InputStatus => "InputStatus",
      MyError::MathOverflow => "MathOverflow",
      MyError::MathUnderflow => "MathUnderflow",
      MyError::NotRentExamptMint22 => "NotRentExamptMint22",
      MyError::NotRentExamptTokAcct22 => "NotRentExamptTokAcct22",
      MyError::NotRentExamptPDA => "NotRentExamptPDA",
      MyError::MintOrMintAuthority => "MintOrMintAuthority",
      MyError::MintOrTokenProgram => "MintOrTokenProgram",
      MyError::PdaAuthority => "PdaAuthority",
      MyError::InsufficientFundNominal => "InsufficientFundNominal",
      MyError::ToWallet => "ToWallet",
      MyError::PdaDataLen => "PdaDataLen",
      MyError::ByteSliceSize32 => "ByteSliceSize32",
      MyError::ByteSliceSize10 => "ByteSliceSize10",
      MyError::ByteSliceSize6 => "ByteSliceSize6",
      MyError::AtokenGPvbd => "AtokenGPvbd",
      MyError::ClockGet => "ClockGet",
      MyError::OnlyOwner => "OnlyOwner",
    }
  }
}

//----------------== Account Verification Functions
pub fn check_signer(account: &AccountInfo) -> Result<(), ProgramError> {
  if !account.is_signer() {
    return Err(MyError::NotSigner.into());
  }
  Ok(())
}
pub fn check_mint0a(mint: &AccountInfo, token_program: &AccountInfo) -> Result<(), ProgramError> {
  //if !mint.is_owned_by(mint_authority)
  if mint.data_len() != pinocchio_token::state::Mint::LEN {
    return Err(MyError::MintDataLen.into());
  }
  if !token_program.key().eq(&pinocchio_token::ID) {
    return Err(MyError::TokenProgram.into());
  }
  if mint.owner() != &pinocchio_token::ID {
    return Err(MyError::MintOrTokenProgram.into());
  }
  Ok(())
}

pub fn check_mint0b(
  mint: &AccountInfo,
  mint_authority: &AccountInfo,
  token_program: &AccountInfo,
  decimals: u8,
) -> Result<(), ProgramError> {
  let mint_info = pinocchio_token::state::Mint::from_account_info(mint)?;
  if mint_info
    .mint_authority()
    .is_some_and(|authority| !mint_authority.key().eq(authority))
  {
    return Err(MyError::MintOrMintAuthority.into());
  }
  if decimals != mint_info.decimals() {
    return Err(MyError::DecimalsValue.into());
  }
  check_mint0a(mint, token_program)?;
  //TODO: over mint supply?
  Ok(())
}

pub fn check_mint22a(mint: &AccountInfo, token_program: &AccountInfo) -> Result<(), ProgramError> {
  //if !mint.is_owned_by(mint_authority)
  if mint.data_len() != pinocchio_token_2022::state::Mint::BASE_LEN {
    return Err(MyError::MintDataLen.into());
  }
  if !token_program.key().eq(&pinocchio_token_2022::ID) {
    return Err(MyError::TokenProgram.into());
  }
  if mint.owner() != &pinocchio_token_2022::ID {
    return Err(MyError::MintOrTokenProgram.into());
  }
  Ok(())
}
pub fn check_mint22b(
  mint: &AccountInfo,
  mint_authority: &AccountInfo,
  token_program: &AccountInfo,
  decimals: u8,
) -> Result<(), ProgramError> {
  let mint_info = pinocchio_token_2022::state::Mint::from_account_info(mint)?;

  if mint_info
    .mint_authority()
    .is_some_and(|authority| !mint_authority.key().eq(authority))
  {
    return Err(MyError::MintOrMintAuthority.into());
  }
  if decimals != mint_info.decimals() {
    return Err(MyError::DecimalsValue.into());
  }
  check_mint22a(mint, token_program)?;
  //TODO: over mint supply?
  Ok(())
}

pub fn check_ata(
  ata: &AccountInfo,
  owner: &AccountInfo,
  mint: &AccountInfo,
) -> Result<(), ProgramError> {
  if ata
    .data_len()
    .ne(&pinocchio_token::state::TokenAccount::LEN)
  {
    return Err(MyError::TokAcctDataLen.into());
  }
  let ata_info = pinocchio_token::state::TokenAccount::from_account_info(ata)?;
  if !ata_info.owner().eq(owner.key()) {
    return Err(MyError::AtaOrOwner.into());
  }
  if !ata_info.mint().eq(mint.key()) {
    return Err(MyError::AtaOrMint.into());
  }
  Ok(())
}
pub fn check_ata22(
  ata: &AccountInfo,
  owner: &AccountInfo,
  mint: &AccountInfo,
) -> Result<(), ProgramError> {
  // token2022 ata has first 165 bytes the same as the legacy ata, but then some more data //log!("ata22 len:{}", ata.data_len());
  let ata_info = TokenAccount22::from_account_info(ata)?;
  if !ata_info.owner().eq(owner.key()) {
    return Err(MyError::AtaOrOwner.into());
  }
  if !ata_info.mint().eq(mint.key()) {
    return Err(MyError::AtaOrMint.into());
  }
  Ok(())
}
pub fn check_ata_x(
  authority: &AccountInfo,
  token_program: &AccountInfo,
  mint: &AccountInfo,
  ata: &AccountInfo,
) -> Result<(), ProgramError> {
  if find_program_address(
    &[authority.key(), token_program.key(), mint.key()],
    &pinocchio_associated_token_account::ID,
  )
  .0
  .ne(ata.key())
  {
    return Err(MyError::AtaCheckFailed.into());
  }
  Ok(())
}
pub fn check_pda(account: &AccountInfo) -> Result<(), ProgramError> {
  if account.lamports() == 0 {
    return Err(MyError::PdaNotInitialized.into());
  }
  if !account.is_owned_by(&crate::ID) {
    return Err(MyError::ForeignPDA.into());
  }
  Ok(())
}
pub fn check_sysprog(system_program: &AccountInfo) -> Result<(), ProgramError> {
  if !system_program.key().eq(&pinocchio_system::ID) {
    return Err(MyError::SystemProgram.into());
  }
  Ok(())
}
pub const ATOKENGPVBD: pinocchio_pubkey::reexport::Pubkey = pinocchio_system::ID; //[0, 0];
pub fn check_atoken_gpvbd(atoken_program: &AccountInfo) -> Result<(), ProgramError> {
  if !atoken_program.key().eq(&ATOKENGPVBD) {
    return Err(MyError::AtokenGPvbd.into());
  }
  Ok(())
}

//----------------== Check Account Properties
pub fn writable(account: &AccountInfo) -> Result<(), ProgramError> {
  if !account.is_writable() {
    return Err(MyError::NotWritable.into());
  }
  Ok(())
}
pub fn executable(account: &AccountInfo) -> Result<(), ProgramError> {
  if !account.executable() {
    return Err(MyError::NotExecutable.into());
  }
  Ok(())
}
//TODO: Mint and ATA from TokenLgc works. For mint and ATA from Token2022?
/// acc_type: 0 Mint, 1 TokenAccount
pub fn rent_exempt(account: &AccountInfo, acc_type: u8) -> Result<(), ProgramError> {
  if acc_type == 0 && account.lamports() < Rent::get()?.minimum_balance(Mint22::BASE_LEN) {
    return Err(MyError::NotRentExamptMint22.into());
  }
  if acc_type == 1 && account.lamports() < Rent::get()?.minimum_balance(TokenAccount22::BASE_LEN) {
    return Err(MyError::NotRentExamptTokAcct22.into());
  }
  if acc_type > 1 {
    return Err(MyError::AcctType.into());
  }
  Ok(())
}

pub fn empty_lamport(account: &AccountInfo) -> Result<(), ProgramError> {
  if account.lamports() == 0 {
    return Ok(());
  }
  Err(ProgramError::AccountAlreadyInitialized)
}
pub fn empty_data(account: &AccountInfo) -> Result<(), ProgramError> {
  if account.data_len() == 0 {
    return Ok(());
  }
  Err(MyError::EmptyData.into())
}

//----------------== Check Input Values
pub fn min_data_len(data: &[u8], min: usize) -> Result<(), ProgramError> {
  if data.len() < min {
    return Err(MyError::InputDataUnderMin.into());
  }
  Ok(())
}
pub fn max_data_len(data: &[u8], max: usize) -> Result<(), ProgramError> {
  if data.len() > max {
    return Err(MyError::InputDataOverMax.into());
  }
  Ok(())
}

pub fn check_decimals(mint: &AccountInfo, decimals: u8) -> Result<(), ProgramError> {
  let mint_info = pinocchio_token::state::Mint::from_account_info(mint)?;
  if decimals != mint_info.decimals() {
    return Err(MyError::DecimalsValue.into());
  }
  Ok(())
}
pub fn check_decimals_max(decimals: u8, max: u8) -> Result<(), ProgramError> {
  if decimals > max {
    return Err(MyError::DecimalsValue.into());
  }
  Ok(())
}
pub fn check_str_len(s: &str, min_len: usize, max_len: usize) -> Result<(), ProgramError> {
  if s.len() < min_len {
    return Err(MyError::StrOverMax.into());
  }
  if s.len() > max_len {
    return Err(MyError::StrUnderMin.into());
  }
  Ok(())
}

//----------------== Parse Functions
/// Parse a u64 from u8 array
pub fn parse_u64(data: &[u8]) -> Result<u64, ProgramError> {
  let bytes: [u8; 8] = data
    .try_into()
    .or_else(|_e| Err(MyError::U64ByteSizeInvalid))?;

  let amt = u64::from_le_bytes(bytes);
  // let amount = u64::from_le_bytes([data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7]]);
  if amt == 0 {
    return Err(MyError::ZeroAsInput.into());
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
pub fn to32bytes(byte_slice: &[u8]) -> Result<&[u8; 32], ProgramError> {
  let bytes: &[u8; 32] = byte_slice
    .try_into()
    .map_err(|_| MyError::ByteSliceSize32)?;
  //let mut str_u8array = [0u8; 32];
  //str_u8array.copy_from_slice(&data[10..42]);
  return Ok(bytes);
}
pub fn to10bytes(byte_slice: &[u8]) -> Result<&[u8; 10], ProgramError> {
  let bytes: &[u8; 10] = byte_slice
    .try_into()
    .map_err(|_| MyError::ByteSliceSize10)?;
  return Ok(bytes);
}
pub fn to6bytes(byte_slice: &[u8]) -> Result<&[u8; 6], ProgramError> {
  let bytes: &[u8; 6] = byte_slice.try_into().map_err(|_| MyError::ByteSliceSize6)?;
  return Ok(bytes);
}
pub fn u8_to_bool(v: u8) -> Result<bool, ProgramError> {
  match v {
    0 => Ok(false),
    1 => Ok(true),
    _ => Err(MyError::InputU8InvalidForBool.into()),
  }
}
pub fn u8_to_status(v: u8) -> Result<Status, ProgramError> {
  match v {
    0 => Ok(Status::Waiting),
    1 => Ok(Status::Active),
    2 => Ok(Status::Expired),
    3 => Ok(Status::Paused),
    4 => Ok(Status::Canceled),
    _ => Err(MyError::InputU8InvalidForStatus.into()),
  }
}

//----------------== Derive Functions
pub fn derive_pda1(user: &AccountInfo, bstr: &[u8]) -> Result<(Pubkey, u8), ProgramError> {
  log!("derive_pda1");
  //find_program_address(&[b"vault", user.key().as_ref()], &crate::ID)
  // let (pda, _bump) =
  try_find_program_address(&[bstr, user.key().as_ref()], &crate::ID)
    .ok_or_else(|| ProgramError::InvalidSeeds)
}
/*let pda = pubkey::create_program_address(
    &[PDA_SEED, &[self.datas.bump as u8]],
    &crate::ID,
) */

//----------------== Token 2022 Interface
/// [4 (extension discriminator) + 32 (update_authority) + 32 (metadata)]
pub const METADATA_POINTER_SIZE: usize = 4 + 32 + 32;
/// [4 (extension discriminator) + 32 (update_authority) + 32 (mint) + 4 (size of name ) + 4 (size of symbol) + 4 (size of uri) + 4 (size of additional_metadata)]
pub const METADATA_EXTENSION_BASE_SIZE: usize = 4 + 32 + 32 + 4 + 4 + 4 + 4;
/// Padding used so that Mint and Account extensions start at the same index
pub const EXTENSIONS_PADDING_AND_OFFSET: usize = 84;
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
pub fn get_time() -> Result<u32, ProgramError> {
  let clock = Clock::get().map_err(|_| MyError::ClockGet)?;
  let time = clock.unix_timestamp as u32;
  log!("Solana time: {}", time);
  Ok(time)
}
