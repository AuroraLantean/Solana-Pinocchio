use core::convert::TryFrom;
use pinocchio::{account_info::AccountInfo, program_error::ProgramError, ProgramResult};
use pinocchio_log::log;

use crate::{
    empty_data, empty_lamport, executable, instructions::check_signer, rent_exempt, writable,
};

/// Token2022 Init ATA(Associated Token Account)
pub struct Token2022InitAta<'a> {
    pub payer: &'a AccountInfo,
    pub to_wallet: &'a AccountInfo,
    pub mint: &'a AccountInfo,
    pub token_account: &'a AccountInfo,
    pub token_program: &'a AccountInfo,
    pub system_program: &'a AccountInfo,
    pub atoken_program: &'a AccountInfo,
}
impl<'a> Token2022InitAta<'a> {
    pub const DISCRIMINATOR: &'a u8 = &52;

    pub fn process(self) -> ProgramResult {
        let Token2022InitAta {
            payer,
            to_wallet,
            mint,
            token_account,
            token_program,
            system_program,
            atoken_program: _,
        } = self;
        log!("Token2022InitAta process()");
        check_signer(payer)?;
        executable(token_program)?;

        log!("Token2022InitAta 1");
        rent_exempt(mint, 0)?;
        //writable(mint)?;//Shank IDL definition

        log!("Token2022InitAta 2");
        empty_lamport(token_account)?;
        empty_data(token_account)?;
        writable(token_account)?;

        log!("Init ATA Token Account");
        pinocchio_associated_token_account::instructions::Create {
            funding_account: payer, // Keypair
            account: token_account,
            wallet: to_wallet,
            mint: mint,
            system_program: system_program,
            token_program: token_program,
        }
        .invoke()?;
        /*pinocchio_token_2022::instructions::InitializeAccount3 {
            account: token_account,
            mint: mint,
            owner: to_wallet.key(),
            token_program: token_program.key(),
        }
        .invoke()?;//invalid account data for instruction*/
        Ok(())
    }
    pub fn init_if_needed(self) -> ProgramResult {
        match empty_lamport(self.token_account) {
            Ok(_) => Self::process(self),
            Err(_) => Ok(()),
        }
    }
}
impl<'a> TryFrom<(&'a [u8], &'a [AccountInfo])> for Token2022InitAta<'a> {
    type Error = ProgramError;

    fn try_from(value: (&'a [u8], &'a [AccountInfo])) -> Result<Self, Self::Error> {
        log!("Token2022InitAta try_from");
        let (data, accounts) = value;
        log!("accounts len: {}, data len: {}", accounts.len(), data.len());

        let [payer, to_wallet, mint, token_account, token_program, system_program, atoken_program] =
            accounts
        else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };
        Ok(Self {
            payer,
            to_wallet,
            mint,
            token_account,
            token_program,
            system_program,
            atoken_program,
        })
    }
}
/*find token_account from to_wallet & token mint"
cargo add spl-associated-token-account
use spl_associated_token_account::get_associated_token_address;
let ata = get_associated_token_address(&wallet, &mint); */
