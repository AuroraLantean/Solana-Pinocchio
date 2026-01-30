use core::convert::TryFrom;
use pinocchio::{error::ProgramError, sysvars::rent::Rent, AccountView, Address, ProgramResult};
use pinocchio_log::log;
use pinocchio_system::instructions::CreateAccount;

use crate::{
  check_decimals_max, check_sysprog, data_len, executable, initialized, instructions::check_signer,
  not_initialized, to10bytes, to32bytes, to6bytes, writable,
};
use pinocchio_token_2022::{instructions::InitializeMint2, state::Mint};

//Initiate Token2022 Mint Account
pub struct Token2022InitMint<'a> {
  pub payer: &'a AccountView, //signer
  pub mint: &'a AccountView,
  pub mint_authority: &'a AccountView,
  pub token_program: &'a AccountView,
  pub freeze_authority_opt: Option<&'a Address>, // or Pubkey
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
      token_name,
      token_symbol,
      token_uri,
    } = self;
    log!("Token2022InitMint process()");

    /// [4 (extension discriminator) + 32 (update_authority) + 32 (metadata)]
    pub const METADATA_POINTER_SIZE: usize = 4 + 32 + 32;
    /// [4 (extension discriminator) + 32 (update_authority) + 32 (mint) + 10 (size of name ) + 6 (size of symbol) + 32 (size of uri) + 4 (size of additional_metadata)]
    pub const METADATA_EXTENSION_BASE_SIZE: usize = 4 + 32 + 32 + 10 + 6 + 32 + 4;
    /// Padding used so that Mint and Account extensions start at the same index
    pub const EXTENSIONS_PADDING_AND_OFFSET: usize = 84;

    let extension_size = METADATA_POINTER_SIZE
      + METADATA_EXTENSION_BASE_SIZE
      + token_name.len()
      + token_symbol.len()
      + token_uri.len();
    let _total_mint_size = Mint::BASE_LEN + EXTENSIONS_PADDING_AND_OFFSET + extension_size;

    let rent = Rent::from_account_view(mint)?;
    let lamports = Rent::try_minimum_balance(&rent, Mint::BASE_LEN)?;
    let space = Mint::BASE_LEN as u64;
    log!("lamports: {}, space: {}", lamports, space);
    //let mint = Keypair::new();

    log!("Make Mint Account"); //payer and mint are both keypairs!
    CreateAccount {
      from: payer, //Keypair
      to: mint,
      owner: token_program.address(), //address("TokenXYZ");
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
      mint_authority: mint_authority.address(),
      freeze_authority: freeze_authority_opt,
      token_program: token_program.address(),
    }
    .invoke()?;

    //TODO: search "InitializeMetadataPointer solana token 2022" to add metadata: https://solana.com/docs/tokens/extensions/metadata
    // Initialize MetadataPointer extension pointing to the Mint account
    /*InitializeMetadataPointer {
      mint: mint_account,
      authority: Some(*payer.address()),
      metadata_address: Some(*mint_account.address()),
    }.invoke()?;

        // Now initialize that account as a Token2022 Mint
    InitializeMint2 {
        mint: mint_account,
        decimals: args.decimals,
        mint_authority: mint_authority.address(),
        freeze_authority: None,
    }
    .invoke(TokenProgramVariant::Token2022)?;

     //https://www.helius.dev/blog/pinocchio#how-is-pinocchio-more-performant-than-solana-program

     // Set the metadata within the Mint account
     InitializeTokenMetadata {
         metadata: mint,
         update_authority: payer,
         mint: mint,
         mint_authority: payer,
         name: &name,
         symbol: &symbol,
         uri: &uri,
     }.invoke()?;*/
    Ok(())
  }
  pub fn init_if_needed(self) -> ProgramResult {
    if self.mint.lamports() == 0 {
      Self::process(self)?;
    }
    Ok(())
  }
}
impl<'a> TryFrom<(&'a [u8], &'a [AccountView])> for Token2022InitMint<'a> {
  type Error = ProgramError;

  fn try_from(value: (&'a [u8], &'a [AccountView])) -> Result<Self, Self::Error> {
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

    let freeze_authority_opt: Option<&'a Address> = if freeze_authority_opt1 == token_program {
      Some(freeze_authority_opt1.address())
    } else {
      None
    };

    data_len(data, 49)?; //1+16+32=49
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
