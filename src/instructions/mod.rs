use pinocchio::{
    account_info::AccountInfo,
    program_error::ProgramError,
    pubkey::{try_find_program_address, Pubkey},
};
#[allow(non_snake_case)]
pub mod depositSol;
pub mod tok22initmint;
#[allow(non_snake_case)]
pub mod withdrawSol;

pub use depositSol::*;
pub use tok22initmint::*;
pub use withdrawSol::*;

use shank::ShankInstruction;

/// Shank IDL enum describes all program instructions and their required accounts.
/// This is used only for IDL generation and does not affect runtime behavior.
#[derive(ShankInstruction)]
pub enum ProgramIx {
    /// Deposit lamports into the vault.
    #[account(0, signer, writable, name = "owner", desc = "Vault owner and payer")]
    #[account(1, writable, name = "vault", desc = "Vault PDA for lamports")]
    #[account(2, name = "program", desc = "Program Address")]
    #[account(3, name = "system_program", desc = "System Program Address")]
    Deposit { amount: u64 },

    /// Withdraw lamports from the vault
    #[account(0, signer, writable, name = "owner", desc = "Vault owner + authority")]
    #[account(1, writable, name = "vault", desc = "Vault PDA for lamports")]
    #[account(2, name = "program", desc = "Program Address")]
    Withdraw { amount: u64 },
}

//-------------==
/// Parse a u64 from instruction data.
/// amount must be non-zero,
fn parse_amount_u64(data: &[u8]) -> Result<u64, ProgramError> {
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

/// Derive the vault PDA for an owner -> (pda, bump)
fn derive_vault_pda(owner: &AccountInfo) -> Result<(Pubkey, u8), ProgramError> {
    //find_program_address(&[b"vault", owner.key().as_ref()], &crate::ID)
    // let (pda, _bump) =
    try_find_program_address(&[b"vault", owner.key().as_ref()], &crate::ID)
        .ok_or(ProgramError::InvalidSeeds)
}
fn check_signer(account: &AccountInfo) -> Result<(), ProgramError> {
    if !account.is_signer() {
        return Err(ProgramError::InvalidAccountOwner);
    }
    Ok(())
}
fn check_pda(account: &AccountInfo) -> Result<(), ProgramError> {
    if !account.is_owned_by(&crate::ID) {
        return Err(ProgramError::InvalidAccountOwner);
    }
    Ok(())
}
fn check_str_len(s: &str, min_len: usize, max_len: usize) -> Result<(), ProgramError> {
    if s.len() < min_len {
        return Err(ProgramError::AccountDataTooSmall);
    }
    if s.len() > max_len {
        return Err(ProgramError::InvalidArgument);
    }
    Ok(())
}
