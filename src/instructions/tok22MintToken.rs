use core::convert::TryFrom;
use pinocchio::{account_info::AccountInfo, program_error::ProgramError, ProgramResult};
use pinocchio_log::log;
use pinocchio_token_2022::state::TokenAccount;

use crate::{
    check_mint22, executable, instructions::check_signer, parse_u64, rent_exempt, writable,
};

/// Token2022 Mint Tokens
pub struct Token2022MintToken<'a> {
    pub mint_authority: &'a AccountInfo, //signer
    pub to_wallet: &'a AccountInfo,
    pub mint: &'a AccountInfo,
    pub token_account: &'a AccountInfo,
    pub token_program: &'a AccountInfo,
    pub system_program: &'a AccountInfo,
    pub atoken_program: &'a AccountInfo,
    pub decimals: u8,
    pub amount: u64,
}
impl<'a> Token2022MintToken<'a> {
    pub const DISCRIMINATOR: &'a u8 = &7;

    pub fn process(self) -> ProgramResult {
        let Token2022MintToken {
            mint_authority,
            to_wallet,
            mint,
            token_account,
            token_program,
            system_program,
            atoken_program: _,
            decimals,
            amount,
        } = self;
        log!("Token2022MintToken process()");
        check_signer(mint_authority)?;
        executable(token_program)?;

        log!("Token2022MintToken 1");
        rent_exempt(mint, 0)?;
        log!("Token2022MintToken 2");
        writable(mint)?;
        log!("Token2022MintToken 3");
        check_mint22(mint, mint_authority)?;

        log!("Token2022MintToken 4");
        if !mint.is_owned_by(token_program.key()) {
            return Err(ProgramError::InvalidAccountData);
        }

        log!("Token2022MintToken 5");
        if !system_program.key().eq(&pinocchio_system::ID) {
            return Err(ProgramError::IncorrectProgramId);
        }

        if token_account.data_is_empty() {
            log!("Make token_account");
            pinocchio_associated_token_account::instructions::Create {
                funding_account: mint_authority,
                account: token_account,
                wallet: to_wallet,
                mint,
                system_program,
                token_program,
            }
            .invoke()?;
            //Please upgrade to SPL Token 2022 for immutable owner support
        } else {
            log!("token_account has data");
            let token_account_info = TokenAccount::from_account_info(token_account)?;
            if !token_account_info.owner().eq(to_wallet.key()) {
                return Err(ProgramError::InvalidAccountData);
            }
        }
        writable(token_account)?;
        rent_exempt(token_account, 1)?;
        log!("Token Account found/verified");

        log!("Mint Tokens");
        pinocchio_token_2022::instructions::MintToChecked {
            mint,
            account: token_account,
            mint_authority,
            amount,
            decimals,
            token_program: token_program.key(),
        }
        .invoke()?;
        Ok(())
    }
}
impl<'a> TryFrom<(&'a [u8], &'a [AccountInfo])> for Token2022MintToken<'a> {
    type Error = ProgramError;

    fn try_from(value: (&'a [u8], &'a [AccountInfo])) -> Result<Self, Self::Error> {
        log!("Token2022MintToken try_from");
        let (data, accounts) = value;
        log!("accounts len: {}, data len: {}", accounts.len(), data.len());

        let [mint_authority, to_wallet, mint, token_account, token_program, system_program, atoken_program] =
            accounts
        else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        //1+8: u8 takes 1, u64 takes 8 bytes
        if data.len() < 9 {
            return Err(ProgramError::AccountDataTooSmall);
        }
        let decimals = data[0];
        let amount = parse_u64(&data[1..])?;
        log!("decimals: {}, amount: {}", decimals, amount);

        Ok(Self {
            mint_authority,
            to_wallet,
            mint,
            token_account,
            token_program,
            system_program,
            atoken_program,
            decimals,
            amount,
        })
    }
}
