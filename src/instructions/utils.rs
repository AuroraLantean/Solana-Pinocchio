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
pub enum Ee {
  #[error("InvalidDiscriminator")]
  InvalidDiscriminator,
  #[error("Xyz001")]
  Xyz001,
  #[error("OnlyProgOwner")]
  OnlyProgOwner,
  #[error("OnlyAdmin")]
  OnlyAdmin,
  #[error("OnlyUser")]
  OnlyUser,
  #[error("Xyz005")]
  Xyz005,
  #[error("Xyz006")]
  Xyz006,
  #[error("NotWritable")]
  NotWritable,
  #[error("NotExecutable")]
  NotExecutable,
  #[error("TokenProgram")]
  TokenProgram,
  #[error("AtokenGPvbd")]
  AtokenGPvbd,
  #[error("SystemProgram")]
  SystemProgram,
  #[error("InputDataUnderMin")]
  InputDataUnderMin,
  #[error("InputDataOverMax")]
  InputDataOverMax,
  #[error("Xyz014")]
  Xyz014,
  #[error("Xyz015")]
  Xyz015,
  #[error("Xyz016")]
  Xyz016,
  #[error("Xyz017")]
  Xyz017,
  #[error("Xyz018")]
  Xyz018,
  #[error("Xyz019")]
  Xyz019,
  //Bytes for Numerical
  #[error("ZeroAsU128")]
  ZeroAsU128,
  #[error("ZeroAsU64")]
  ZeroAsU64,
  #[error("ZeroAsU32")]
  ZeroAsU32,
  #[error("ZeroAsU16")]
  ZeroAsU16,
  #[error("ZeroAsU8")]
  ZeroAsU8,
  //Bytes Sizes
  #[error("ByteSizeForU128")]
  ByteSizeForU128,
  #[error("ByteSizeForU64")]
  ByteSizeForU64,
  #[error("ByteSizeForU32")]
  ByteSizeForU32,
  #[error("ByteSizeForU16")]
  ByteSizeForU16,
  #[error("ByteSizeForU8")]
  ByteSizeForU8,
  //Byte Slice Sizes
  #[error("ByteSliceSize128")]
  ByteSliceSize128,
  #[error("ByteSliceSize64")]
  ByteSliceSize64,
  #[error("ByteSliceSize32")]
  ByteSliceSize32,
  #[error("ByteSliceSize10")]
  ByteSliceSize10,
  #[error("ByteSliceSize6")]
  ByteSliceSize6,
  #[error("Xyz035")]
  Xyz035,
  #[error("Xyz036")]
  Xyz036,
  #[error("Xyz037")]
  Xyz037,
  #[error("Xyz038")]
  Xyz038,
  #[error("Xyz039")]
  Xyz039,
  //Inputs
  #[error("StrOverMax")]
  StrOverMax,
  #[error("StrUnderMin")]
  StrUnderMin,
  #[error("ArrayLength")]
  ArrayLength,
  #[error("ByteForStatus")]
  ByteForStatus,
  #[error("ByteForBool")]
  ByteForBool,
  #[error("Xyz045")]
  Xyz045,
  #[error("Xyz046")]
  Xyz046,
  #[error("Xyz047")]
  Xyz047,
  #[error("Xyz048")]
  Xyz048,
  #[error("Xyz049")]
  Xyz049,
  //PDA
  #[error("PdaNoLamport")]
  PdaNoLamport,
  #[error("ForeignPDA")]
  ForeignPDA,
  #[error("ConfigPDA")]
  ConfigPDA,
  #[error("VaultPDA")]
  VaultPDA,
  #[error("AdminPDA")]
  AdminPDA,
  #[error("UserPDA")]
  UserPDA,
  #[error("EscrowPDA")]
  EscrowPDA,
  #[error("ActionPDA")]
  ActionPDA,
  #[error("Xyz058")]
  Xyz058,
  #[error("Xyz059")]
  Xyz059,
  //PDA Data Length
  #[error("ConfigDataLengh")]
  ConfigDataLengh,
  #[error("VaultDataLengh")]
  VaultDataLengh,
  #[error("AdminDataLengh")]
  AdminDataLengh,
  #[error("UserDataLengh")]
  UserDataLengh,
  #[error("EscrowDataLengh")]
  EscrowDataLengh,
  #[error("ActionDataLengh")]
  ActionDataLengh,
  #[error("Xyz066")]
  Xyz066,
  #[error("Xyz067")]
  Xyz067,
  #[error("Xyz068")]
  Xyz068,
  #[error("Xyz069")]
  Xyz069,
  //Mint Account
  #[error("DecimalsValue")]
  DecimalsValue,
  #[error("MintDataLen")]
  MintDataLen,
  #[error("MintOrMintAuthority")]
  MintOrMintAuthority,
  #[error("MintOrTokenProgram")]
  MintOrTokenProgram,
  #[error("Xyz074")]
  Xyz074,
  #[error("Xyz075")]
  Xyz075,
  #[error("Xyz076")]
  Xyz076,
  #[error("Xyz077")]
  Xyz077,
  #[error("Xyz078")]
  Xyz078,
  #[error("Xyz079")]
  Xyz079,
  //ATA
  #[error("AtaDataLen")]
  AtaDataLen,
  #[error("Ata22DataLen")]
  Ata22DataLen,
  #[error("AtaOrOwner")]
  AtaOrOwner,
  #[error("AtaOrMint")]
  AtaOrMint,
  #[error("AtaCheckX1")]
  AtaCheckX1,
  #[error("ForeignAta")]
  ForeignAta,
  #[error("Xyz086")]
  Xyz086,
  #[error("Xyz087")]
  Xyz087,
  #[error("Xyz088")]
  Xyz088,
  #[error("Xyz089")]
  Xyz089,
  //Token 2022
  #[error("Acct22Type")]
  Acct22Type,
  #[error("Tok22AcctDiscOffset")]
  Tok22AcctDiscOffset,
  #[error("NotRentExemptMint22")]
  NotRentExemptMint22,
  #[error("NotRentExemptTokAcct22")]
  NotRentExemptTokAcct22,
  #[error("Xyz094")]
  Xyz094,
  //Withdraw
  #[error("PdaToBeBelowRentExempt")]
  PdaToBeBelowRentExempt,
  #[error("ToWallet")]
  ToWallet,
  #[error("Xyz097")]
  Xyz097,
  #[error("Xyz098")]
  Xyz098,
  #[error("Xyz099")]
  Xyz099,
  //Math
  #[error("MathOverflow")]
  MathOverflow,
  #[error("MathUnderflow")]
  MathUnderflow,
  #[error("MultiplyOverflow")]
  MultiplyOverflow,
  #[error("DividedByZero")]
  DividedByZero,
  #[error("Remainder")]
  Remainder,
  //Misc...
  #[error("EmptyData")]
  EmptyData,
  #[error("FunctionSelector")]
  FunctionSelector,
  #[error("ClockGet")]
  ClockGet,
  #[error("ErrorNotMapped")]
  ErrorNotMapped,
  //ProgramError: AccountBorrowFailed
}
impl From<Ee> for ProgramError {
  fn from(e: Ee) -> Self {
    ProgramError::Custom(e as u32)
  }
} //TODO: fix tryFrom and to_str below
  //Deserialize Errors from Raw Values
