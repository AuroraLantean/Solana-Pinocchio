use crate::{
  check_mint0a, check_sysprog, executable, initialized, instructions::check_signer,
  not_initialized, rent_exempt22,
};
use core::convert::TryFrom;
use pinocchio::{account_info::AccountInfo, program_error::ProgramError, ProgramResult};
use pinocchio_log::log;

/// Token Legacy Init ATA(Associated Token Account)
pub struct TokenLgcInitAta<'a> {
  pub payer: &'a AccountInfo,
  pub to_wallet: &'a AccountInfo,
  pub mint: &'a AccountInfo,
  pub ata: &'a AccountInfo,
  pub token_program: &'a AccountInfo,
  pub system_program: &'a AccountInfo,
  pub atoken_program: &'a AccountInfo,
}
impl<'a> TokenLgcInitAta<'a> {
  pub const DISCRIMINATOR: &'a u8 = &3;

  pub fn process(self) -> ProgramResult {
    let TokenLgcInitAta {
      payer, //signer
      to_wallet,
      mint,
      ata,
      token_program,
      system_program,
      atoken_program: _,
    } = self;
    log!("TokenLgcInitAta process()");
    pinocchio_associated_token_account::instructions::Create {
      funding_account: payer, // Keypair
      account: ata,
      wallet: to_wallet,
      mint: mint,
      system_program: system_program,
      token_program: token_program,
    }
    .invoke()?;
    Ok(())
  }
  pub fn init_if_needed(self) -> ProgramResult {
    if self.ata.lamports() == 0 {
      Self::process(self)?;
    }
    Ok(())
  }
}
impl<'a> TryFrom<(&'a [u8], &'a [AccountInfo])> for TokenLgcInitAta<'a> {
  type Error = ProgramError;

  fn try_from(value: (&'a [u8], &'a [AccountInfo])) -> Result<Self, Self::Error> {
    log!("TokenLgcInitAta try_from");
    let (data, accounts) = value;
    log!("accounts len: {}, data len: {}", accounts.len(), data.len());

    let [payer, to_wallet, mint, ata, token_program, system_program, atoken_program] = accounts
    else {
      return Err(ProgramError::NotEnoughAccountKeys);
    };
    check_signer(payer)?;
    executable(token_program)?;
    check_sysprog(system_program)?;
    //check_pda(config_pda)?;
    not_initialized(ata)?;
    initialized(to_wallet)?;
    log!("TokenLgcInitAta try_from 3");
    rent_exempt22(mint, 0)?;
    check_mint0a(mint, token_program)?;

    log!("TokenLgcInitAta try_from end");
    Ok(Self {
      payer,
      to_wallet,
      mint,
      ata,
      token_program,
      system_program,
      atoken_program,
    })
  }
}
/*find ata from to_wallet & token mint ... but ignored because this make the program bigger
cargo add spl-associated-token-account
use spl_associated_token_account::get_associated_token_address;
let ata = get_associated_token_address(&to_wallet, &mint);

        CreateAccount {
            from: payer,
            to: account,
            lamports,
            space: pinocchio_token::state::TokenAccount::LEN as u64,
            owner: &pinocchio_token::ID,
        }.invoke()?;

        // Initialize the Token Account
        InitializeAccount3 {
            account,
            mint,
            owner,
        }.invoke()
*/
