use crate::{empty_data, empty_lamport, instructions::check_signer, rent_exempt};
use core::convert::TryFrom;
use pinocchio::{account_info::AccountInfo, program_error::ProgramError, ProgramResult};
use pinocchio_log::log;

/// Token Legacy Init Token Account: not working in test... but we do not need this function because MintToChecked() will auto make such account!
pub struct TokenLgcInitTokAcct<'a> {
    pub payer: &'a AccountInfo,
    pub to_wallet: &'a AccountInfo,
    pub mint: &'a AccountInfo,
    pub token_account: &'a AccountInfo,
    pub token_program: &'a AccountInfo,
    pub system_program: &'a AccountInfo,
    pub bump: u8,
}
impl<'a> TokenLgcInitTokAcct<'a> {
    pub const DISCRIMINATOR: &'a u8 = &3;

    pub fn process(self) -> ProgramResult {
        let TokenLgcInitTokAcct {
            payer, //signer
            to_wallet,
            mint,
            token_account,
            token_program,
            system_program,
            bump: _,
        } = self;
        log!("TokenLgcInitTokAcct process()");
        check_signer(payer)?;
        empty_lamport(token_account)?;
        empty_data(token_account)?;
        rent_exempt(mint, 0)?;

        /*find token_account from to_wallet & token mint"
        cargo add spl-associated-token-account
        use spl_associated_token_account::get_associated_token_address;
        let ata = get_associated_token_address(&to_wallet, &mint);*/

        log!("Make Token Account");
        pinocchio_associated_token_account::instructions::Create {
            funding_account: payer, // Keypair
            account: token_account,
            wallet: to_wallet, //tried addr and Kp
            mint: mint,
            system_program: system_program,
            token_program: token_program,
        }
        .invoke()?;
        //.invoke_signed(&[signer])?;An account required by the instruction is missin
        //.invoke()?;

        /*CreateAccount {
            from: payer,
            to: token_account,
            owner: token_program.key(),
            lamports,
            space,
        }
        .invoke()?;
        log!("TokenLgcInitTokAcct 7");
        writable(token_account)?;

        log!("Init Token Account");
        InitializeAccount3 {
            account: token_account,
            mint: mint,
            owner: to_wallet.key(),
        };*/
        Ok(())
    }
    pub fn init_if_needed(self) -> ProgramResult {
        match empty_lamport(self.token_account) {
            Ok(_) => Self::process(self),
            Err(_) => Ok(()),
        }
    }
}
impl<'a> TryFrom<(&'a [u8], &'a [AccountInfo])> for TokenLgcInitTokAcct<'a> {
    type Error = ProgramError;

    fn try_from(value: (&'a [u8], &'a [AccountInfo])) -> Result<Self, Self::Error> {
        log!("TokenLgcInitTokAcct try_from");
        let (data, accounts) = value;
        log!("accounts len: {}, data len: {}", accounts.len(), data.len());

        if accounts.len() < 6 {
            return Err(ProgramError::NotEnoughAccountKeys);
        }
        let payer = &accounts[0];
        let to_wallet = &accounts[1];
        let mint = &accounts[2];
        let token_account = &accounts[3];
        let token_program = &accounts[4];
        let _this_program = &accounts[5];
        let system_program = &accounts[6];
        //let [payer, mint, _] = accounts else {  does not work };

        //1+8: u8 takes 1, u64 takes 8 bytes
        if data.len() < 1 {
            return Err(ProgramError::AccountDataTooSmall);
        }
        let bump = data[0];
        log!("bump: {}", bump);

        log!("TokenLgcInitTokAcct try_from end");
        Ok(Self {
            payer,
            to_wallet,
            mint,
            token_account,
            token_program,
            system_program,
            bump,
        })
    }
}