impl TryFrom<u32> for Ee {
  type Error = ProgramError;
  fn try_from(error: u32) -> Result<Self, Self::Error> {
    match error {
      0 => Ok(Ee::InvalidDiscriminator),
      1 => Ok(Ee::Xyz001),
      2 => Ok(Ee::OnlyProgOwner),
      3 => Ok(Ee::OnlyAdmin),
      4 => Ok(Ee::OnlyUser),
      5 => Ok(Ee::Xyz005),
      6 => Ok(Ee::Xyz006),
      7 => Ok(Ee::NotWritable),
      8 => Ok(Ee::NotExecutable),
      9 => Ok(Ee::TokenProgram),
      10 => Ok(Ee::AtokenGPvbd),
      11 => Ok(Ee::SystemProgram),
      12 => Ok(Ee::InputDataUnderMin),
      13 => Ok(Ee::InputDataOverMax),
      14 => Ok(Ee::Xyz014),
      15 => Ok(Ee::Xyz015),
      16 => Ok(Ee::Xyz016),
      17 => Ok(Ee::Xyz017),
      18 => Ok(Ee::Xyz018),
      19 => Ok(Ee::Xyz019),
      20 => Ok(Ee::ZeroAsU128),
      21 => Ok(Ee::ZeroAsU64),
      22 => Ok(Ee::ZeroAsU32),
      23 => Ok(Ee::ZeroAsU16),
      24 => Ok(Ee::ZeroAsU8),
      25 => Ok(Ee::ByteSizeForU128),
      26 => Ok(Ee::ByteSizeForU64),
      27 => Ok(Ee::ByteSizeForU32),
      28 => Ok(Ee::ByteSizeForU16),
      29 => Ok(Ee::ByteSizeForU8),
      30 => Ok(Ee::ByteSliceSize128),
      31 => Ok(Ee::ByteSliceSize64),
      32 => Ok(Ee::ByteSliceSize32),
      33 => Ok(Ee::ByteSliceSize10),
      34 => Ok(Ee::ByteSliceSize6),
      35 => Ok(Ee::Xyz035),
      36 => Ok(Ee::Xyz036),
      37 => Ok(Ee::Xyz037),
      38 => Ok(Ee::Xyz038),
      39 => Ok(Ee::Xyz039),
      40 => Ok(Ee::StrOverMax),
      41 => Ok(Ee::StrUnderMin),
      42 => Ok(Ee::ArrayLength),
      43 => Ok(Ee::ByteForStatus),
      44 => Ok(Ee::ByteForBool),
      45 => Ok(Ee::Xyz045),
      46 => Ok(Ee::Xyz046),
      47 => Ok(Ee::Xyz047),
      48 => Ok(Ee::Xyz048),
      49 => Ok(Ee::Xyz049),
      50 => Ok(Ee::PdaNoLamport),
      51 => Ok(Ee::ForeignPDA),
      52 => Ok(Ee::ConfigPDA),
      53 => Ok(Ee::VaultPDA),
      54 => Ok(Ee::AdminPDA),
      55 => Ok(Ee::UserPDA),
      56 => Ok(Ee::EscrowPDA),
      57 => Ok(Ee::ActionPDA),
      58 => Ok(Ee::Xyz058),
      59 => Ok(Ee::Xyz059),
      60 => Ok(Ee::ConfigDataLengh),
      61 => Ok(Ee::VaultDataLengh),
      62 => Ok(Ee::AdminDataLengh),
      63 => Ok(Ee::UserDataLengh),
      64 => Ok(Ee::EscrowDataLengh),
      65 => Ok(Ee::ActionDataLengh),
      66 => Ok(Ee::Xyz066),
      67 => Ok(Ee::Xyz067),
      68 => Ok(Ee::Xyz068),
      69 => Ok(Ee::Xyz069),
      70 => Ok(Ee::DecimalsValue),
      71 => Ok(Ee::MintDataLen),
      72 => Ok(Ee::MintOrMintAuthority),
      73 => Ok(Ee::MintOrTokenProgram),
      74 => Ok(Ee::Xyz074),
      75 => Ok(Ee::Xyz075),
      76 => Ok(Ee::Xyz076),
      77 => Ok(Ee::Xyz077),
      78 => Ok(Ee::Xyz078),
      79 => Ok(Ee::Xyz079),
      80 => Ok(Ee::AtaDataLen),
      81 => Ok(Ee::Ata22DataLen),
      82 => Ok(Ee::AtaOrOwner),
      83 => Ok(Ee::AtaOrMint),
      84 => Ok(Ee::AtaCheckX1),
      85 => Ok(Ee::ForeignAta),
      86 => Ok(Ee::Xyz086),
      87 => Ok(Ee::Xyz087),
      88 => Ok(Ee::Xyz088),
      89 => Ok(Ee::Xyz089),
      90 => Ok(Ee::Acct22Type),
      91 => Ok(Ee::Tok22AcctDiscOffset),
      92 => Ok(Ee::NotRentExemptMint22),
      93 => Ok(Ee::NotRentExemptTokAcct22),
      94 => Ok(Ee::Xyz094),
      95 => Ok(Ee::PdaToBeBelowRentExempt),
      96 => Ok(Ee::ToWallet),
      97 => Ok(Ee::Xyz097),
      98 => Ok(Ee::Xyz098),
      99 => Ok(Ee::Xyz099),
      100 => Ok(Ee::MathOverflow),
      101 => Ok(Ee::MathUnderflow),
      102 => Ok(Ee::MultiplyOverflow),
      103 => Ok(Ee::DividedByZero),
      104 => Ok(Ee::Remainder),
      105 => Ok(Ee::EmptyData),
      106 => Ok(Ee::FunctionSelector),
      107 => Ok(Ee::ClockGet),
      _ => Err(Ee::ErrorNotMapped.into()),
    }
  }
}
//Human Readable Errors; TODO: arrange below
impl ToStr for Ee {
  fn to_str<E>(&self) -> &'static str {
    match self {
      Ee::InvalidDiscriminator => "InvalidDiscriminator",
      Ee::Xyz001 => "Xyz001",
      Ee::OnlyProgOwner => "OnlyProgOwner",
      Ee::OnlyAdmin => "OnlyAdmin",
      Ee::OnlyUser => "OnlyUser",
      Ee::Xyz005 => "Xyz005",
      Ee::Xyz006 => "Xyz006",

      Ee::NotWritable => "NotWritable",
      Ee::NotExecutable => "NotExecutable",
      Ee::TokenProgram => "TokenProgram",
      Ee::AtokenGPvbd => "AtokenGPvbd",
      Ee::SystemProgram => "SystemProgram",
      Ee::InputDataUnderMin => "InputDataUnderMin",
      Ee::InputDataOverMax => "InputDataOverMax",
      Ee::Xyz014 => "Xyz014",
      Ee::Xyz015 => "Xyz015",
      Ee::Xyz016 => "Xyz016",
      Ee::Xyz017 => "Xyz017",
      Ee::Xyz018 => "Xyz018",
      Ee::Xyz019 => "Xyz019",

      Ee::ZeroAsU128 => "ZeroAsU128",
      Ee::ZeroAsU64 => "ZeroAsU64",
      Ee::ZeroAsU32 => "ZeroAsU32",
      Ee::ZeroAsU16 => "ZeroAsU16",
      Ee::ZeroAsU8 => "ZeroAsU8",
      Ee::ByteSizeForU128 => "ByteSizeForU128",
      Ee::ByteSizeForU64 => "ByteSizeForU64",
      Ee::ByteSizeForU32 => "ByteSizeForU32",
      Ee::ByteSizeForU16 => "ByteSizeForU16",
      Ee::ByteSizeForU8 => "ByteSizeForU8",
      Ee::ByteSliceSize128 => "ByteSliceSize128",
      Ee::ByteSliceSize64 => "ByteSliceSize64",
      Ee::ByteSliceSize32 => "ByteSliceSize32",
      Ee::ByteSliceSize10 => "ByteSliceSize10",
      Ee::ByteSliceSize6 => "ByteSliceSize6",
      Ee::Xyz035 => "Xyz035",
      Ee::Xyz036 => "Xyz036",
      Ee::Xyz037 => "Xyz037",
      Ee::Xyz038 => "Xyz038",
      Ee::Xyz039 => "Xyz039",

      Ee::StrOverMax => "StrOverMax",
      Ee::StrUnderMin => "StrUnderMin",
      Ee::ArrayLength => "ArrayLength",
      Ee::ByteForStatus => "ByteForStatus",
      Ee::ByteForBool => "ByteForBool",
      Ee::Xyz045 => "Xyz045",
      Ee::Xyz046 => "Xyz046",
      Ee::Xyz047 => "Xyz047",
      Ee::Xyz048 => "Xyz048",
      Ee::Xyz049 => "Xyz049",

      Ee::PdaNoLamport => "PdaNoLamport",
      Ee::ForeignPDA => "ForeignPDA",
      Ee::ConfigPDA => "ConfigPDA",
      Ee::VaultPDA => "VaultPDA",
      Ee::AdminPDA => "AdminPDA",
      Ee::UserPDA => "UserPDA",
      Ee::ActionPDA => "ActionPDA",
      Ee::EscrowPDA => "EscrowPDA",
      Ee::Xyz058 => "Xyz058",
      Ee::Xyz059 => "Xyz059",

      Ee::ConfigDataLengh => "ConfigDataLengh",
      Ee::VaultDataLengh => "VaultDataLengh",
      Ee::AdminDataLengh => "AdminDataLengh",
      Ee::UserDataLengh => "UserDataLengh",
      Ee::EscrowDataLengh => "EscrowDataLengh",
      Ee::ActionDataLengh => "ActionDataLengh",
      Ee::Xyz066 => "Xyz066",
      Ee::Xyz067 => "Xyz067",
      Ee::Xyz068 => "Xyz068",
      Ee::Xyz069 => "Xyz069",

      Ee::DecimalsValue => "DecimalsValue",
      Ee::MintDataLen => "MintDataLen",
      Ee::MintOrMintAuthority => "MintOrMintAuthority",
      Ee::MintOrTokenProgram => "MintOrTokenProgram",
      Ee::Xyz074 => "Xyz074",
      Ee::Xyz075 => "Xyz075",
      Ee::Xyz076 => "Xyz076",
      Ee::Xyz077 => "Xyz077",
      Ee::Xyz078 => "Xyz078",
      Ee::Xyz079 => "Xyz079",

      Ee::AtaDataLen => "AtaDataLen",
      Ee::Ata22DataLen => "Ata22DataLen",
      Ee::AtaOrOwner => "AtaOrOwner",
      Ee::AtaOrMint => "AtaOrMint",
      Ee::AtaCheckX1 => "AtaCheckX1",
      Ee::ForeignAta => "ForeignAta",
      Ee::Xyz086 => "Xyz086",
      Ee::Xyz087 => "Xyz087",
      Ee::Xyz088 => "Xyz088",
      Ee::Xyz089 => "Xyz089",

      Ee::Acct22Type => "Acct22Type",
      Ee::Tok22AcctDiscOffset => "Tok22AcctDiscOffset",
      Ee::NotRentExemptMint22 => "NotRentExemptMint22",
      Ee::NotRentExemptTokAcct22 => "NotRentExemptTokAcct22",
      Ee::Xyz094 => "Xyz094",
      Ee::PdaToBeBelowRentExempt => "PdaToBeBelowRentExempt",
      Ee::ToWallet => "ToWallet",
      Ee::Xyz097 => "Xyz097",
      Ee::Xyz098 => "Xyz098",
      Ee::Xyz099 => "Xyz099",

      Ee::MathOverflow => "MathOverflow",
      Ee::MathUnderflow => "MathUnderflow",
      Ee::MultiplyOverflow => "MultiplyOverflow",
      Ee::DividedByZero => "DividedByZero",
      Ee::Remainder => "Remainder",
      Ee::EmptyData => "EmptyData",
      Ee::FunctionSelector => "FunctionSelector",
      Ee::ClockGet => "ClockGet",
      Ee::ErrorNotMapped => "ErrorNotMapped",
    }
  }
}

