use core::convert::TryFrom;
use pinocchio::{account_info::AccountInfo, program_error::ProgramError, ProgramResult};
//use pinocchio_log::log;
//use pinocchio_system::instructions::CreateAccount;

use crate::instructions::{check_signer, check_str_len};
//use pinocchio_token::instructions::InitializeMint2;

//Token2022 Accounts
pub struct Token2022<'a> {
    pub owner: &'a AccountInfo,
    pub name: &'a str,
    pub symbol: &'a str,
    pub uri: &'a str,
    pub decimals: u8,
}
impl<'a> Token2022<'a> {
    pub const DISCRIMINATOR: &'a u8 = &2;

    pub fn process(self) -> ProgramResult {
        let Token2022 {
            owner,
            name,
            symbol,
            uri,
            decimals,
        } = self;
        check_signer(owner)?;
        check_str_len(name, 3, 20)?;
        check_str_len(symbol, 3, 20)?;
        check_str_len(uri, 3, 20)?;

        /// [4 (extension discriminator) + 32 (update_authority) + 32 (metadata)]
        const METADATA_POINTER_SIZE: usize = 4 + 32 + 32;
        /// [4 (extension discriminator) + 32 (update_authority) + 32 (mint) + 4 (size of name ) + 4 (size of symbol) + 4 (size of uri) + 4 (size of additional_metadata)]
        const METADATA_EXTENSION_BASE_SIZE: usize = 4 + 32 + 32 + 4 + 4 + 4 + 4;
        /// Padding used so that Mint and Account extensions start at the same index
        const EXTENSIONS_PADDING_AND_OFFSET: usize = 84;
        /*
        https://www.helius.dev/blog/pinocchio#pinocchio-vs-steel
                /* within `process_instruction` */
                let extension_size = METADATA_POINTER_SIZE
                    + METADATA_EXTENSION_BASE_SIZE
                    + name.len()
                    + symbol.len()
                    + uri.len();
                let total_mint_size = Mint::LEN + EXTENSIONS_PADDING_AND_OFFSET + extension_size;

                let rent = Rent::get()?;
                // Create the account for the Mint
                CreateAccount {
                    from: owner,
                    to: mint_account,
                    owner: token2022_program.key(),
                    lamports: rent.minimum_balance(Mint::LEN),
                    space: Mint::LEN as u64,
                }
                .invoke()?;

              // Initialize MetadataPointer extension pointing to the Mint account
        InitializeMetadataPointer {
        mint: mint_account,
        authority: Some(*payer.key()),
        metadata_address: Some(*mint_account.key()),
        }
        .invoke()?;

        // Now initialize that account as a Token2022 Mint
        InitializeMint2 {
        mint: mint_account,
        decimals: args.decimals,
        mint_authority: mint_authority.key(),
        freeze_authority: None,
        }
        .invoke(TokenProgramVariant::Token2022)?;

        // Set the metadata within the Mint account
        InitializeTokenMetadata {
        metadata: mint_account,
        update_authority: payer,
        mint: mint_account,
        mint_authority: payer,
        name: &args.name,
        symbol: &args.symbol,
        uri: &args.uri,
        }
        .invoke()?;
        */
        Ok(())
    }
}
impl<'a> TryFrom<(&'a [u8], &'a [AccountInfo])> for Token2022<'a> {
    type Error = ProgramError;

    fn try_from(value: (&'a [u8], &'a [AccountInfo])) -> Result<Self, Self::Error> {
        let (data, accounts) = value;
        if accounts.len() < 1 {
            return Err(ProgramError::NotEnoughAccountKeys);
        }
        let owner = &accounts[0];
        //let [name, symbol, _system_program, _] = accounts else { return Err(ProgramError::NotEnoughAccountKeys);}

        if data.len() == 0 {
            return Err(ProgramError::AccountDataTooSmall);
        }
        let decimals = data[0];
        if decimals > 18 {
            return Err(ProgramError::InvalidInstructionData);
        }
        //TODO: extract name, symbol, uri with decimals from data
        Ok(Self {
            owner,
            name: "token_name",
            symbol: "token_symbol",
            uri: "token_uri",
            decimals,
        })
    }
}
