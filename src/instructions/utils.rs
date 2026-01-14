//use num_derive::FromPrimitive;
use pinocchio::{
  account_info::AccountInfo,
  program_error::{ProgramError, ToStr},
  pubkey::{find_program_address, try_find_program_address, Pubkey},
  sysvars::{clock::Clock, rent::Rent, Sysvar},
  ProgramResult,
};
use pinocchio_log::log;
use pinocchio_token::state::{Mint, TokenAccount};
use pinocchio_token_2022::state::{Mint as Mint22, TokenAccount as TokenAccount22};
use thiserror::Error;

use crate::Status;

//TODO: put errors in error.rs ... https://learn.blueshift.gg/en/courses/pinocchio-for-dummies/pinocchio-errors
#[derive(Clone, Debug, Eq, Error, PartialEq)] //FromPrimitive
pub enum Ee {
  #[error("MethodDiscriminator")]
  MethodDiscriminator,
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
  #[error("MintNotAccepted")]
  MintNotAccepted,
  #[error("MintsAreTheSame")]
  MintsAreTheSame,
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
  #[error("ZeroU128")]
  ZeroU128,
  #[error("ZeroU64")]
  ZeroU64,
  #[error("ZeroU32")]
  ZeroU32,
  #[error("ZeroU16")]
  ZeroU16,
  #[error("ZeroU8")]
  ZeroU8,
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
  #[error("InputDataLen")]
  InputDataLen,
  #[error("InputDataBump")]
  InputDataBump,
  #[error("ByteForBool")]
  ByteForBool,
  #[error("ByteForStatus")]
  ByteForStatus,
  #[error("Xyz044")]
  Xyz044,
  #[error("Xyz045")]
  Xyz045,
  #[error("Xyz046")]
  Xyz046,
  #[error("Xyz047")]
  Xyz047,
  #[error("EscrowIsForeign")]
  EscrowIsForeign,
  #[error("ConfigIsForeign")]
  ConfigIsForeign,
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
  #[error("NoRentExemptMint")]
  NoRentExemptMint,
  #[error("NoRentExemptTokAcct")]
  NoRentExemptTokAcct,
  #[error("NoRentExemptMint22")]
  NoRentExemptMint22,
  #[error("NoRentExemptTokAcct22")]
  NoRentExemptTokAcct22,
  #[error("Tok22AcctDiscOffset")]
  Tok22AcctDiscOffset,
  //Withdraw
  #[error("PdaToBeBelowRentExempt")]
  PdaToBeBelowRentExempt,
  #[error("ToWallet")]
  ToWallet,
  #[error("ToWalletNoLamport")]
  ToWalletNoLamport,
  #[error("ToWalletForeignPDA")]
  ToWalletForeignPDA,
  #[error("Xyz099")]
  Xyz099,
  //Math
  //ArithmeticOverflow exists
  #[error("Xyz100")]
  Xyz100,
  #[error("Xyz101")]
  Xyz101,
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
  #[error("NotMapped")]
  NotMapped,
  //ProgramError: AccountBorrowFailed
}
impl Ee {
  pub fn e(self) -> Result<(), ProgramError> {
    Err(ProgramError::Custom(self as u32))
  }
}
impl From<Ee> for ProgramError {
  fn from(e: Ee) -> Self {
    ProgramError::Custom(e as u32)
  }
}
//Deserialize Errors from Raw Values
impl TryFrom<u32> for Ee {
  type Error = ProgramError;
  fn try_from(error: u32) -> Result<Self, Self::Error> {
    match error {
      0 => Ok(Ee::MethodDiscriminator),
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
      12 => Ok(Ee::MintNotAccepted),
      13 => Ok(Ee::MintsAreTheSame),
      14 => Ok(Ee::Xyz014),
      15 => Ok(Ee::Xyz015),
      16 => Ok(Ee::Xyz016),
      17 => Ok(Ee::Xyz017),
      18 => Ok(Ee::Xyz018),
      19 => Ok(Ee::Xyz019),
      20 => Ok(Ee::ZeroU128),
      21 => Ok(Ee::ZeroU64),
      22 => Ok(Ee::ZeroU32),
      23 => Ok(Ee::ZeroU16),
      24 => Ok(Ee::ZeroU8),
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
      40 => Ok(Ee::InputDataLen),
      41 => Ok(Ee::InputDataBump),
      42 => Ok(Ee::ByteForBool),
      43 => Ok(Ee::ByteForStatus),
      44 => Ok(Ee::Xyz044),
      45 => Ok(Ee::Xyz045),
      46 => Ok(Ee::Xyz046),
      47 => Ok(Ee::Xyz047),
      48 => Ok(Ee::EscrowIsForeign),
      49 => Ok(Ee::ConfigIsForeign),
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
      90 => Ok(Ee::NoRentExemptMint),
      91 => Ok(Ee::NoRentExemptTokAcct),
      92 => Ok(Ee::NoRentExemptMint22),
      93 => Ok(Ee::NoRentExemptTokAcct22),
      94 => Ok(Ee::Tok22AcctDiscOffset),
      95 => Ok(Ee::PdaToBeBelowRentExempt),
      96 => Ok(Ee::ToWallet),
      97 => Ok(Ee::ToWalletNoLamport),
      98 => Ok(Ee::ToWalletForeignPDA),
      99 => Ok(Ee::Xyz099),
      100 => Ok(Ee::Xyz100),
      101 => Ok(Ee::Xyz101),
      102 => Ok(Ee::MultiplyOverflow),
      103 => Ok(Ee::DividedByZero),
      104 => Ok(Ee::Remainder),
      105 => Ok(Ee::EmptyData),
      106 => Ok(Ee::FunctionSelector),
      107 => Ok(Ee::ClockGet),
      _ => Err(Ee::NotMapped.into()),
    }
  }
}
//Human Readable Errors; TODO: arrange below
impl ToStr for Ee {
  fn to_str<E>(&self) -> &'static str {
    match self {
      Ee::MethodDiscriminator => "MethodDiscriminator",
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
      Ee::MintNotAccepted => "MintNotAccepted",
      Ee::MintsAreTheSame => "MintsAreTheSame",
      Ee::Xyz014 => "Xyz014",
      Ee::Xyz015 => "Xyz015",
      Ee::Xyz016 => "Xyz016",
      Ee::Xyz017 => "Xyz017",
      Ee::Xyz018 => "Xyz018",
      Ee::Xyz019 => "Xyz019",

      Ee::ZeroU128 => "ZeroU128",
      Ee::ZeroU64 => "ZeroU64",
      Ee::ZeroU32 => "ZeroU32",
      Ee::ZeroU16 => "ZeroU16",
      Ee::ZeroU8 => "ZeroU8",
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

      Ee::InputDataLen => "InputDataLen",
      Ee::InputDataBump => "InputDataBump",
      Ee::ByteForBool => "ByteForBool",
      Ee::ByteForStatus => "ByteForStatus",
      Ee::Xyz044 => "Xyz044",
      Ee::Xyz045 => "Xyz045",
      Ee::Xyz046 => "Xyz046",
      Ee::Xyz047 => "Xyz047",
      Ee::EscrowIsForeign => "EscrowIsForeign",
      Ee::ConfigIsForeign => "ConfigIsForeign",

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

      Ee::NoRentExemptMint => "NoRentExemptMint",
      Ee::NoRentExemptTokAcct => "NoRentExemptTokAcct",
      Ee::NoRentExemptMint22 => "NoRentExemptMint22",
      Ee::NoRentExemptTokAcct22 => "NoRentExemptTokAcct22",
      Ee::Tok22AcctDiscOffset => "Tok22AcctDiscOffset",
      Ee::PdaToBeBelowRentExempt => "PdaToBeBelowRentExempt",
      Ee::ToWallet => "ToWallet",
      Ee::ToWalletNoLamport => "ToWalletNoLamport",
      Ee::ToWalletForeignPDA => "ToWalletForeignPDA",
      Ee::Xyz099 => "Xyz099",

      Ee::Xyz100 => "Xyz100",
      Ee::Xyz101 => "Xyz101",
      Ee::MultiplyOverflow => "MultiplyOverflow",
      Ee::DividedByZero => "DividedByZero",
      Ee::Remainder => "Remainder",
      Ee::EmptyData => "EmptyData",
      Ee::FunctionSelector => "FunctionSelector",
      Ee::ClockGet => "ClockGet",
      Ee::NotMapped => "NotMapped",
    }
  }
}