//----------------== Account Verification Functions
pub fn check_signer(account: &AccountInfo) -> Result<(), ProgramError> {
  if !account.is_signer() {
    return Err(ProgramError::MissingRequiredSignature);
  }
  Ok(())
}
pub fn check_mint0a(mint: &AccountInfo, token_program: &AccountInfo) -> Result<(), ProgramError> {
  //if !mint.is_owned_by(mint_authority)
  if mint.data_len() != pinocchio_token::state::Mint::LEN {
    return Err(Ee::MintDataLen.into());
  }
  if !token_program.key().eq(&pinocchio_token::ID) {
    return Err(Ee::TokenProgram.into());
  }
  if mint.owner() != &pinocchio_token::ID {
    return Err(Ee::MintOrTokenProgram.into());
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
    return Err(Ee::MintOrMintAuthority.into());
  }
  if decimals != mint_info.decimals() {
    return Err(Ee::DecimalsValue.into());
  }
  check_mint0a(mint, token_program)?;
  //TODO: over mint supply?
  Ok(())
}

pub fn check_mint22a(mint: &AccountInfo, token_program: &AccountInfo) -> Result<(), ProgramError> {
  //if !mint.is_owned_by(mint_authority)
  if mint.data_len() != pinocchio_token_2022::state::Mint::BASE_LEN {
    return Err(Ee::MintDataLen.into());
  }
  if !token_program.key().eq(&pinocchio_token_2022::ID) {
    return Err(Ee::TokenProgram.into());
  }
  if mint.owner() != &pinocchio_token_2022::ID {
    return Err(Ee::MintOrTokenProgram.into());
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
    return Err(Ee::MintOrMintAuthority.into());
  }
  if decimals != mint_info.decimals() {
    return Err(Ee::DecimalsValue.into());
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
    return Err(Ee::AtaDataLen.into());
  }
  let ata_info = pinocchio_token::state::TokenAccount::from_account_info(ata)?;
  if !ata_info.owner().eq(owner.key()) {
    return Err(Ee::AtaOrOwner.into());
  }
  if !ata_info.mint().eq(mint.key()) {
    return Err(Ee::AtaOrMint.into());
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
    return Err(Ee::AtaOrOwner.into());
  }
  if !ata_info.mint().eq(mint.key()) {
    return Err(Ee::AtaOrMint.into());
  }
  Ok(())
}
pub fn check_ata_x1(
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
    return Err(Ee::AtaCheckX1.into());
  }
  Ok(())
}
pub fn check_pda(account: &AccountInfo) -> Result<(), ProgramError> {
  if account.lamports() == 0 {
    return Err(Ee::PdaNoLamport.into());
  }
  if !account.is_owned_by(&crate::ID) {
    return Err(Ee::ForeignPDA.into());
  }
  Ok(())
}
pub fn check_sysprog(system_program: &AccountInfo) -> Result<(), ProgramError> {
  if !system_program.key().eq(&pinocchio_system::ID) {
    return Err(Ee::SystemProgram.into());
  }
  Ok(())
}
pub const ATOKENGPVBD: pinocchio_pubkey::reexport::Pubkey = pinocchio_system::ID; //[0, 0];
pub fn check_atoken_gpvbd(atoken_program: &AccountInfo) -> Result<(), ProgramError> {
  if !atoken_program.key().eq(&ATOKENGPVBD) {
    return Err(Ee::AtokenGPvbd.into());
  }
  Ok(())
}

