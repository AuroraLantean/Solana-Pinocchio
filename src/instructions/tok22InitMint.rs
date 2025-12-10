use core::convert::TryFrom;
use pinocchio::{
    account_info::AccountInfo,
    program_error::ProgramError,
    sysvars::{rent::Rent, Sysvar},
    ProgramResult,
};
use pinocchio_log::log;
use pinocchio_system::instructions::CreateAccount;

use crate::{empty_data, empty_lamport, executable, instructions::check_signer, writable};
use pinocchio_token_2022::{instructions::InitializeMint2, state::Mint};

//Initiate Token2022 Mint Account
pub struct Token2022InitMint<'a> {
    pub mint_authority: &'a AccountInfo, //signer
    pub mint: &'a AccountInfo,
    pub token_program: &'a AccountInfo,
    pub freeze_authority_opt: Option<&'a [u8; 32]>, // or Pubkey
    //pub name: &'a str,
    //pub symbol: &'a str,
    //pub uri: &'a str,
    pub decimals: u8,
}
impl<'a> Token2022InitMint<'a> {
    pub const DISCRIMINATOR: &'a u8 = &3;

    pub fn process(self) -> ProgramResult {
        let Token2022InitMint {
            mint_authority,
            mint,
            token_program,
            freeze_authority_opt,
            //name,symbol,uri,
            decimals,
        } = self;
        log!("process()");
        check_signer(mint_authority)?;
        log!("here 2");
        empty_lamport(mint)?;
        log!("here 3");
        empty_data(mint)?;
        log!("here 4");
        executable(token_program)?;

        if decimals > 18 {
            return Err(ProgramError::InvalidArgument);
        }
        //check_str_len(name, 3, 20)?;
        //check_str_len(symbol, 3, 20)?;
        //check_str_len(uri, 3, 20)?;

        log!("Make Mint Account");
        CreateAccount {
            from: mint_authority,
            to: mint,
            owner: token_program.key(),
            lamports: Rent::get()?.minimum_balance(Mint::BASE_LEN),
            space: Mint::BASE_LEN as u64,
        }
        .invoke()?;
        log!("here 5");
        writable(mint)?;

        log!("Init Mint");
        InitializeMint2 {
            mint: mint,
            decimals: decimals,
            mint_authority: mint_authority.key(),
            freeze_authority: freeze_authority_opt,
            token_program: token_program.key(),
        }
        .invoke()?;
        Ok(())
    }
    pub fn init_if_needed(self) -> ProgramResult {
        match empty_lamport(self.mint) {
            Ok(_) => Self::process(self),
            Err(_) => Ok(()),
        }
    }
}
impl<'a> TryFrom<(&'a [u8], &'a [AccountInfo])> for Token2022InitMint<'a> {
    type Error = ProgramError;

    fn try_from(value: (&'a [u8], &'a [AccountInfo])) -> Result<Self, Self::Error> {
        log!("try_from");
        let (data, accounts) = value;
        log!("accounts len: {}, data len: {}", accounts.len(), data.len());
        //accounts len: 5, data len: 1

        if accounts.len() < 3 {
            return Err(ProgramError::NotEnoughAccountKeys);
        }
        let mint_authority = &accounts[0];
        let mint = &accounts[1];
        let token_program = &accounts[2];
        let freeze_authority_opt: Option<&'a [u8; 32]>;
        if accounts.len() > 3 {
            freeze_authority_opt = Some(&accounts[3].key());
        } else {
            freeze_authority_opt = None;
        }

        if data.len() < 1 {
            return Err(ProgramError::AccountDataTooSmall);
        }
        let decimals = data[0];
        log!("decimals: {}", decimals);
        //TODO: extract name, symbol, uri
        //let mint_authority = &data[1..];
        Ok(Self {
            mint_authority, //.try_into()
            mint,
            token_program,
            freeze_authority_opt,
            decimals,
            //name: "token_name",
            //symbol: "token_symbol",
            //uri: "token_uri",
        })
    }
}
// Initialize MetadataPointer extension pointing to the Mint account
/*InitializeMetadataPointer {
    mint: mint,
    authority: Some(*payer.key()),
    metadata_address: Some(*mint.key()),
}
.invoke()?;*/

// Set the metadata within the Mint account
/*InitializeTokenMetadata {
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
    return Err(ProgramError::InvalidAccountData);
}*/
