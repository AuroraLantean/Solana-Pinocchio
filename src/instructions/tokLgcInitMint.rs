use core::convert::TryFrom;
use pinocchio::{
  sysvars::{rent::Rent, Sysvar},
  AccountView, ProgramResult,
};
use pinocchio_log::log;
use pinocchio_system::instructions::CreateAccount;

use crate::{
  check_decimals_max, check_sysprog, data_len, executable, initialized, instructions::check_signer,
  not_initialized, writable,
};
use pinocchio_token::{instructions::InitializeMint2, state::Mint};

//TokenLgc Init Mint Account
pub struct TokenLgcInitMint<'a> {
  pub payer: &'a AccountView, //signer
  pub mint: &'a AccountView,
  pub mint_authority: &'a AccountView,
  pub token_program: &'a AccountView,
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

    let lamports = Rent::get()?.minimum_balance(Mint::LEN);
    log!("TokenLgcInitMint 6");
    let space = Mint::LEN as u64;
    log!("lamports: {}, space: {}", lamports, space);
    //let mint = Keypair::new();

    log!("Make Mint Account"); //payer and mint are both keypairs!
    CreateAccount {
      from: payer, //Keypair
      to: mint,
      owner: token_program.key(), //address("TokenXYZ");
      lamports,
      space,
    }
    .invoke()?;
    log!("TokenLgcInitMint 7");
    writable(mint)?;

    log!("Init Mint");
    InitializeMint2 {
      mint, //Keypair
      decimals,
      mint_authority: mint_authority.key(),
      freeze_authority: freeze_authority_opt,
    }
    .invoke()?;
    Ok(())
  }
  pub fn init_if_needed(self) -> ProgramResult {
    if self.mint.lamports() == 0 {
      Self::process(self)?;
    }
    Ok(())
  }
}
impl<'a> TryFrom<(&'a [u8], &'a [AccountView])> for TokenLgcInitMint<'a> {
  type Error = ProgramError;

  fn try_from(value: (&'a [u8], &'a [AccountView])) -> Result<Self, Self::Error> {
    log!("TokenLgcInitMint try_from");
    let (data, accounts) = value;
    log!("accounts len: {}, data len: {}", accounts.len(), data.len());

    let [payer, mint, mint_authority, token_program, freeze_authority_opt1, system_program] =
      accounts
    else {
      return Err(ProgramError::NotEnoughAccountKeys);
    };
    check_signer(payer)?;
    executable(token_program)?;
    check_sysprog(system_program)?;
    //check_pda(config_pda)?;
    not_initialized(mint)?;
    initialized(mint_authority)?;
    log!("TokenLgcInitMint try_from 3");

    let freeze_authority_opt: Option<&'a [u8; 32]> = if freeze_authority_opt1 == system_program {
      Some(freeze_authority_opt1.key())
    } else {
      None
    };

    data_len(data, 1)?;
    let decimals = data[0];
    log!("decimals: {}", decimals);
    check_decimals_max(decimals, 18)?;

    Ok(Self {
      payer,
      mint,
      mint_authority, //.try_into()
      token_program,
      freeze_authority_opt,
      decimals,
    })
  }
}
