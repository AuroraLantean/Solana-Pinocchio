use core::convert::TryFrom;
use pinocchio::{account_info::AccountInfo, program_error::ProgramError, ProgramResult};
use pinocchio_log::log;

use crate::{
  check_ata, check_decimals, check_from_balc, check_mint0a, check_pda, check_sysprog, data_len,
  executable, instructions::check_signer, none_zero_u64, parse_u64, rent_exempt22, writable,
};

/// Make Escrow Token Offer
pub struct EscrowTokMake<'a> {
  pub maker: &'a AccountInfo, //signer
  pub from_ata: &'a AccountInfo,
  pub vault_ata: &'a AccountInfo, //as to_ata
  pub vault: &'a AccountInfo,     //PDA or to_wallet
  pub mint_maker: &'a AccountInfo,
  pub mint_taker: &'a AccountInfo,
  pub token_program: &'a AccountInfo,
  pub system_program: &'a AccountInfo,
  pub atoken_program: &'a AccountInfo,
  pub amount: u64,
  pub decimals: u8,
}
impl<'a> EscrowTokMake<'a> {
  pub const DISCRIMINATOR: &'a u8 = &15;

  pub fn process(self) -> ProgramResult {
    let EscrowTokMake {
      maker,
      from_ata,
      vault_ata,
      vault,
      mint_maker,
      mint_taker,
      token_program,
      system_program,
      atoken_program: _,
      amount,
      decimals,
    } = self;
    log!("EscrowTokMake process()");
    log!("EscrowTokMake 1");
    rent_exempt22(mint_maker, 0)?; //invalid mint_maker will also fail this txn
    rent_exempt22(mint_taker, 0)?;
    log!("EscrowTokMake 2");
    check_ata(from_ata, maker, mint_maker)?;

    log!("EscrowTokMake 3");
    check_decimals(mint_maker, decimals)?;
    check_mint0a(mint_maker, token_program)?;
    check_mint0a(mint_taker, token_program)?;

    log!("EscrowTokMake 5");
    /*Make a valid program derived address without searching for a bump seed:
    let seed = [(b"escrow"), maker.key().as_slice(), bump.as_ref()];
    let seeds = &seed[..];
    let pda = pubkey::checked_create_program_address(seeds, &crate::ID).unwrap();*/
    if vault_ata.data_is_empty() {
      log!("Make vault_ata");
      pinocchio_associated_token_account::instructions::Create {
        funding_account: maker,
        account: vault_ata,
        wallet: vault,
        mint: mint_maker,
        system_program,
        token_program,
      }
      .invoke()?;
      //Please upgrade to SPL Token 2022 for immutable owner support
    } else {
      log!("vault_ata has data");
      check_ata(vault_ata, vault, mint_maker)?;
    }
    writable(vault_ata)?;
    rent_exempt22(vault_ata, 1)?;
    log!("EscrowTokMake 7: ToATA is found/verified");

    log!("EscrowTokMake 8: Transfer Tokens");
    pinocchio_token::instructions::TransferChecked {
      from: from_ata,
      mint: mint_maker,
      to: vault_ata,
      authority: maker,
      amount,
      decimals,
    }
    .invoke()?;
    /*  pinocchio_token::instructions::Transfer {
        from: vault,
        to: vault_ata,
        authority: escrow,
        amount: vault_account.amount(),
    }.invoke_signed(&[seeds.clone()])?; */
    Ok(())
  }
}
impl<'a> TryFrom<(&'a [u8], &'a [AccountInfo])> for EscrowTokMake<'a> {
  type Error = ProgramError;

  fn try_from(value: (&'a [u8], &'a [AccountInfo])) -> Result<Self, Self::Error> {
    log!("EscrowTokMake try_from");
    let (data, accounts) = value;
    log!("accounts len: {}, data len: {}", accounts.len(), data.len());

    let [maker, from_ata, vault_ata, vault, mint_maker, mint_taker, token_program, system_program, atoken_program] =
      accounts
    else {
      return Err(ProgramError::NotEnoughAccountKeys);
    };
    check_signer(maker)?;
    executable(token_program)?;
    check_sysprog(system_program)?;
    writable(from_ata)?;
    check_pda(vault)?;
    //check_pda(config_pda)?;
    log!("precheck passed");

    //u8 takes 1 + u64 takes 8 bytes
    data_len(data, 9)?;

    let decimals = data[0];
    let amount = parse_u64(&data[1..])?;
    log!("decimals: {}, amount: {}", decimals, amount);

    none_zero_u64(amount)?;
    check_from_balc(from_ata, amount)?;

    Ok(Self {
      maker,
      from_ata,
      vault_ata,
      vault,
      mint_maker,
      mint_taker,
      token_program,
      system_program,
      atoken_program,
      decimals,
      amount,
    })
  }
}
