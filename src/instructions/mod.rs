//---------------== Module Declaration
#[allow(non_snake_case)]
pub mod closeConfig;
#[allow(non_snake_case)]
pub mod depositSol;
#[allow(non_snake_case)]
pub mod escrowTokMake;
#[allow(non_snake_case)]
pub mod initConfig;
#[allow(non_snake_case)]
pub mod tok22InitATA;
#[allow(non_snake_case)]
pub mod tok22InitMint;
#[allow(non_snake_case)]
pub mod tok22MintToken;
#[allow(non_snake_case)]
pub mod tokLgcDeposit;
#[allow(non_snake_case)]
pub mod tokLgcInitATA;
#[allow(non_snake_case)]
pub mod tokLgcInitMint;
#[allow(non_snake_case)]
pub mod tokLgcMintToken;
#[allow(non_snake_case)]
pub mod tokLgcPay;
#[allow(non_snake_case)]
pub mod tokLgcRedeem;
#[allow(non_snake_case)]
pub mod tokLgcWithdraw;
#[allow(non_snake_case)]
pub mod updateConfig;
pub mod utils;
#[allow(non_snake_case)]
pub mod withdrawSol;
//file names start with a lower case + Camel cases, but struct names start with Upper case + Camel cases!
pub use closeConfig::*;
pub use depositSol::*;
pub use escrowTokMake::*;
pub use initConfig::*;
pub use tok22InitATA::*;
pub use tok22InitMint::*;
pub use tok22MintToken::*;
pub use tokLgcDeposit::*;
pub use tokLgcInitATA::*;
pub use tokLgcInitMint::*;
pub use tokLgcMintToken::*;
pub use tokLgcPay::*;
pub use tokLgcRedeem::*;
pub use tokLgcWithdraw::*;
pub use updateConfig::*;
pub use utils::*;
pub use withdrawSol::*;

use shank::ShankInstruction;

//---------------== Shank IDL Definition
/// Shank IDL enum describes all program instructions and their required accounts.
/// Manually write this below, then run IDL generation; This below does not affect runtime behavior.
/// TODO: when is signer writable?
/// writable(to be modified):, name= signer, ata, pda
/// non writable: program, system_program, mint
#[derive(ShankInstruction)]
pub enum ProgramIx {
  /// 0 Deposit lamports into the vault.
  #[account(0, signer, writable, name = "user", desc = "User")]
  #[account(1, writable, name = "vault", desc = "VaultPDA")]
  #[account(2, name = "system_program", desc = "System Program")]
  Deposit { amount: u64 },

  /// 1 Withdraw lamports from the vault
  #[account(0, signer, writable, name = "user", desc = "User")]
  #[account(1, writable, name = "vault", desc = "Vault PDA")]
  Withdraw { amount: u64 },

  /// 2 TokLgc Init Mint
  #[account(0, signer, writable, name = "payer", desc = "Payer")]
  #[account(1, signer, writable, name = "mint", desc = "Mint")]
  #[account(2, name = "mint_authority", desc = "Mint Authority")]
  #[account(3, name = "token_program", desc = "Token Program")]
  #[account(4, name = "freeze_authority_opt", desc = "Freeze Authority")]
  #[account(5, name = "system_program", desc = "System Program")]
  TokenLgcInitMint { decimals: u8 },

  /// 3 TokLgc Init ATA(Associated Token Acct)
  #[account(0, signer, writable, name = "payer", desc = "Payer")]
  #[account(1, name = "to_wallet", desc = "To Wallet")]
  #[account(2, name = "mint", desc = "Mint")]
  #[account(3, writable, name = "ata", desc = "ATA Token Account")]
  #[account(4, name = "token_program", desc = "Token Program")]
  #[account(5, name = "system_program", desc = "System Program")]
  #[account(6, name = "atoken_program", desc = "AToken Program")]
  TokenLgcInitATA {},