//----------------== Check Account Properties
pub fn writable(account: &AccountInfo) -> Result<(), ProgramError> {
  if !account.is_writable() {
    return Err(Ee::NotWritable.into());
  }
  Ok(())
}
pub fn executable(account: &AccountInfo) -> Result<(), ProgramError> {
  if !account.executable() {
    return Err(Ee::NotExecutable.into());
  }
  Ok(())
}
//TODO: Mint and ATA from TokenLgc works. For mint and ATA from Token2022?
/// acc_type: 0 Mint, 1 TokenAccount
pub fn rent_exempt22(account: &AccountInfo, acc_type: u8) -> Result<(), ProgramError> {
  //Check Mint
  if acc_type == 0 && account.lamports() < Rent::get()?.minimum_balance(Mint22::BASE_LEN) {
    return Err(Ee::NotRentExemptMint22.into());
  }
  //Check TokenAccount
  if acc_type == 1 && account.lamports() < Rent::get()?.minimum_balance(TokenAccount22::BASE_LEN) {
    return Err(Ee::NotRentExemptTokAcct22.into());
  }
  if acc_type > 1 {
    return Err(Ee::Acct22Type.into());
  }
  Ok(())
}
pub fn rent_exempt(account: &AccountInfo) -> Result<(u64, u64), ProgramError> {
  let min_balance = Rent::get()?.minimum_balance(account.data_len());
  let current = account.lamports();
  if current < min_balance {
    return Err(ProgramError::AccountNotRentExempt);
  }
  Ok((current, min_balance))
}
pub fn not_initialized(account: &AccountInfo) -> Result<(), ProgramError> {
  if account.lamports() > 0 {
    return Err(ProgramError::AccountAlreadyInitialized);
  }
  Ok(())
}
pub fn initialized(account: &AccountInfo) -> Result<(), ProgramError> {
  if account.lamports() == 0 {
    return Err(ProgramError::UninitializedAccount);
  }
  Ok(())
}
pub fn empty_data(account: &AccountInfo) -> Result<(), ProgramError> {
  if account.data_len() == 0 {
    return Ok(());
  }
  Err(Ee::EmptyData.into())
}

