use core::convert::TryFrom;
use pinocchio::{
    account_info::AccountInfo,
    program_error::ProgramError,
    sysvars::{rent::Rent, Sysvar},
    ProgramResult,
};
use pinocchio_log::log;
use pinocchio_system::instructions::CreateAccount;

use crate::{empty_lamport, instructions::check_signer, rent_exempt, writable};
use pinocchio_token_2022::{instructions::InitializeAccount3, state::TokenAccount};

/// Token2022 Init Token Account
pub struct Token2022InitTokAcct<'a> {
    pub payer: &'a AccountInfo,
    pub tok_acc_owner: &'a AccountInfo,
    pub mint: &'a AccountInfo,
    pub token_account: &'a AccountInfo,
    pub token_program: &'a AccountInfo,
}
impl<'a> Token2022InitTokAcct<'a> {
    pub const DISCRIMINATOR: &'a u8 = &4;

    pub fn process(self) -> ProgramResult {
        let Token2022InitTokAcct {
            payer,
            tok_acc_owner,
            mint,
            token_account,
            token_program,
        } = self;
        check_signer(payer)?;
        empty_lamport(token_account)?;
        rent_exempt(mint, 0)?;
        /*find token_account from tok_acc_owner & token mint"
        cargo add spl-associated-token-account
        use spl_associated_token_account::get_associated_token_address;
        let ata = get_associated_token_address(&wallet, &mint);
        */
        log!("Make Token Account");
        CreateAccount {
            from: payer,
            to: token_account,
            owner: token_program.key(),
            lamports: Rent::get()?.minimum_balance(TokenAccount::BASE_LEN),
            space: TokenAccount::BASE_LEN as u64,
        }
        .invoke()?;

        writable(token_account)?;

        log!("Init Token Account");
        InitializeAccount3 {
            account: token_account,
            mint: mint,
            owner: tok_acc_owner.key(),
            token_program: token_program.key(),
        };
        Ok(())
    }
    pub fn init_if_needed(self) -> ProgramResult {
        match empty_lamport(self.token_account) {
            Ok(_) => Self::process(self),
            Err(_) => Ok(()),
        }
    }
}
impl<'a> TryFrom<(&'a [u8], &'a [AccountInfo])> for Token2022InitTokAcct<'a> {
    type Error = ProgramError;

    fn try_from(value: (&'a [u8], &'a [AccountInfo])) -> Result<Self, Self::Error> {
        let (data, accounts) = value;

        let [payer, tok_acc_owner, mint, token_account, token_program, _] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        if data.len() < 1 {
            return Err(ProgramError::AccountDataTooSmall);
        }

        Ok(Self {
            payer,
            tok_acc_owner,
            mint,
            token_account,
            token_program,
        })
    }
}
