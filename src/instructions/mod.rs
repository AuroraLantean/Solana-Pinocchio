use pinocchio::pubkey::find_program_address;
use pinocchio::{
    account_info::AccountInfo,
    program_error::ProgramError,
    pubkey::{try_find_program_address, Pubkey},
    sysvars::{rent::Rent, Sysvar},
};
use pinocchio_token_2022::state::{Mint as Mint22, TokenAccount as TokenAccount22};

#[allow(non_snake_case)]
pub mod closeConfig;
#[allow(non_snake_case)]
pub mod depositSol;
#[allow(non_snake_case)]
pub mod tok22InitATA;
#[allow(non_snake_case)]
pub mod tok22InitMint;
#[allow(non_snake_case)]
pub mod tok22MintToken;
#[allow(non_snake_case)]
pub mod tokLgcDeposit;
#[allow(non_snake_case)]
pub mod tokLgcInitATA;
#[allow(non_snake_case)]
pub mod tokLgcInitMint;
#[allow(non_snake_case)]
pub mod tokLgcMintToken;
#[allow(non_snake_case)]
pub mod tokLgcRedeem;
#[allow(non_snake_case)]
pub mod tokLgcWithdraw;
pub mod utils;
#[allow(non_snake_case)]
pub mod withdrawSol;

pub use closeConfig::*;
pub use depositSol::*;
pub use tok22InitATA::*;
pub use tok22InitMint::*;
pub use tok22MintToken::*;
pub use tokLgcDeposit::*;
pub use tokLgcInitATA::*;
pub use tokLgcInitMint::*;
pub use tokLgcMintToken::*;
pub use tokLgcRedeem::*;
pub use tokLgcWithdraw::*;
pub use utils::*;
pub use withdrawSol::*;

use shank::ShankInstruction;

/// Shank IDL enum describes all program instructions and their required accounts.
/// Manually write this below, then run IDL generation; This below does not affect runtime behavior.
/// TODO: when is signer writable?
/// writable(to be modified):, name= signer, token_account, pda
/// non writable: program, system_program, mint
#[derive(ShankInstruction)]
pub enum ProgramIx {
    /// 0 Deposit lamports into the vault.
    #[account(0, signer, writable, name = "user", desc = "User")]
    #[account(1, writable, name = "vault", desc = "VaultPDA")]
    #[account(2, name = "system_program", desc = "System Program")]
    Deposit { amount: u64 },

    /// 1 Withdraw lamports from the vault
    #[account(0, signer, writable, name = "user", desc = "User")]
    #[account(1, writable, name = "vault", desc = "Vault PDA")]
    Withdraw { amount: u64 },

    /// 2 TokLgc Init Mint
    #[account(0, signer, writable, name = "payer", desc = "Payer")]
    #[account(1, signer, writable, name = "mint", desc = "Mint")]
    #[account(2, name = "mint_authority", desc = "Mint Authority")]
    #[account(3, name = "token_program", desc = "Token Program")]
    #[account(4, name = "freeze_authority_opt", desc = "Freeze Authority")]
    #[account(5, name = "system_program", desc = "System Program")]
    TokenLgcInitMint { decimals: u8 },

    /// 3 TokLgc Init ATA(Associated Token Acct)
    #[account(0, signer, writable, name = "payer", desc = "Payer")]
    #[account(1, name = "to_wallet", desc = "To Wallet")]
    #[account(2, name = "mint", desc = "Mint")]
    #[account(3, writable, name = "token_account", desc = "ATA Token Account")]
    #[account(4, name = "token_program", desc = "Token Program")]
    #[account(5, name = "system_program", desc = "System Program")]
    #[account(6, name = "atoken_program", desc = "AToken Program")]
    TokenLgcInitATA {},

    /// 4 TokLgc Mint Token
    #[account(0, signer, writable, name = "mint_authority", desc = "Mint Authority")]
    #[account(1, name = "to_wallet", desc = "ToWallet")]
    #[account(2, writable, name = "mint", desc = "Mint")]
    #[account(3, writable, name = "token_account", desc = "ATA Token Account")]
    #[account(4, name = "token_program", desc = "Token Program")]
    #[account(5, name = "system_program", desc = "System Program")]
    #[account(6, name = "atoken_program", desc = "AToken Program")]
    TokLgcMintToken { decimals: u8, amount: u64 },

    /// 5 TokLgc Deposit/Pay Tokens
    #[account(0, signer, writable, name = "user", desc = "User")]
    #[account(1, writable, name = "from", desc = "From ATA")]
    #[account(2, writable, name = "to", desc = "To ATA")]
    #[account(3, name = "to_wallet", desc = "To Wallet")]
    #[account(4, name = "mint", desc = "Mint")]
    #[account(5, name = "token_program", desc = "Token Program")]
    #[account(6, name = "system_program", desc = "System Program")]
    #[account(7, name = "atoken_program", desc = "AToken Program")]
    TokLgcDeposit { decimals: u8, amount: u64 },

