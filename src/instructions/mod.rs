use pinocchio::{
    account_info::AccountInfo,
    program_error::ProgramError,
    pubkey::{try_find_program_address, Pubkey},
    sysvars::{rent::Rent, Sysvar},
};
use pinocchio_token_2022::state::{Mint as Mint22, TokenAccount as TokenAccount22};

#[allow(non_snake_case)]
pub mod depositSol;
#[allow(non_snake_case)]
pub mod tok22InitATA;
#[allow(non_snake_case)]
pub mod tok22InitMint;
#[allow(non_snake_case)]
pub mod tok22MintToken;
#[allow(non_snake_case)]
pub mod tokLgcInitATA;
#[allow(non_snake_case)]
pub mod tokLgcInitMint;
#[allow(non_snake_case)]
pub mod tokLgcMintToken;
#[allow(non_snake_case)]
pub mod withdrawSol;

pub use depositSol::*;
pub use tok22InitATA::*;
pub use tok22InitMint::*;
pub use tok22MintToken::*;
pub use tokLgcInitATA::*;
pub use tokLgcInitMint::*;
pub use tokLgcMintToken::*;
pub use withdrawSol::*;

use shank::ShankInstruction;

/// Shank IDL enum describes all program instructions and their required accounts.
/// Manually write this below, then run IDL generation; This below does not affect runtime behavior.
/// TODO: when is signer writable?
/// writable(to be modified):, name= signer, token_account, pda
/// non writable: program, system_program, mint
#[derive(ShankInstruction)]
pub enum ProgramIx {
    /// Deposit lamports into the vault.
    #[account(0, signer, writable, name = "owner", desc = "Vault owner and payer")]
    #[account(1, writable, name = "vault", desc = "VaultPDA")]
    #[account(2, name = "program", desc = "Program Address")]
    #[account(3, name = "system_program", desc = "System Program")]
    Deposit { amount: u64 },

    /// Withdraw lamports from the vault
    #[account(0, signer, writable, name = "owner", desc = "Vault owner + authority")]
    #[account(1, writable, name = "vault", desc = "Vault PDA")]
    #[account(2, name = "program", desc = "Program Address")]
    Withdraw { amount: u64 },

    /// TokLgc Init Mint
    #[account(0, signer, writable, name = "payer", desc = "Payer")]
    #[account(1, signer, writable, name = "mint", desc = "Mint")]
    #[account(2, name = "mint_authority", desc = "Mint Authority")]
    #[account(3, name = "token_program", desc = "Token Program")]
    #[account(4, name = "freeze_authority_opt", desc = "Freeze Authority")]
    #[account(5, name = "program", desc = "This Program")]
    #[account(6, name = "system_program", desc = "System Program")]
    TokenLgcInitMint { decimals: u8 },

    /// TokLgc Init ATA(Associated Token Acct)
    #[account(0, signer, writable, name = "payer", desc = "Payer")]
    #[account(1, name = "to_wallet", desc = "To Wallet")]
    #[account(2, name = "mint", desc = "Mint")]
    #[account(3, writable, name = "token_account", desc = "ATA Token Account")]
    #[account(4, name = "token_program", desc = "Token Program")]
    #[account(5, name = "system_program", desc = "System Program")]
    #[account(6, name = "atoken_program", desc = "AToken Program")]
    TokenLgcInitATA {},

    /// TokLgc Mint Token
    #[account(0, signer, writable, name = "mint_authority", desc = "Mint Authority")]
    #[account(1, name = "to_wallet", desc = "ToWallet")]
    #[account(2, writable, name = "mint", desc = "Mint")]
    #[account(3, writable, name = "token_account", desc = "ATA Token Account")]
    #[account(4, name = "token_program", desc = "Token Program")]
    #[account(5, name = "system_program", desc = "System Program")]
    #[account(6, name = "atoken_program", desc = "AToken Program")]
    TokLgcMintToken { decimals: u8, amount: u64 },

    /// Token2022 Init Mint
    #[account(0, signer, writable, name = "payer", desc = "Payer")]
    #[account(1, signer, writable, name = "mint", desc = "Mint")]
    #[account(2, name = "mint_authority", desc = "Mint Authority")]
    #[account(3, name = "token_program", desc = "Token Program")]
    #[account(4, name = "freeze_authority_opt", desc = "Freeze Authority")]
    #[account(5, name = "program", desc = "This Program")]
    #[account(6, name = "system_program", desc = "System Program")]
    Token2022InitMint { decimals: u8 },

