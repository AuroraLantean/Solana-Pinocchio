// instructions.rs
use core::convert::TryFrom;
use core::mem::size_of;
use pinocchio::{
    account_info::AccountInfo,
    instruction::{Seed, Signer},
    program_error::ProgramError,
    pubkey::{find_program_address, Pubkey},
    sysvars::{rent::Rent, Sysvar},
    ProgramResult,
};
use pinocchio_log::log;
use pinocchio_system::instructions::{CreateAccount, Transfer as SystemTransfer};
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

    /// Withdraw all lamports from the vault back to the owner.
    #[account(
        0,
        signer,
        writable,
        name = "owner",
        desc = "Vault owner and authority"
    )]
    #[account(1, writable, name = "vault", desc = "Vault PDA for lamports")]
    #[account(2, name = "program", desc = "Program Address")]
    Withdraw {},
}

//-------------==
/// Parse a u64 from instruction data.
/// amount must be non-zero,
fn parse_amount(data: &[u8]) -> Result<u64, ProgramError> {
    if data.len() != core::mem::size_of::<u64>() {
        return Err(ProgramError::InvalidInstructionData);
    }
    let amt = u64::from_le_bytes(data.try_into().unwrap());
    if amt == 0 {
        return Err(ProgramError::InvalidInstructionData);
    }
    Ok(amt)
}

/// Derive the vault PDA for an owner and return (pda, bump).
fn derive_vault(owner: &AccountInfo) -> (Pubkey, u8) {
    find_program_address(&[b"vault", owner.key().as_ref()], &crate::ID)
}

/// Ensure the vault exists; if not, create it with PDA seeds.
/// owner must be a signer, vault must be writable, and rent minimum must be respected for creation.
fn ensure_vault_exists(owner: &AccountInfo, vault: &AccountInfo) -> ProgramResult {
    if !owner.is_signer() {
        return Err(ProgramError::InvalidAccountOwner);
    }

    // Create when empty and fund rent-exempt.
    if vault.lamports() == 0 {
        const ACCOUNT_DISCRIMINATOR_SIZE: usize = 8;

        let (_pda, bump) = derive_vault(owner);
        let signer_seeds = [
            Seed::from(b"vault".as_slice()),
            Seed::from(owner.key().as_ref()),
            Seed::from(core::slice::from_ref(&bump)),
        ];
        let signer = Signer::from(&signer_seeds);

        // Make the account rent-exempt.
        const VAULT_SIZE: usize = ACCOUNT_DISCRIMINATOR_SIZE + size_of::<u64>();
        let needed_lamports = Rent::get()?.minimum_balance(VAULT_SIZE);

        CreateAccount {
            from: owner,
            to: vault,
            lamports: needed_lamports,
            space: VAULT_SIZE as u64,
            owner: &crate::ID,
        }
        .invoke_signed(&[signer])?;

        log!("Vault created");
    } else {
        // If vault already exists, validate owner matches the program.
        if !vault.is_owned_by(&crate::ID) {
            return Err(ProgramError::InvalidAccountOwner);
        }
        log!("Vault already exists");
    }

    Ok(())
}
