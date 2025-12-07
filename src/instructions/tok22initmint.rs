use core::convert::TryFrom;
use pinocchio::{
    account_info::AccountInfo,
    program_error::ProgramError,
    pubkey::{Pubkey, PUBKEY_BYTES},
    sysvars::{rent::Rent, Sysvar},
    ProgramResult,
};
use pinocchio_log::log;
use pinocchio_system::instructions::CreateAccount;
//use pinocchio_log::log;

use crate::instructions::{check_signer, check_str_len};
use pinocchio_token_2022::{instructions::InitializeMint2, state::Mint};

//Initiate Token2022 Mint Account
pub struct Token2022InitMint<'a> {
    pub payer: &'a AccountInfo,
    pub mint_authority: &'a AccountInfo,
    pub mint_account: &'a AccountInfo,
    pub token_program: &'a AccountInfo,
    pub freeze_authority_opt: Option<&'a AccountInfo>,
    pub name: &'a str,
    pub symbol: &'a str,
    pub uri: &'a str,
    pub decimals: u8,
}
impl<'a> Token2022InitMint<'a> {
    pub const DISCRIMINATOR: &'a u8 = &2;

    pub fn init(self) -> ProgramResult {
        let Token2022InitMint {
            payer,
            mint_authority,
            mint_account,
            token_program,
            freeze_authority_opt,
            name,
            symbol,
            uri,
            decimals,
        } = self;
        check_signer(payer)?;
        if decimals > 18 {
            return Err(ProgramError::InvalidArgument);
        }
        check_str_len(name, 3, 20)?;
        check_str_len(symbol, 3, 20)?;
        check_str_len(uri, 3, 20)?;

        if mint_account.lamports() != 0 {
            log!("mint already exists");
            return Ok(());
        }

        /// [4 (extension discriminator) + 32 (update_authority) + 32 (metadata)]
        const METADATA_POINTER_SIZE: usize = 4 + 32 + 32;
        /// [4 (extension discriminator) + 32 (update_authority) + 32 (mint) + 4 (size of name ) + 4 (size of symbol) + 4 (size of uri) + 4 (size of additional_metadata)]
        const METADATA_EXTENSION_BASE_SIZE: usize = 4 + 32 + 32 + 4 + 4 + 4 + 4;
        /// Padding used so that Mint and Account extensions start at the same index
        const EXTENSIONS_PADDING_AND_OFFSET: usize = 84;

        CreateAccount {
            from: payer,
            to: mint_account,
            owner: token_program.key(),
            lamports: Rent::get()?.minimum_balance(Mint::BASE_LEN),
            space: Mint::BASE_LEN as u64,
        }
        .invoke()?;

        // Initialize MetadataPointer extension pointing to the Mint account
        /*InitializeMetadataPointer {
            mint: mint_account,
            authority: Some(*payer.key()),
            metadata_address: Some(*mint_account.key()),
        }
        .invoke()?;*/

        // initialize Token2022 Mint
        let freeze_authority: Option<&[u8; PUBKEY_BYTES]> =
            if let Some(freeze_authority) = freeze_authority_opt {
                Some(freeze_authority.key())
            } else {
                None
            };

        if !mint_account.is_writable() {
            return Err(ProgramError::InvalidAccountData);
        }
        if mint_account.data_len() != Mint::BASE_LEN {
            return Err(ProgramError::AccountDataTooSmall.into());
        }
        InitializeMint2 {
            mint: mint_account,
            decimals: decimals,
            mint_authority: mint_authority.key(),
            freeze_authority: freeze_authority,
            token_program: token_program.key(),
        }
        .invoke()?;

        // Set the metadata within the Mint account
        /*InitializeTokenMetadata {
            metadata: mint_account,
            update_authority: payer,
            mint: mint_account,
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
        Ok(())
    }
}
impl<'a> TryFrom<(&'a [u8], &'a [AccountInfo])> for Token2022InitMint<'a> {
    type Error = ProgramError;

    fn try_from(value: (&'a [u8], &'a [AccountInfo])) -> Result<Self, Self::Error> {
        let (data, accounts) = value;

        let [payer, mint_authority, mint_account, token_program, _] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        let data_iter = data.iter();
        if data.len() == 0 {
            return Err(ProgramError::AccountDataTooSmall);
        }
        let decimals = data[0];

        //TODO: extract freeze_authority, name, symbol, uri with decimals from data
        Ok(Self {
            payer,
            mint_authority,
            mint_account,
            token_program,
            freeze_authority_opt: None,
            name: "token_name",
            symbol: "token_symbol",
            uri: "token_uri",
            decimals,
        })
    }
}
