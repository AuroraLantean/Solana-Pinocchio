use core::convert::TryFrom;
use pinocchio::{account_info::AccountInfo, program_error::ProgramError, ProgramResult};
use pinocchio_log::log;

use crate::{instructions::check_signer, parse_u64, rent_exempt, writable};
use pinocchio_token::{
    instructions::MintToChecked,
    state::{Mint, TokenAccount},
};

/// TokLgc Mint Tokens
pub struct TokLgcMintToken<'a> {
    pub mint_authority: &'a AccountInfo, //signer
    pub mint: &'a AccountInfo,
    pub to_wallet: &'a AccountInfo,
    pub token_program: &'a AccountInfo,
    pub system_program: &'a AccountInfo,
    pub token_account: &'a AccountInfo,
    pub decimals: u8,
    pub amount: u64,
}
impl<'a> TokLgcMintToken<'a> {
    pub const DISCRIMINATOR: &'a u8 = &4;

    pub fn process(self) -> ProgramResult {
        let TokLgcMintToken {
            mint_authority,
            mint,
            to_wallet,
            token_program,
            system_program,
            token_account,
            decimals,
            amount,
        } = self;
        log!("TokLgcMintToken process()");
        check_signer(mint_authority)?;
        log!("TokLgcMintToken 1");
        rent_exempt(mint, 0)?;
        //writable(mint)?;
        writable(token_account)?;

        if !token_program.executable() {
            return Err(ProgramError::IncorrectProgramId);
        }
        log!("TokLgcMintToken 3");
        if !mint.is_owned_by(token_program.key()) {
            return Err(ProgramError::InvalidAccountData);
        }
        let mint_info = Mint::from_account_info(mint)?;
        if mint_info
            .mint_authority()
            .is_some_and(|authority| !mint_authority.key().eq(authority))
        {
            return Err(ProgramError::IncorrectAuthority);
        }
        log!("TokLgcMintToken 5");
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
        } else {
            log!("token_account has data");
            let token_account_info = TokenAccount::from_account_info(token_account)?;
            if !token_account_info.owner().eq(to_wallet.key()) {
                return Err(ProgramError::InvalidAccountData);
            }
        }
        log!("Token Account exists");
        rent_exempt(token_account, 1)?;

        log!("Mint Tokens");
        MintToChecked {
            mint,
            account: token_account,
            mint_authority,
            amount,
            decimals,
        }
        .invoke()?;
        Ok(())
    }
}
impl<'a> TryFrom<(&'a [u8], &'a [AccountInfo])> for TokLgcMintToken<'a> {
    type Error = ProgramError;

    fn try_from(value: (&'a [u8], &'a [AccountInfo])) -> Result<Self, Self::Error> {
        log!("TokLgcMintToken try_from");
        let (data, accounts) = value;
        log!("accounts len: {}, data len: {}", accounts.len(), data.len());

        let [mint_authority, mint, to_wallet, token_program, system_program, token_account] =
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
            mint,
            to_wallet,
            token_program,
            system_program,
            token_account,
            decimals,
            amount,
        })
    }
}
/*Transfer mint_x from user ata to vault
      pinocchio_token::instructions::Transfer {
          from: maker_ata,
          to: vault,
          authority: maker,
          amount: unsafe { *(data.as_ptr().add(1 + 8) as *const u64)},
      }.invoke()?;

//----------==
  pinocchio_token::instructions::Transfer {
      from: vault,
      to: taker_ata_x,
      authority: escrow,
      amount: vault_account.amount(),
  }.invoke_signed(&[seeds.clone()])?;

  pinocchio_token::instructions::CloseAccount {
      account: vault,
      destination: maker,
      authority: escrow,
  }.invoke_signed(&[seeds])?;
     */