//----------------== Check Input Values
pub fn min_data_len(data: &[u8], min: usize) -> Result<(), ProgramError> {
  if data.len() < min {
    return Err(Ee::InputDataUnderMin.into());
  }
  Ok(())
}
pub fn max_data_len(data: &[u8], max: usize) -> Result<(), ProgramError> {
  if data.len() > max {
    return Err(Ee::InputDataOverMax.into());
  }
  Ok(())
}

pub fn check_decimals(mint: &AccountInfo, decimals: u8) -> Result<(), ProgramError> {
  let mint_info = pinocchio_token::state::Mint::from_account_info(mint)?;
  if decimals != mint_info.decimals() {
    return Err(Ee::DecimalsValue.into());
  }
  Ok(())
}
pub fn check_decimals_max(decimals: u8, max: u8) -> Result<(), ProgramError> {
  if decimals > max {
    return Err(Ee::DecimalsValue.into());
  }
  Ok(())
}
pub fn check_str_len(s: &str, min_len: usize, max_len: usize) -> Result<(), ProgramError> {
  if s.len() < min_len {
    return Err(Ee::StrOverMax.into());
  }
  if s.len() > max_len {
    return Err(Ee::StrUnderMin.into());
  }
  Ok(())
}

//----------------== Parse Functions
/// Parse a u64 from u8 array
pub fn parse_u64(data: &[u8]) -> Result<u64, ProgramError> {
  let bytes: [u8; 8] = data.try_into().or_else(|_e| Err(Ee::ByteSizeForU64))?;

  let amt = u64::from_le_bytes(bytes);
  // let amount = u64::from_le_bytes([data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7]]);
  if amt == 0 {
    return Err(Ee::ZeroAsU64.into());
  }
  Ok(amt)
}
pub fn parse_u32(data: &[u8]) -> Result<u32, ProgramError> {
  let bytes: [u8; 4] = data.try_into().or_else(|_e| Err(Ee::ByteSizeForU32))?;

  let amt = u32::from_le_bytes(bytes);
  // let amount = u64::from_le_bytes([data[0], data[1], data[2], data[3]]);
  if amt == 0 {
    return Err(Ee::ZeroAsU32.into());
  }
  Ok(amt)
}
pub fn to32bytes(byte_slice: &[u8]) -> Result<&[u8; 32], ProgramError> {
  let bytes: &[u8; 32] = byte_slice.try_into().map_err(|_| Ee::ByteSliceSize32)?;
  //let mut str_u8array = [0u8; 32];
  //str_u8array.copy_from_slice(&data[10..42]);
  return Ok(bytes);
}
pub fn to10bytes(byte_slice: &[u8]) -> Result<&[u8; 10], ProgramError> {
  let bytes: &[u8; 10] = byte_slice.try_into().map_err(|_| Ee::ByteSliceSize10)?;
  return Ok(bytes);
}
pub fn to6bytes(byte_slice: &[u8]) -> Result<&[u8; 6], ProgramError> {
  let bytes: &[u8; 6] = byte_slice.try_into().map_err(|_| Ee::ByteSliceSize6)?;
  return Ok(bytes);
}
pub fn u8_to_bool(v: u8) -> Result<bool, ProgramError> {
  match v {
    0 => Ok(false),
    1 => Ok(true),
    _ => Err(Ee::ByteForBool.into()),
  }
}
pub fn u8_to_status(v: u8) -> Result<Status, ProgramError> {
  match v {
    0 => Ok(Status::Waiting),
    1 => Ok(Status::Active),
    2 => Ok(Status::Expired),
    3 => Ok(Status::Paused),
    4 => Ok(Status::Canceled),
    _ => Err(Ee::ByteForStatus.into()),
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
      return Err(Ee::MintOrTokenProgram.into());
    } else {
      if mint.data_len().ne(&pinocchio_token::state::Mint::LEN) {
        return Err(Ee::MintDataLen.into());
      }
    }
  } else {
    //Token2022
    let data = mint.try_borrow_data()?;

    if data.len().ne(&pinocchio_token::state::Mint::LEN) {
      if data.len().le(&TOKEN_2022_ACCOUNT_DISCRIMINATOR_OFFSET) {
        return Err(Ee::Ata22DataLen.into());
      }
      if data[TOKEN_2022_ACCOUNT_DISCRIMINATOR_OFFSET].ne(&TOKEN_2022_MINT_DISCRIMINATOR) {
        return Err(Ee::Tok22AcctDiscOffset.into());
      }
    }
  }
  Ok(())
}