    /// 6 TokLgc Withdraw Token
    #[account(0, signer, writable, name = "user", desc = "User")]
    #[account(1, writable, name = "from", desc = "From ATA")]
    #[account(2, writable, name = "to", desc = "To ATA")]
    #[account(3, name = "from_wallet", desc = "From Wallet")]
    #[account(4, name = "mint", desc = "Mint")]
    #[account(5, name = "token_program", desc = "Token Program")]
    #[account(6, name = "system_program", desc = "System Program")]
    #[account(7, name = "atoken_program", desc = "AToken Program")]
    TokLgcWithdraw { decimals: u8, amount: u64 },

    /// 7 TokLgc Redeem Tokens
    #[account(0, signer, writable, name = "user", desc = "User")]
    #[account(1, writable, name = "from", desc = "From ATA")]
    #[account(2, writable, name = "to", desc = "To ATA")]
    #[account(3, name = "from_pda", desc = "From PDA")]
    #[account(4, name = "from_pda_owner", desc = "From PDA Owner")]
    #[account(5, name = "mint", desc = "Mint")]
    #[account(6, name = "token_program", desc = "Token Program")]
    #[account(7, name = "system_program", desc = "System Program")]
    #[account(8, name = "atoken_program", desc = "AToken Program")]
    TokLgcRedeem { decimals: u8, amount: u64 },

    //---------== Token2022
    /// 8 Token2022 Init Mint
    #[account(0, signer, writable, name = "payer", desc = "Payer")]
    #[account(1, signer, writable, name = "mint", desc = "Mint")]
    #[account(2, name = "mint_authority", desc = "Mint Authority")]
    #[account(3, name = "token_program", desc = "Token Program")]
    #[account(4, name = "freeze_authority_opt", desc = "Freeze Authority")]
    #[account(5, name = "system_program", desc = "System Program")]
    Token2022InitMint { decimals: u8 },

    /// 9 Token2022 Init ATA(Associated Token Acct)
    #[account(0, signer, writable, name = "payer", desc = "Payer")]
    #[account(1, name = "to_wallet", desc = "To Wallet")]
    #[account(2, name = "mint", desc = "Mint")]
    #[account(3, writable, name = "token_account", desc = "ATA Token Account")]
    #[account(4, name = "token_program", desc = "Token Program")]
    #[account(5, name = "system_program", desc = "System Program")]
    #[account(6, name = "atoken_program", desc = "AToken Program")]
    Token2022InitATA {},

    /// 10 Token2022 Mint Token
    #[account(0, signer, writable, name = "mint_authority", desc = "Mint Authority")]
    #[account(1, name = "to_wallet", desc = "ToWallet")]
    #[account(2, writable, name = "mint", desc = "Mint")]
    #[account(3, writable, name = "token_account", desc = "ATA Token Account")]
    #[account(4, name = "token_program", desc = "Token Program")]
    #[account(5, name = "system_program", desc = "System Program")]
    #[account(6, name = "atoken_program", desc = "AToken Program")]
    Tok22MintToken { decimals: u8, amount: u64 },

    /// 11 Init Config PDA
    #[account(0, signer, writable, name = "authority", desc = "Authority")]
    #[account(1, name = "pda", desc = "PDA")]
    #[account(2, name = "dest", desc = "Destination")]
    //#[account(5, name = "system_program", desc = "System Program")]
    InitConfigPda {},

    /// 12 Close Config PDA
    #[account(0, signer, writable, name = "authority", desc = "Authority")]
    #[account(1, name = "pda", desc = "PDA")]
    #[account(2, name = "dest", desc = "Destination")]
    //#[account(5, name = "system_program", desc = "System Program")]
    CloseConfigPda {},
} //update here and lib.rs for new functions

//-------------==
/// Parse a u64 from instruction data.
/// amount must be non-zero,
pub fn parse_u64(data: &[u8]) -> Result<u64, ProgramError> {
    if data.len() != core::mem::size_of::<u64>() {
        return Err(ProgramError::Custom(400));
    }

    let bytes: [u8; 8] = data
        .try_into()
        .or_else(|_e| Err(ProgramError::Custom(401)))?;

    // Convert the byte slice to a u64
    let amt = u64::from_le_bytes(bytes);
    // let amount = u64::from_le_bytes([data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7]]);

    // Validate the amount (e.g., not zero)
    if amt == 0 {
        return Err(ProgramError::InvalidArgument);
    }
    Ok(amt)
}

