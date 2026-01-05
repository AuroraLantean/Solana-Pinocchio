use core::convert::TryFrom;
use pinocchio::{
  account_info::AccountInfo,
  program_error::ProgramError,
  sysvars::{rent::Rent, Sysvar},
  ProgramResult,
};
use pinocchio_log::log;
use pinocchio_system::instructions::CreateAccount;

use crate::{
  check_decimals_max, check_sysprog, empty_data, executable, initialized,
  instructions::check_signer, min_data_len, not_initialized, to10bytes, to32bytes, to6bytes,
  writable,
};
use pinocchio_token_2022::{instructions::InitializeMint2, state::Mint};

//Initiate Token2022 Mint Account
pub struct Token2022InitMint<'a> {
  pub payer: &'a AccountInfo, //signer
  pub mint: &'a AccountInfo,
  pub mint_authority: &'a AccountInfo,
  pub token_program: &'a AccountInfo,
  pub freeze_authority_opt: Option<&'a [u8; 32]>, // or Pubkey
  pub decimals: u8,
  pub token_name: [u8; 10],
  pub token_symbol: [u8; 6],
  pub token_uri: [u8; 32],
}
impl<'a> Token2022InitMint<'a> {
  pub const DISCRIMINATOR: &'a u8 = &9;

  pub fn process(self) -> ProgramResult {
    let Token2022InitMint {
      payer,
      mint,
      mint_authority,
      token_program,
      freeze_authority_opt,
      decimals,
      token_name: _,
      token_symbol: _,
      token_uri: _,
    } = self;
    log!("Token2022InitMint process()");

    let lamports = Rent::get()?.minimum_balance(Mint::BASE_LEN);
    let space = Mint::BASE_LEN as u64;
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
    log!("Token2022InitMint 7");
    writable(mint)?;

    log!("Init Mint");
    InitializeMint2 {
      mint: mint, //Keypair
      decimals: decimals,
      mint_authority: mint_authority.key(),
      freeze_authority: freeze_authority_opt,
      token_program: token_program.key(),
    }
    .invoke()?;
    //TODO: search "InitializeMetadataPointer solana token 2022" to add metadata: https://solana.com/docs/tokens/extensions/metadata
    Ok(())
  }
  pub fn init_if_needed(self) -> ProgramResult {
    if self.mint.lamports() == 0 {
      Self::process(self)?;
    }
    Ok(())
  }
}
impl<'a> TryFrom<(&'a [u8], &'a [AccountInfo])> for Token2022InitMint<'a> {
  type Error = ProgramError;

  fn try_from(value: (&'a [u8], &'a [AccountInfo])) -> Result<Self, Self::Error> {
    log!("Token2022InitMint try_from");
    let (data, accounts) = value;
    log!("accounts len: {}, data len: {}", accounts.len(), data.len());
    //accounts len: 5, data len: 1

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
    log!("Token2022InitMint try_from 3");

    let freeze_authority_opt: Option<&'a [u8; 32]> = if freeze_authority_opt1 == token_program {
      Some(freeze_authority_opt1.key())
    } else {
      None
    };

    min_data_len(data, 49)?; //1+16+32=49
    let decimals = data[0];
    log!("decimals: {}", decimals);
    check_decimals_max(decimals, 18)?;

    // 16 = 10 token name + 6 token symbol
    let token_name = *to10bytes(&data[1..11])?;
    log!("token_name: {}", &token_name);
    let token_symbol = *to6bytes(&data[11..17])?;
    log!("token_symbol: {}", &token_symbol);

    //32 length
    let token_uri = *to32bytes(&data[17..49])?;
    log!("token_uri: {}", &token_uri);

    Ok(Self {
      payer,
      mint,
      mint_authority, //.try_into()
      token_program,
      freeze_authority_opt,
      decimals,
      token_name,
      token_symbol,
      token_uri,
    })
  }
}
/*https://www.helius.dev/blog/pinocchio#how-is-pinocchio-more-performant-than-solana-program
#[derive(BorshDeserialize, Debug)]
pub struct CreateTokenArgs {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub decimals: u8,
}

// Initialize MetadataPointer extension pointing to the Mint account
InitializeMetadataPointer {
    mint: mint,
    authority: Some(*payer.key()),
    metadata_address: Some(*mint.key()),
}
.invoke()?;

// Set the metadata within the Mint account
InitializeTokenMetadata {
    metadata: mint,
    update_authority: payer,
    mint: mint,
    mint_authority: payer,
    name: &name,
    symbol: &symbol,
    uri: &uri,
}
.invoke()?;
https://www.helius.dev/blog/pinocchio#pinocchio-vs-steel
// within `process_instruction`
let extension_size = METADATA_POINTER_SIZE
            + METADATA_EXTENSION_BASE_SIZE
            + name.len()
            + symbol.len()
            + uri.len();
let total_mint_size = Mint::LEN + EXTENSIONS_PADDING_AND_OFFSET + extension_size;        */
/*if rent_sysvar.key != &solana_program::sysvar::rent::ID {
    return Err();
}*/