pub fn check_tokacct_interface(ata: &AccountInfo) -> Result<(), ProgramError> {
  if !ata.is_owned_by(&pinocchio_token_2022::ID) {
    //Legacy ATA
    if !ata.is_owned_by(&pinocchio_token::ID) {
      return Err(Ee::ForeignAta.into());
    } else {
      if ata
        .data_len()
        .ne(&pinocchio_token::state::TokenAccount::LEN)
      {
        return Err(Ee::AtaDataLen.into());
      }
    }
  } else {
    //Token2022 ATA
    let data = ata.try_borrow_data()?;

    if data.len().ne(&pinocchio_token::state::TokenAccount::LEN) {
      if data.len().le(&TOKEN_2022_ACCOUNT_DISCRIMINATOR_OFFSET) {
        return Err(Ee::Ata22DataLen.into());
      }
      if data[TOKEN_2022_ACCOUNT_DISCRIMINATOR_OFFSET].ne(&TOKEN_2022_TOKEN_ACCOUNT_DISCRIMINATOR) {
        return Err(Ee::Tok22AcctDiscOffset.into());
      }
    }
  }
  Ok(())
}
pub fn get_time() -> Result<u32, ProgramError> {
  let clock = Clock::get().map_err(|_| Ee::ClockGet)?;
  let time = clock.unix_timestamp as u32;
  log!("Solana time: {}", time);
  Ok(time)
}