//----------------== Account Verification
pub fn check_signer(account: &AccountInfo) -> Result<(), ProgramError> {
  if !account.is_signer() {
    return Err(ProgramError::MissingRequiredSignature);
  }
  Ok(())
}
pub fn check_mint_xy(mint_x: &AccountInfo, mint_y: &AccountInfo) -> Result<(), ProgramError> {
  if mint_x.key() == mint_y.key() {
    return Ee::MintsAreTheSame.e();
  }
  Ok(())
}
pub fn check_mint0a(mint: &AccountInfo, token_program: &AccountInfo) -> Result<(), ProgramError> {
  //if !mint.is_owned_by(mint_authority)
  if mint.data_len() != pinocchio_token::state::Mint::LEN {
    return Ee::MintDataLen.e();
  }
  if !token_program.key().eq(&pinocchio_token::ID) {
    return Ee::TokenProgram.e();
  }
  if mint.owner() != &pinocchio_token::ID {
    return Ee::MintOrTokenProgram.e();
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
    return Ee::MintOrMintAuthority.e();
  }
  if decimals != mint_info.decimals() {
    return Ee::DecimalsValue.e();
  }
  check_mint0a(mint, token_program)?;
  //TODO: over mint supply?
  Ok(())
}

pub fn check_mint22a(mint: &AccountInfo, token_program: &AccountInfo) -> Result<(), ProgramError> {
  //if !mint.is_owned_by(mint_authority)
  if mint.data_len() != pinocchio_token_2022::state::Mint::BASE_LEN {
    return Ee::MintDataLen.e();
  }
  if !token_program.key().eq(&pinocchio_token_2022::ID) {
    return Ee::TokenProgram.e();
  }
  if mint.owner() != &pinocchio_token_2022::ID {
    return Ee::MintOrTokenProgram.e();
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
    return Ee::MintOrMintAuthority.e();
  }
  if decimals != mint_info.decimals() {
    return Ee::DecimalsValue.e();
  }
  check_mint22a(mint, token_program)?;
  //TODO: over mint supply?
  Ok(())
}