    /// Token2022 Init ATA(Associated Token Acct)
    #[account(0, signer, writable, name = "payer", desc = "Payer")]
    #[account(1, name = "to_wallet", desc = "To Wallet")]
    #[account(2, name = "mint", desc = "Mint")]
    #[account(3, writable, name = "token_account", desc = "ATA Token Account")]
    #[account(4, name = "token_program", desc = "Token Program")]
    #[account(5, name = "system_program", desc = "System Program")]
    #[account(6, name = "atoken_program", desc = "AToken Program")]
    Token2022InitATA {},

    /// Token2022 Mint Token
    #[account(0, signer, writable, name = "mint_authority", desc = "Mint Authority")]
    #[account(1, name = "to_wallet", desc = "ToWallet")]
    #[account(2, writable, name = "mint", desc = "Mint")]
    #[account(3, writable, name = "token_account", desc = "ATA Token Account")]
    #[account(4, name = "token_program", desc = "Token Program")]
    #[account(5, name = "system_program", desc = "System Program")]
    #[account(6, name = "atoken_program", desc = "AToken Program")]
    Tok22MintToken { decimals: u8, amount: u64 },
} //update here and lib.rs for new functions

//-------------==
/// Parse a u64 from instruction data.
/// amount must be non-zero,
pub fn parse_u64(data: &[u8]) -> Result<u64, ProgramError> {
    // Verify the data length matches a u64 (8 bytes)
    if data.len() != core::mem::size_of::<u64>() {
        return Err(ProgramError::InvalidInstructionData);
    }
    // Convert the byte slice to a u64
    let amt = u64::from_le_bytes(data.try_into().expect("invalid_argument"));
    // let amount = u64::from_le_bytes([data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7]]);

    // Validate the amount (e.g., not zero)
    if amt == 0 {
        return Err(ProgramError::InvalidInstructionData);
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
pub fn check_mint(mint: &AccountInfo, mint_authority: &AccountInfo) -> Result<(), ProgramError> {
    let mint_info = pinocchio_token::state::Mint::from_account_info(mint)?;

    if mint_info
        .mint_authority()
        .is_some_and(|authority| !mint_authority.key().eq(authority))
    {
        return Err(ProgramError::IncorrectAuthority);
    }
    Ok(())
}
/// returns different error type than check_mint()... thus cannot be combined with it
pub fn check_mint22(mint: &AccountInfo, mint_authority: &AccountInfo) -> Result<(), ProgramError> {
    let mint_info = pinocchio_token_2022::state::Mint::from_account_info(mint)?;

    if mint_info
        .mint_authority()
        .is_some_and(|authority| !mint_authority.key().eq(authority))
    {
        return Err(ProgramError::IncorrectAuthority);
    }
    Ok(())
}
//TODO
pub fn check_tok_acct(
    account: &AccountInfo,
    user: &AccountInfo,
    mint: &AccountInfo,
) -> Result<(), ProgramError> {
    empty_lamport(account)?;
    empty_lamport(user)?;
    empty_lamport(mint)?;
    Ok(())
}
pub fn check_pda(account: &AccountInfo) -> Result<(), ProgramError> {
    if !account.is_owned_by(&crate::ID) {
        return Err(ProgramError::InvalidAccountOwner);
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
    Err(ProgramError::AccountAlreadyInitialized)
}
pub fn writable(account: &AccountInfo) -> Result<(), ProgramError> {
    if !account.is_writable() {
        return Err(ProgramError::InvalidAccountData);
    }
    Ok(())
}
pub fn executable(account: &AccountInfo) -> Result<(), ProgramError> {
    if !account.executable() {
        return Err(ProgramError::InvalidAccountData);
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
        return Err(ProgramError::InvalidArgument);
    }
    Ok(())
}
pub fn check_str_len(s: &str, min_len: usize, max_len: usize) -> Result<(), ProgramError> {
    if s.len() < min_len {
        return Err(ProgramError::AccountDataTooSmall);
    }
    if s.len() > max_len {
        return Err(ProgramError::InvalidArgument);
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
