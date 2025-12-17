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

    //reads the first byte as a discriminator to determine which method to call (here: 0 = DepositSol, 1 = WithdrawSol).
    match discriminator {
        DepositSol::DISCRIMINATOR => DepositSol::try_from((data, accounts))?.process(),
        WithdrawSol::DISCRIMINATOR => WithdrawSol::try_from((data, accounts))?.process(),
        TokenLgcInitMint::DISCRIMINATOR => TokenLgcInitMint::try_from((data, accounts))?.process(),
        TokenLgcInitAta::DISCRIMINATOR => TokenLgcInitAta::try_from((data, accounts))?.process(),
        TokLgcMintToken::DISCRIMINATOR => TokLgcMintToken::try_from((data, accounts))?.process(),
        TokLgcDeposit::DISCRIMINATOR => TokLgcDeposit::try_from((data, accounts))?.process(),
        TokLgcWithdraw::DISCRIMINATOR => TokLgcWithdraw::try_from((data, accounts))?.process(),
        Token2022InitMint::DISCRIMINATOR => {
            Token2022InitMint::try_from((data, accounts))?.process()
        }
        Token2022InitAta::DISCRIMINATOR => Token2022InitAta::try_from((data, accounts))?.process(),
        Token2022MintToken::DISCRIMINATOR => {
            Token2022MintToken::try_from((data, accounts))?.process()
        }
        _ => Err(ProgramError::InvalidArgument),
    }
}
