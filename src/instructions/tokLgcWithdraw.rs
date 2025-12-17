use core::convert::TryFrom;
use pinocchio::{
    account_info::AccountInfo,
    instruction::{Seed, Signer},
    program_error::ProgramError,
    ProgramResult,
};
use pinocchio_log::log;

use crate::{
    check_ata, check_decimals, check_sysprog, derive_pda1, executable, instructions::check_signer,
    parse_u64, rent_exempt, writable,
};

/// TokLgc: Users to Withdraw Tokens
pub struct TokLgcWithdraw<'a> {
    pub user: &'a AccountInfo, //signer
    pub from_ata: &'a AccountInfo,
    pub to_ata: &'a AccountInfo,
    pub from_wallet: &'a AccountInfo,
    pub mint: &'a AccountInfo,
    pub token_program: &'a AccountInfo,
    pub system_program: &'a AccountInfo,
    pub atoken_program: &'a AccountInfo,
    pub decimals: u8,
    pub amount: u64,
}
impl<'a> TokLgcWithdraw<'a> {
    pub const DISCRIMINATOR: &'a u8 = &6;

    pub fn process(self) -> ProgramResult {
        let TokLgcWithdraw {
            user,
            from_ata,
            to_ata,
            from_wallet,
            mint,
            token_program,
            system_program,
            atoken_program: _,
            decimals,
            amount,
        } = self;
        log!("TokLgcWithdraw process()");
        check_signer(user)?;
        executable(token_program)?;
        writable(from_ata)?;
        check_ata(from_ata, from_wallet, mint)?;

        log!("TokLgcWithdraw 1");
        rent_exempt(mint, 0)?;
        check_decimals(mint, token_program, decimals)?;

        log!("TokLgcWithdraw 5");
        check_sysprog(system_program)?;

        if to_ata.data_is_empty() {
            log!("Make to_ata");
            pinocchio_associated_token_account::instructions::Create {
                funding_account: user,
                account: to_ata,
                wallet: user,
                mint,
                system_program,
                token_program,
            }
            .invoke()?;
            //Please upgrade to SPL Token 2022 for immutable owner support
        } else {
            log!("to_ata has data");
            check_ata(to_ata, user, mint)?;
        }
        writable(to_ata)?;
        rent_exempt(to_ata, 1)?;
        log!("ToATA is found/verified");

        let (expected_vault_pda, bump) = derive_pda1(user, b"vault")?;
        if from_wallet.key() != &expected_vault_pda {
            return Err(ProgramError::InvalidAccountData);
        }
        let signer_seeds = [
            Seed::from(b"vault".as_slice()),
            Seed::from(user.key().as_ref()),
            Seed::from(core::slice::from_ref(&bump)),
        ];
        let signer = Signer::from(&signer_seeds);

        log!("Transfer Tokens");
        pinocchio_token::instructions::TransferChecked {
            from: from_ata,
            mint,
            to: to_ata,
            authority: from_wallet,
            amount, // unsafe { *(data.as_ptr().add(1 + 8) as *const u64)}
            decimals,
        }
        .invoke_signed(&[signer])?;
        /*  pinocchio_token::instructions::Transfer {
            from: vault,
            to: to_ata,
            authority: escrow,
            amount: vault_account.amount(),
        }.invoke_signed(&[seeds.clone()])?; */
        Ok(())
    }
}
impl<'a> TryFrom<(&'a [u8], &'a [AccountInfo])> for TokLgcWithdraw<'a> {
    type Error = ProgramError;

    fn try_from(value: (&'a [u8], &'a [AccountInfo])) -> Result<Self, Self::Error> {
        log!("TokLgcWithdraw try_from");
        let (data, accounts) = value;
        log!("accounts len: {}, data len: {}", accounts.len(), data.len());

        let [user, from_ata, to_ata, from_wallet, mint, token_program, system_program, atoken_program] =
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
            user,
            from_ata,
            to_ata,
            from_wallet,
            mint,
            token_program,
            system_program,
            atoken_program,
            decimals,
            amount,
        })
    }
}
/*  pinocchio_token::instructions::CloseAccount {
    account: vault,
    destination: maker,
    authority: escrow,
}.invoke_signed(&[seeds])?;  */