//----------------== ATA
pub fn ata_balc(from_ata: &AccountInfo, amount: u64) -> Result<(), ProgramError> {
  let from_ata_info = TokenAccount::from_account_info(from_ata)?;
  if from_ata_info.amount() < amount {
    return Err(ProgramError::InsufficientFunds);
  }
  Ok(())
}
pub fn ata_balc22(from_ata: &AccountInfo, amount: u64) -> Result<(), ProgramError> {
  let from_ata_info = TokenAccount22::from_account_info(from_ata)?;
  if from_ata_info.amount() < amount {
    return Err(ProgramError::InsufficientFunds);
  }
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
    return Ee::AtaDataLen.e();
  }
  let ata_info = pinocchio_token::state::TokenAccount::from_account_info(ata)?;
  if !ata_info.owner().eq(owner.key()) {
    return Ee::AtaOrOwner.e();
  }
  if !ata_info.mint().eq(mint.key()) {
    return Ee::AtaOrMint.e();
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
    return Ee::AtaOrOwner.e();
  }
  if !ata_info.mint().eq(mint.key()) {
    return Ee::AtaOrMint.e();
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
    return Ee::AtaCheckX1.e();
  }
  Ok(())
}
pub fn check_ata_escrow(
  ata: &AccountInfo,
  owner: &AccountInfo,
  mint: &AccountInfo,
) -> Result<(), ProgramError> {
  // if !owner.is_owned_by(&crate::ID) {
  //   return Ee::ToWalletForeignPDA.e();
  // } ... escrow as owner may not exist yet
  if ata
    .data_len()
    .ne(&pinocchio_token::state::TokenAccount::LEN)
  {
    return Ee::AtaDataLen.e();
  }
  let ata_info = pinocchio_token::state::TokenAccount::from_account_info(ata)?;
  if !ata_info.owner().eq(owner.key()) {
    return Ee::AtaOrOwner.e();
  }
  if !ata_info.mint().eq(mint.key()) {
    return Ee::AtaOrMint.e();
  }
  Ok(())
}
//----------------== PDAs and Other Accounts
pub fn derive_pda1(user: &Pubkey, bstr: &[u8]) -> Result<(Pubkey, u8), ProgramError> {
  log!("derive_pda1");
  //find_program_address(&[b"vault", user.key().as_ref()], &crate::ID)
  // let (pda, _bump) =
  try_find_program_address(&[bstr, user.as_ref()], &crate::ID)
    .ok_or_else(|| ProgramError::InvalidSeeds)
}
/*let pda = pubkey::create_program_address(
    &[PDA_SEED, &[self.datas.bump as u8]],
    &crate::ID,
) */
pub fn check_pda(account: &AccountInfo) -> Result<(), ProgramError> {
  if account.lamports() == 0 {
    return Ee::PdaNoLamport.e();
  }
  if !account.is_owned_by(&crate::ID) {
    return Ee::ForeignPDA.e();
  }
  Ok(())
}
pub fn check_vault(input_vault: &AccountInfo, config_vault: &[u8; 32]) -> Result<(), ProgramError> {
  if input_vault.lamports() == 0 {
    return Ee::ToWalletNoLamport.e();
  }
  if !input_vault.is_owned_by(&crate::ID) {
    return Ee::ToWalletForeignPDA.e();
  }
  if input_vault.key() != config_vault {
    return Ee::ToWallet.e();
  }
  Ok(())
}
pub fn check_sysprog(system_program: &AccountInfo) -> Result<(), ProgramError> {
  if !system_program.key().eq(&pinocchio_system::ID) {
    return Ee::SystemProgram.e();
  }
  Ok(())
}
pub const ATOKENGPVBD: pinocchio_pubkey::reexport::Pubkey = pinocchio_associated_token_account::ID;
pub fn check_atoken_gpvbd(atoken_program: &AccountInfo) -> Result<(), ProgramError> {
  if !atoken_program.key().eq(&ATOKENGPVBD) {
    return Ee::AtokenGPvbd.e();
  }
  Ok(())
}
//pub const SYSTEMPROGRAM: pinocchio_pubkey::reexport::Pubkey = solana_system_interface::program::ID;