  /// 4 TokLgc Mint Token
  #[account(0, signer, writable, name = "mint_authority", desc = "Mint Authority")]
  #[account(1, name = "to_wallet", desc = "ToWallet")]
  #[account(2, writable, name = "mint", desc = "Mint")]
  #[account(3, writable, name = "ata", desc = "ATA Token Account")]
  #[account(4, name = "token_program", desc = "Token Program")]
  #[account(5, name = "system_program", desc = "System Program")]
  #[account(6, name = "atoken_program", desc = "AToken Program")]
  TokLgcMintToken { decimals: u8, amount: u64 },

  /// 5 TokLgc Deposit Tokens
  #[account(0, signer, writable, name = "user", desc = "User")]
  #[account(1, writable, name = "from", desc = "From ATA")]
  #[account(2, writable, name = "to", desc = "To ATA")]
  #[account(3, name = "to_wallet", desc = "To Wallet")]
  #[account(4, name = "mint", desc = "Mint")]
  #[account(5, writable, name = "config_pda", desc = "config_pda")]
  #[account(6, name = "token_program", desc = "Token Program")]
  #[account(7, name = "system_program", desc = "System Program")]
  #[account(8, name = "atoken_program", desc = "AToken Program")]
  TokLgcDeposit { decimals: u8, amount: u64 },

  /// 6 TokLgc Withdraw Token
  #[account(0, signer, writable, name = "user", desc = "User")]
  #[account(1, writable, name = "from", desc = "From ATA")]
  #[account(2, writable, name = "to", desc = "To ATA")]
  #[account(3, name = "from_wallet", desc = "From Wallet")]
  #[account(4, name = "mint", desc = "Mint")]
  #[account(5, name = "token_program", desc = "Token Program")]
  #[account(6, name = "system_program", desc = "System Program")]
  #[account(7, name = "atoken_program", desc = "AToken Program")]
  TokLgcWithdraw { decimals: u8, amount: u64 },

  /// 7 TokLgc User Pays Tokens to VaultPDA
  #[account(0, signer, writable, name = "user", desc = "User")]
  #[account(1, writable, name = "from", desc = "User ATA")]
  #[account(2, writable, name = "to", desc = "Vault ATA")]
  #[account(3, name = "vault", desc = "Vault as To Wallet")]
  #[account(4, name = "mint", desc = "Mint")]
  #[account(5, writable, name = "config_pda", desc = "config_pda")]
  #[account(6, name = "token_program", desc = "Token Program")]
  #[account(7, name = "system_program", desc = "System Program")]
  #[account(8, name = "atoken_program", desc = "AToken Program")]
  TokLgcPay { decimals: u8, amount: u64 },

  /// 8 TokLgc Redeem Tokens
  #[account(0, signer, writable, name = "user", desc = "User")]
  #[account(1, writable, name = "from", desc = "From ATA")]
  #[account(2, writable, name = "to", desc = "To ATA")]
  #[account(3, name = "vault", desc = "Vault as From PDA")]
  #[account(4, name = "config_pda", desc = "Config PDA")]
  #[account(5, name = "mint", desc = "Mint")]
  #[account(6, name = "token_program", desc = "Token Program")]
  #[account(7, name = "system_program", desc = "System Program")]
  #[account(8, name = "atoken_program", desc = "AToken Program")]
  TokLgcRedeem { decimals: u8, amount: u64 },

  //---------== Token2022
  /// 9 Token2022 Init Mint
  #[account(0, signer, writable, name = "payer", desc = "Payer")]
  #[account(1, signer, writable, name = "mint", desc = "Mint")]
  #[account(2, name = "mint_authority", desc = "Mint Authority")]
  #[account(3, name = "token_program", desc = "Token Program")]
  #[account(4, name = "freeze_authority_opt", desc = "Freeze Authority")]
  #[account(5, name = "system_program", desc = "System Program")]
  Token2022InitMint {
    decimals: u8,
    token_name: [u8; 10],
    token_symbol: [u8; 6],
    token_uri: [u8; 32],
  },