/// Derive the vault PDA for an user -> (pda, bump)
pub fn derive_pda1(user: &AccountInfo, bstr: &[u8]) -> Result<(Pubkey, u8), ProgramError> {
    //find_program_address(&[b"vault", user.key().as_ref()], &crate::ID)
    // let (pda, _bump) =
    try_find_program_address(&[bstr, user.key().as_ref()], &crate::ID)
        .ok_or(ProgramError::InvalidSeeds)
}
pub fn check_signer(account: &AccountInfo) -> Result<(), ProgramError> {
    if !account.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }
    Ok(())
}
pub fn check_mint0a(mint: &AccountInfo, token_program: &AccountInfo) -> Result<(), ProgramError> {
    //if !mint.is_owned_by(mint_authority)
    if mint.data_len() != pinocchio_token::state::Mint::LEN {
        return Err(ProgramError::Custom(402));
    }
    if !token_program.key().eq(&pinocchio_token::ID) {
        return Err(ProgramError::Custom(403));
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
        return Err(ProgramError::IncorrectAuthority);
    }
    if decimals != mint_info.decimals() {
        return Err(ProgramError::Custom(404));
    }
    check_mint0a(mint, token_program)?;
    //TODO: over mint supply?
    Ok(())
}
pub fn check_mint22a(mint: &AccountInfo, token_program: &AccountInfo) -> Result<(), ProgramError> {
    //if !mint.is_owned_by(mint_authority)
    if mint.data_len() != pinocchio_token_2022::state::Mint::BASE_LEN {
        return Err(ProgramError::Custom(405));
    }
    if !token_program.key().eq(&pinocchio_token_2022::ID) {
        return Err(ProgramError::Custom(406));
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
        return Err(ProgramError::IncorrectAuthority);
    }
    if decimals != mint_info.decimals() {
        return Err(ProgramError::Custom(407));
    }
    check_mint22a(mint, token_program)?;
    //TODO: over mint supply?
    Ok(())
}
pub fn check_decimals(mint: &AccountInfo, decimals: u8) -> Result<(), ProgramError> {
    let mint_info = pinocchio_token::state::Mint::from_account_info(mint)?;
    if decimals != mint_info.decimals() {
        return Err(ProgramError::Custom(408));
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
        return Err(ProgramError::Custom(409));
    }
    let ata_info = pinocchio_token::state::TokenAccount::from_account_info(ata)?;
    if !ata_info.owner().eq(owner.key()) {
        return Err(ProgramError::InvalidAccountOwner);
    }
    if !ata_info.mint().eq(mint.key()) {
        return Err(ProgramError::Custom(410));
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
        return Err(ProgramError::InvalidAccountOwner);
    }
    if !ata_info.mint().eq(mint.key()) {
        return Err(ProgramError::Custom(411));
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
        return Err(ProgramError::Custom(421));
    }
    Ok(())
}
pub fn check_sysprog(system_program: &AccountInfo) -> Result<(), ProgramError> {
    if !system_program.key().eq(&pinocchio_system::ID) {
        return Err(ProgramError::Custom(412));
    }
    Ok(())
}

pub fn check_pda(account: &AccountInfo) -> Result<(), ProgramError> {
    if !account.is_owned_by(&crate::ID) {
        return Err(ProgramError::Custom(413));
    }
    Ok(())
}
pub fn empty_lamport(account: &AccountInfo) -> Result<(), ProgramError> {
    if account.lamports() == 0 {
        return Ok(());
    }
    Err(ProgramError::Custom(414))
}
pub fn empty_data(account: &AccountInfo) -> Result<(), ProgramError> {
    if account.data_len() == 0 {
        return Ok(());
    }
    Err(ProgramError::Custom(415))
}
pub fn writable(account: &AccountInfo) -> Result<(), ProgramError> {
    if !account.is_writable() {
        return Err(ProgramError::Custom(416));
    }
    Ok(())
}
pub fn executable(account: &AccountInfo) -> Result<(), ProgramError> {
    if !account.executable() {
        return Err(ProgramError::Custom(417));
    }
    Ok(())
}
//TODO: Mint and ATA from TokenLgc works. For mint and ATA from Token2022?
/// acc_type: 0 Mint, 1 TokenAccount
pub fn rent_exempt(account: &AccountInfo, acc_type: u8) -> Result<(), ProgramError> {
    if acc_type == 0 && account.lamports() < Rent::get()?.minimum_balance(Mint22::BASE_LEN) {
        return Err(ProgramError::AccountNotRentExempt);
    }
    if acc_type == 1 && account.lamports() < Rent::get()?.minimum_balance(TokenAccount22::BASE_LEN)
    {
        return Err(ProgramError::AccountNotRentExempt);
    }
    if acc_type > 1 {
        return Err(ProgramError::Custom(418));
    }
    Ok(())
}
pub fn check_str_len(s: &str, min_len: usize, max_len: usize) -> Result<(), ProgramError> {
    if s.len() < min_len {
        return Err(ProgramError::Custom(419));
    }
    if s.len() > max_len {
        return Err(ProgramError::Custom(420));
    }
    Ok(())
}

pub const ACCOUNT_DISCRIMINATOR_SIZE: usize = 8;

/// [4 (extension discriminator) + 32 (update_authority) + 32 (metadata)]
pub const METADATA_POINTER_SIZE: usize = 4 + 32 + 32;
/// [4 (extension discriminator) + 32 (update_authority) + 32 (mint) + 4 (size of name ) + 4 (size of symbol) + 4 (size of uri) + 4 (size of additional_metadata)]
pub const METADATA_EXTENSION_BASE_SIZE: usize = 4 + 32 + 32 + 4 + 4 + 4 + 4;
/// Padding used so that Mint and Account extensions start at the same index
pub const EXTENSIONS_PADDING_AND_OFFSET: usize = 84;
