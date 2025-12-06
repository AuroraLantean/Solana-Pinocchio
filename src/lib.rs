/*lib.rs serves as your program’s entrypoint
- takes in the program ID, accounts, and instruction data, then reads the first byte as a discriminator to determine which method to call (for example, 0 = Deposit, 1 = Withdraw).
 *
 *
 */
#![no_std]

use pinocchio::{
    account_info::AccountInfo, entrypoint, program_error::ProgramError, pubkey::Pubkey,
    ProgramResult,
};
use pinocchio_pubkey::declare_id;

entrypoint!(process_instruction);

pub mod instructions;
pub use instructions::*;

declare_id!("7EKqBVYSCmJbt2T8tGSmwzNKnpL29RqcJcyUr9aEEr6e"); //crate::ID

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if program_id != &crate::ID {
        return Err(ProgramError::IncorrectProgramId);
    }
    // `split_first` separates the first byte (discriminator) from the rest (payload).
    let (discriminator, data) = instruction_data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    //reads the first byte as a discriminator to determine which method to call (here: 0 = Deposit, 1 = Withdraw).
    match discriminator {
        Deposit::DISCRIMINATOR => Deposit::try_from((data, accounts))?.process(),
        Withdraw::DISCRIMINATOR => Withdraw::try_from((data, accounts))?.process(),
        _ => Err(ProgramError::InvalidArgument),
    }
}
