use core::convert::TryFrom;
use pinocchio::{
    account_info::AccountInfo,
    instruction::{Seed, Signer},
    program_error::ProgramError,
    sysvars::{rent::Rent, Sysvar},
    ProgramResult,
};
use pinocchio_log::log;
use pinocchio_system::instructions::CreateAccount;

use crate::{derive_pda1, empty_data, empty_lamport, instructions::check_signer, writable};
use pinocchio_token::{instructions::InitializeMint2, state::Mint};

//Initiate TokenLgc Mint Account
pub struct TokenLgcInitMint<'a> {
    pub payer: &'a AccountInfo, //signer
    pub mint: &'a AccountInfo,
    pub mint_authority: &'a AccountInfo,
    pub token_program: &'a AccountInfo,
    pub freeze_authority_opt: Option<&'a [u8; 32]>, // or Pubkey
    pub decimals: u8,
}
impl<'a> TokenLgcInitMint<'a> {
    pub const DISCRIMINATOR: &'a u8 = &2;

    pub fn process(self) -> ProgramResult {
        let TokenLgcInitMint {
            payer,
            mint_authority,
            mint,
            token_program,
            freeze_authority_opt,
            decimals,
        } = self;
        log!("TokenLgcInitMint process()");
        check_signer(mint_authority)?;
        log!("TokenLgcInitMint 2");
        empty_lamport(mint)?;
        log!("TokenLgcInitMint 3");
        empty_data(mint)?;

        if decimals > 18 {
            return Err(ProgramError::InvalidArgument);
        }
        log!("TokenLgcInitMint 4");
        /*let toklgc = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        .as_bytes()
        .try_into()
        .expect("token addr");*/

        log!("TokenLgcInitMint 5");
        let lamports = Rent::get()?.minimum_balance(Mint::LEN);
        log!("TokenLgcInitMint 6");
        let space = Mint::LEN as u64;
        log!("lamports: {}, space: {}", lamports, space);
        log!("payer: {}", payer.key());
        log!("mint: {}", mint.key());
        log!("token_program: {}", token_program.key());
        //let mint = Keypair::new();
        let (mint_exp, bump) = derive_pda1(payer, b"mint")?;
        if mint.key() != &mint_exp {
            return Err(ProgramError::InvalidAccountData);
        }
        let signer_seeds = [
            Seed::from(b"mint".as_slice()),
            Seed::from(payer.key().as_ref()),
            Seed::from(core::slice::from_ref(&bump)),
        ];
        let signer = Signer::from(&signer_seeds);

        log!("Make Mint Account");
        CreateAccount {
            from: payer,
            to: mint,
            //owner: &crate::ID,
            owner: token_program.key(),
            lamports,
            space,
        }
        .invoke_signed(&[signer])?;

        log!("TokenLgcInitMint 7");
        writable(mint)?;

        log!("Init Mint");
        InitializeMint2 {
            mint,
            decimals,
            mint_authority: mint_authority.key(),
            freeze_authority: freeze_authority_opt,
        }
        .invoke()?;
        Ok(())
    }
    pub fn init_if_needed(self) -> ProgramResult {
        match empty_lamport(self.mint) {
            Ok(_) => Self::process(self),
            Err(_) => Ok(()),
        }
    }
}
impl<'a> TryFrom<(&'a [u8], &'a [AccountInfo])> for TokenLgcInitMint<'a> {
    type Error = ProgramError;

    fn try_from(value: (&'a [u8], &'a [AccountInfo])) -> Result<Self, Self::Error> {
        log!("TokenLgcInitMint try_from");
        let (data, accounts) = value;
        log!("accounts len: {}, data len: {}", accounts.len(), data.len());

        if accounts.len() < 4 {
            return Err(ProgramError::NotEnoughAccountKeys);
        }
        let payer = &accounts[0];
        let mint = &accounts[1];
        let mint_authority = &accounts[2];
        let token_program = &accounts[3];
        let freeze_authority_opt: Option<&'a [u8; 32]>;
        if accounts.len() > 4 {
            freeze_authority_opt = Some(&accounts[4].key());
        } else {
            freeze_authority_opt = None;
        }

        if data.len() < 1 {
            return Err(ProgramError::AccountDataTooSmall);
        }
        let decimals = data[0];
        log!("decimals: {}", decimals);
        //let mint_authority = &data[1..];
        Ok(Self {
            payer,
            mint_authority, //.try_into()
            mint,
            token_program,
            freeze_authority_opt,
            decimals,
        })
    }
}
