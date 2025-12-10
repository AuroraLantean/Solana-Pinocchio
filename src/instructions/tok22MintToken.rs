use core::convert::TryFrom;
use pinocchio::{account_info::AccountInfo, program_error::ProgramError, ProgramResult};
use pinocchio_log::log;

use crate::{empty_data, executable, instructions::check_signer, parse_u64, rent_exempt, writable};
use pinocchio_token_2022::instructions::MintToChecked;

/// Token2022 Mint Tokens
pub struct Token2022MintToken<'a> {
    pub mint_account: &'a AccountInfo,
    pub token_account: &'a AccountInfo,
    pub mint_authority: &'a AccountInfo, //signer
    pub token_program: &'a AccountInfo,
    pub decimals: u8,
    pub amount: u64,
}
impl<'a> Token2022MintToken<'a> {
    pub const DISCRIMINATOR: &'a u8 = &4;

    pub fn process(self) -> ProgramResult {
        let Token2022MintToken {
            mint_account,
            token_account,
            mint_authority,
            token_program,
            decimals,
            amount,
        } = self;
        log!("decimals: {}, amount: {}", decimals, amount);

        check_signer(mint_authority)?;
        rent_exempt(mint_account, 0)?;
        rent_exempt(token_account, 1)?;
        empty_data(mint_account)?;
        writable(token_account)?;
        executable(token_program)?;

        log!("Mint Tokens");
        MintToChecked {
            mint: mint_account,
            account: token_account,
            mint_authority: mint_authority,
            amount,
            decimals,
            token_program: token_program.key(),
        };
        Ok(())
    }
}
impl<'a> TryFrom<(&'a [u8], &'a [AccountInfo])> for Token2022MintToken<'a> {
    type Error = ProgramError;

    fn try_from(value: (&'a [u8], &'a [AccountInfo])) -> Result<Self, Self::Error> {
        let (data, accounts) = value;

        let [mint_account, token_account, mint_authority, token_program, _] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        if data.len() < 1 {
            return Err(ProgramError::AccountDataTooSmall);
        }
        let decimals = data[0];
        let amount = parse_u64(&data[1..])?;
        Ok(Self {
            mint_account,
            token_account,
            mint_authority,
            token_program,
            decimals,
            amount,
        })
    }
}