  /// 10 Token2022 Init ATA(Associated Token Acct)
  #[account(0, signer, writable, name = "payer", desc = "Payer")]
  #[account(1, name = "to_wallet", desc = "To Wallet")]
  #[account(2, name = "mint", desc = "Mint")]
  #[account(3, writable, name = "ata", desc = "ATA Token Account")]
  #[account(4, name = "token_program", desc = "Token Program")]
  #[account(5, name = "system_program", desc = "System Program")]
  #[account(6, name = "atoken_program", desc = "AToken Program")]
  Token2022InitATA {},

  /// 11 Token2022 Mint Token
  #[account(0, signer, writable, name = "mint_authority", desc = "Mint Authority")]
  #[account(1, name = "to_wallet", desc = "ToWallet")]
  #[account(2, writable, name = "mint", desc = "Mint")]
  #[account(3, writable, name = "ata", desc = "ATA Token Account")]
  #[account(4, name = "token_program", desc = "Token Program")]
  #[account(5, name = "system_program", desc = "System Program")]
  #[account(6, name = "atoken_program", desc = "AToken Program")]
  Tok22MintToken { decimals: u8, amount: u64 },

  //---------------== Config PDA
  /// 12 Init Config PDA
  #[account(0, signer, writable, name = "signer", desc = "Signer")]
  #[account(1, writable, name = "config_pda", desc = "Config PDA")]
  #[account(2, name = "mint0", desc = "Mint 0")]
  #[account(3, name = "mint1", desc = "Mint 1")]
  #[account(4, name = "mint2", desc = "Mint 2")]
  #[account(5, name = "mint3", desc = "Mint 3")]
  #[account(6, name = "vault", desc = "VaultO")]
  #[account(7, name = "prog_owner", desc = "Program Owner")]
  #[account(8, name = "prog_admin", desc = "Program Admin")]
  #[account(9, name = "system_program", desc = "System Program")]
  InitConfig { fee: u64, is_authorized: bool },

  /// 13 Update Config PDA
  #[account(0, signer, writable, name = "authority", desc = "Authority")]
  #[account(1, writable, name = "config_pda", desc = "Config PDA")]
  #[account(2, name = "account1", desc = "Account1")]
  #[account(3, name = "account2", desc = "Account2")]
  UpdateConfig {
    bools: [u8; 4],
    u8s: [u8; 4],
    u32s: [u32; 4],
    u64s: [u64; 4],
    str_u8: [u8; 32],
  },

  /// 14 Close Config PDA
  #[account(0, signer, writable, name = "authority", desc = "Authority")]
  #[account(1, name = "pda", desc = "PDA")]
  #[account(2, name = "dest", desc = "Destination")]
  //#[account(5, name = "system_program", desc = "System Program")]
  CloseConfigPda {},

  //---------------== Escrow PDA
  /// 15 Escrow Token Make Offer
  #[account(0, signer, writable, name = "user_x", desc = "User X")]
  #[account(1, writable, name = "user_x_ata", desc = "User X ATA")]
  #[account(2, writable, name = "escrow_ata", desc = "Escrow ATA")]
  #[account(3, writable, name = "escrow", desc = "Escrow as To Wallet")]
  #[account(4, name = "mint_x", desc = "Mint X")]
  #[account(5, name = "mint_y", desc = "Mint Y")]
  #[account(6, writable, name = "config_pda", desc = "Config PDA")]
  #[account(7, name = "token_program", desc = "Token Program")]
  #[account(8, name = "system_program", desc = "System Program")]
  #[account(9, name = "atoken_program", desc = "AToken Program")]
  EscrowTokMake {
    decimal_x: u8,
    amount_x: u64,
    decimal_y: u8,
    amount_y: u64,
  },
  //---------------== Admin PDA
  //---------------== User PDA
  //---------------== Action PDA
} //update here and lib.rs for new functions
