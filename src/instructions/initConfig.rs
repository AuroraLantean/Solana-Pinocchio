use crate::{
  check_sysprog, data_len, derive_pda1, get_time, instructions::check_signer, not_initialized,
  parse_u64, rent_exempt_mint22, to32bytes, u8_to_bool, Config, Ee, PROG_ADDR, VAULT_SEED,
};
use core::convert::TryFrom;
use pinocchio::{
  cpi::{Seed, Signer},
  error::ProgramError,
  sysvars::rent::Rent,
  AccountView, Address, ProgramResult,
};
use pinocchio_log::log;

/// Init Config PDA
pub struct InitConfig<'a> {
  pub signer: &'a AccountView,
  pub config_pda: &'a AccountView,
  pub prog_owner: &'a Address,
  pub prog_admin: &'a Address,
  pub mints: [&'a Address; 4],
  pub vault: &'a Address,
  pub system_program: &'a AccountView,
  pub fee: u64,
  pub is_authorized: bool,
  pub status: u8,
  pub vault_bump: u8,
  pub str_u8array: [u8; 32],
}
impl<'a> InitConfig<'a> {
  pub const DISCRIMINATOR: &'a u8 = &12;

  pub fn process(self) -> ProgramResult {
    let InitConfig {
      signer,
      config_pda,
      prog_owner,
      prog_admin,
      mints,
      vault,
      system_program: _,
      fee,
      is_authorized,
      status,
      vault_bump,
      str_u8array,
    } = self;
    log!("InitConfig process()");
    let config_rent = Rent::from_account_view(config_pda)?;
    let lamports = Rent::try_minimum_balance(&config_rent, Config::LEN)?;
    let space = Config::LEN as u64;

    log!("InitConfig 4. space: {}", space);
    let (expected_config_pda, bump) = derive_pda1(prog_owner, Config::SEED)?;

    log!("InitConfig 5");
    if expected_config_pda != *config_pda.address() {
      return Ee::ConfigPDA.e();
    }

    log!("InitConfig 6");
    let seeds = [
      Seed::from(Config::SEED),
      Seed::from(prog_owner.as_ref()),
      Seed::from(core::slice::from_ref(&bump)),
    ];
    let seed_signer = Signer::from(&seeds);

    log!("InitConfig 7");
    pinocchio_system::instructions::CreateAccount {
      from: signer,
      to: config_pda,
      lamports,
      space,
      owner: &PROG_ADDR,
    }
    .invoke_signed(&[seed_signer])?;

    log!("InitConfig after initialization");
    let time = get_time()?;

    self.config_pda.check_borrow_mut()?;
    let config = Config::from_account_view(&config_pda)?;
    config.set_mints(mints);
    config.set_vault(vault);
    config.set_prog_owner(prog_owner);
    config.set_admin(prog_admin);
    config.set_str_u8array(str_u8array);
    config.set_fee(fee)?;
    config.set_updated_at(time);
    config.set_is_authorized(is_authorized);
    config.set_status(status);
    config.set_vault_bump(vault_bump);
    config.set_bump(bump);
    Ok(())
  }
}
impl<'a> TryFrom<(&'a [u8], &'a [AccountView])> for InitConfig<'a> {
  type Error = ProgramError;

  fn try_from(value: (&'a [u8], &'a [AccountView])) -> Result<Self, Self::Error> {
    log!("InitConfig try_from");
    let (data, accounts) = value;
    log!("accounts len: {}, data len: {}", accounts.len(), data.len());

    let [signer, config_pda, mint0, mint1, mint2, mint3, vault, prog_owner, prog_admin, system_program] =
      accounts
    else {
      return Err(ProgramError::NotEnoughAccountKeys);
    };
    check_signer(signer)?;
    check_sysprog(system_program)?;
    //writable(config_pda)?;
    not_initialized(config_pda)?;
    log!("try check mints");
    rent_exempt_mint22(mint0)?;
    rent_exempt_mint22(mint1)?;
    rent_exempt_mint22(mint2)?;
    rent_exempt_mint22(mint3)?;

    let (vault_expected, vault_bump) = derive_pda1(prog_owner.address(), VAULT_SEED)?;
    if vault.address() != &vault_expected {
      return Err(Ee::VaultPDA.into());
    }

    let data_size1 = 42; //1+1+8+32
    data_len(data, data_size1)?;

    let is_authorized = u8_to_bool(data[0])?;
    let status = data[1];
    let fee = parse_u64(&data[2..10])?;
    let str_u8array = *to32bytes(&data[10..data_size1])?;

    Ok(Self {
      signer,
      config_pda,
      prog_owner: prog_owner.address(),
      prog_admin: prog_admin.address(),
      mints: [
        mint0.address(),
        mint1.address(),
        mint2.address(),
        mint3.address(),
      ],
      vault: vault.address(),
      system_program,
      fee,
      is_authorized,
      status,
      vault_bump,
      str_u8array,
    })
  }
}
