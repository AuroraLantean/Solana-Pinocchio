// instructions.rs
use core::convert::TryFrom;
use core::mem::size_of;
use pinocchio::{
    account_info::AccountInfo,
    instruction::{Seed, Signer},
    program_error::ProgramError,
    pubkey::{try_find_program_address, Pubkey},
    sysvars::{rent::Rent, Sysvar},
    ProgramResult,
};
use pinocchio_log::log;
use pinocchio_system::instructions::{CreateAccount, Transfer as SystemTransfer};
//use pinocchio_token::instructions::InitializeMint2;
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

// Deposit SOL to program PDA
// make and rent-funds the vault PDA
// check the PDA exists and is owned by the program
// transfer the SOL amount to the vault
pub struct Deposit<'a> {
    pub owner: &'a AccountInfo,
    pub vault: &'a AccountInfo,
    pub amount: u64,
}
impl<'a> Deposit<'a> {
    pub const DISCRIMINATOR: &'a u8 = &0;

    pub fn process(self) -> ProgramResult {
        let Deposit {
            owner,
            vault,
            amount,
        } = self;

        ensure_deposit_accounts(owner, vault)?;

        SystemTransfer {
            from: owner,
            to: vault,
            lamports: amount,
        }
        .invoke()?;
        log!("{} Lamports deposited to vault", amount);
        Ok(())
    }
}
impl<'a> TryFrom<(&'a [u8], &'a [AccountInfo])> for Deposit<'a> {
    type Error = ProgramError;

    fn try_from(value: (&'a [u8], &'a [AccountInfo])) -> Result<Self, Self::Error> {
        let (data, accounts) = value;
        if accounts.len() < 2 {
            return Err(ProgramError::NotEnoughAccountKeys);
        }
        let owner = &accounts[0];
        let vault = &accounts[1];
        //let [owner, vault, _system_program, _] = accounts else {..}

        let amount = parse_amount_u64(data)?;
        Ok(Self {
            owner,
            vault,
            amount,
        })
    }
}

//  vault is owned by the program, matches the PDA derived from the owner, and the owner is the signer of the withdraw transaction. The withdrawn amount is everything above the rent minimum.
pub struct Withdraw<'a> {
    pub owner: &'a AccountInfo,
    pub vault: &'a AccountInfo,
    pub amount: u64,
}
impl<'a> Withdraw<'a> {
    pub const DISCRIMINATOR: &'a u8 = &1;

    /// Transfer lamports from the vault PDA to the owner, leaving the rent minimum in place.
    pub fn process(self) -> ProgramResult {
        let Withdraw {
            owner,
            vault,
            amount,
        } = self;
        // Validate owner is signer
        check_signer(owner)?;

        // Validate the vault is owned by the program
        check_pda(vault)?;

        // Validate the vault is the correct PDA for this owner
        let (expected_vault_pda, _bump) = derive_vault_pda(owner)?;
        if vault.key() != &expected_vault_pda {
            return Err(ProgramError::InvalidAccountData);
        }

        // Compute how much can be withdrawn while keeping the account rent-exempt
        let data_len = vault.data_len();
        let min_balance = Rent::get()?.minimum_balance(data_len);

        log!("withdraw amt: {}", amount);
        let current = vault.lamports();
        log!("vault balc: {}", current);
        if current <= min_balance.checked_add(amount).unwrap() {
            return Err(ProgramError::InsufficientFunds);
        }

        // Transfer SOL from vault to owner
        {
            let mut vault_lamports = vault.try_borrow_mut_lamports()?;
            *vault_lamports = vault_lamports
                .checked_sub(amount)
                .ok_or(ProgramError::InsufficientFunds)?;
        }

        {
            let mut owner_lamports = owner.try_borrow_mut_lamports()?;
            *owner_lamports = owner_lamports
                .checked_add(amount)
                .ok_or(ProgramError::InsufficientFunds)?;
        }
        log!("{} lamports withdrawn from vault", amount);
        Ok(())
    }
}
/**impl<'a> TryFrom<(&'a [u8], &'a [AccountInfo])> for Deposit<'a> {
   type Error = ProgramError;

   fn try_from(value: (&'a [u8], &'a [AccountInfo])) -> Result<Self, Self::Error> {
       let (data, accounts) = value;
*
*/
impl<'a> TryFrom<(&'a [u8], &'a [AccountInfo])> for Withdraw<'a> {
    type Error = ProgramError;

    fn try_from(value: (&'a [u8], &'a [AccountInfo])) -> Result<Self, Self::Error> {
        let (data, accounts) = value;
        if accounts.len() < 2 {
            return Err(ProgramError::NotEnoughAccountKeys);
        }
        let owner = &accounts[0];
        let vault = &accounts[1];

        let amount = parse_amount_u64(data)?;
        Ok(Self {
            owner,
            vault,
            amount,
        })
    }
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
    let amt = u64::from_le_bytes(data.try_into().unwrap());
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
/// Ensure the vault exists; if not, create it with PDA seeds. owner must be a signer, vault must be writable, and rent minimum must be respected for creation.
fn ensure_deposit_accounts(owner: &AccountInfo, vault: &AccountInfo) -> ProgramResult {
    check_signer(owner)?;

    // Create when empty and fund rent-exempt.
    if vault.lamports() == 0 {
        const ACCOUNT_DISCRIMINATOR_SIZE: usize = 8;

        let (expected_vault_pda, bump) = derive_vault_pda(owner)?;
        if vault.key() != &expected_vault_pda {
            return Err(ProgramError::InvalidAccountData);
        }
        //assert_eq!(&expected_vault_pda, vault.key());

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
        check_pda(vault)?;
        log!("Vault already exists");
    }
    Ok(())
}
