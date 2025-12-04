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

declare_id!("J7KVuhnkyabChZs2r7wLVduZvJr4GiSGTZt3b3dJyykJ"); //crate::ID

fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    match instruction_data.split_first() {
        Some((Deposit::DISCRIMINATOR, data)) => Deposit::try_from((data, accounts))?.process(),
        Some((Withdraw::DISCRIMINATOR, _)) => Withdraw::try_from(accounts)?.process(),
        _ => Err(ProgramError::InvalidInstructionData),
    }
}