//----------------== Check Account Properties
pub fn writable(account: &AccountInfo) -> Result<(), ProgramError> {
  if !account.is_writable() {
    return Ee::NotWritable.e();
  }
  Ok(())
}
pub fn executable(account: &AccountInfo) -> Result<(), ProgramError> {
  if !account.executable() {
    return Ee::NotExecutable.e();
  }
  Ok(())
}
//TODO: Mint and ATA from TokenLgc works. For mint and ATA from Token2022?
/// acc_type: 0 Mint, 1 TokenAccount
pub fn rent_exempt_mint(account: &AccountInfo) -> Result<(), ProgramError> {
  if account.lamports() < Rent::get()?.minimum_balance(Mint::LEN) {
    return Ee::NoRentExemptMint.e();
  }
  Ok(())
}
pub fn rent_exempt_mint22(account: &AccountInfo) -> Result<(), ProgramError> {
  if account.lamports() < Rent::get()?.minimum_balance(Mint22::BASE_LEN) {
    return Ee::NoRentExemptMint22.e();
  }
  Ok(())
}
pub fn rent_exempt_tokacct(account: &AccountInfo) -> Result<(), ProgramError> {
  if account.lamports() < Rent::get()?.minimum_balance(TokenAccount::LEN) {
    return Ee::NoRentExemptTokAcct22.e();
  }
  Ok(())
}
pub fn rent_exempt_tokacct22(account: &AccountInfo) -> Result<(), ProgramError> {
  if account.lamports() < Rent::get()?.minimum_balance(TokenAccount22::BASE_LEN) {
    return Ee::NoRentExemptTokAcct22.e();
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
  Ee::EmptyData.e()
}

//----------------== Check Input Values
pub fn data_len(data: &[u8], expected: usize) -> Result<(), ProgramError> {
  if data.len() != expected {
    return Ee::InputDataLen.e();
  }
  Ok(())
}
pub fn check_decimals(mint: &AccountInfo, decimals: u8) -> Result<(), ProgramError> {
  let mint_info = pinocchio_token::state::Mint::from_account_info(mint)?;
  if decimals != mint_info.decimals() {
    return Ee::DecimalsValue.e();
  }
  Ok(())
}
pub fn check_decimals_max(decimals: u8, max: u8) -> Result<(), ProgramError> {
  if decimals > max {
    return Ee::DecimalsValue.e();
  }
  Ok(())
}

//----------------== Parse Functions
//cannot use std traits for u64, u32, ...
pub fn none_zero_u64(uint: u64) -> ProgramResult {
  if uint == 0u64 {
    return Ee::ZeroU64.e();
  }
  Ok(())
}
pub fn none_zero_u32(uint: u32) -> ProgramResult {
  if uint == 0u32 {
    return Ee::ZeroU32.e();
  }
  Ok(())
}
pub fn none_zero_u8(uint: u8) -> ProgramResult {
  if uint == 0u8 {
    return Ee::ZeroU8.e();
  }
  Ok(())
}
/// Parse a u64 from u8 array
pub fn parse_u64(data: &[u8]) -> Result<u64, ProgramError> {
  let bytes: [u8; 8] = data.try_into().or_else(|_e| Err(Ee::ByteSizeForU64))?;

  let amt = u64::from_le_bytes(bytes);
  // let amount = u64::from_le_bytes([data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7]]);
  Ok(amt)
}
pub fn parse_u32(data: &[u8]) -> Result<u32, ProgramError> {
  let bytes: [u8; 4] = data.try_into().or_else(|_e| Err(Ee::ByteSizeForU32))?;

  let amt = u32::from_le_bytes(bytes);
  // let amount = u64::from_le_bytes([data[0], data[1], data[2], data[3]]);
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
//----------------== Balance
pub fn sol_balc(from: &AccountInfo, amount: u64) -> Result<(), ProgramError> {
  if from.lamports() < amount {
    return Err(ProgramError::InsufficientFunds);
  }
  Ok(())
}

//----------------== Token 2022 Interface
const TOKEN_2022_ACCOUNT_DISCRIMINATOR_OFFSET: usize = 165;
pub const TOKEN_2022_MINT_DISCRIMINATOR: u8 = 0x01;
pub const TOKEN_2022_TOKEN_ACCOUNT_DISCRIMINATOR: u8 = 0x02;

pub fn check_mint_interface(mint: &AccountInfo) -> Result<(), ProgramError> {
  if !mint.is_owned_by(&pinocchio_token_2022::ID) {
    //legacy token
    if !mint.is_owned_by(&pinocchio_token::ID) {
      return Ee::MintOrTokenProgram.e();
    } else {
      if mint.data_len().ne(&pinocchio_token::state::Mint::LEN) {
        return Ee::MintDataLen.e();
      }
    }
  } else {
    //Token2022
    let data = mint.try_borrow_data()?;

    if data.len().ne(&pinocchio_token::state::Mint::LEN) {
      if data.len().le(&TOKEN_2022_ACCOUNT_DISCRIMINATOR_OFFSET) {
        return Ee::Ata22DataLen.e();
      }
      if data[TOKEN_2022_ACCOUNT_DISCRIMINATOR_OFFSET].ne(&TOKEN_2022_MINT_DISCRIMINATOR) {
        return Ee::Tok22AcctDiscOffset.e();
      }
    }
  }
  Ok(())
}

pub fn check_tokacct_interface(ata: &AccountInfo) -> Result<(), ProgramError> {
  if !ata.is_owned_by(&pinocchio_token_2022::ID) {
    //Legacy ATA
    if !ata.is_owned_by(&pinocchio_token::ID) {
      return Ee::ForeignAta.e();
    } else {
      if ata
        .data_len()
        .ne(&pinocchio_token::state::TokenAccount::LEN)
      {
        return Ee::AtaDataLen.e();
      }
    }
  } else {
    //Token2022 ATA
    let data = ata.try_borrow_data()?;

    if data.len().ne(&pinocchio_token::state::TokenAccount::LEN) {
      if data.len().le(&TOKEN_2022_ACCOUNT_DISCRIMINATOR_OFFSET) {
        return Ee::Ata22DataLen.e();
      }
      if data[TOKEN_2022_ACCOUNT_DISCRIMINATOR_OFFSET].ne(&TOKEN_2022_TOKEN_ACCOUNT_DISCRIMINATOR) {
        return Ee::Tok22AcctDiscOffset.e();
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
