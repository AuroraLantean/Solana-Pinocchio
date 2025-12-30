use litesvm::LiteSVM;
use solana_instruction::AccountMeta;
use solana_instruction::Instruction;
use solana_keypair::Keypair;
use solana_kite::{
  create_associated_token_account, create_token_mint, deploy_program, get_pda_and_bump,
  mint_tokens_to_account, send_transaction_from_instructions, SolanaKiteError,
};
use solana_pubkey::Pubkey;
use solana_signer::Signer;
//use std::cell::Cell;

pub const PROGRAM_ID: &str = "7EKqBVYSCmJbt2T8tGSmwzNKnpL29RqcJcyUr9aEEr6e";

pub const TOKEN_A_BASE: u64 = 1_000_000_000;
pub const TOKEN_B_BASE: u64 = 1_000_000_000;
pub struct TestEnvironment {
  /// The LiteSVM instance for simulating Solana transactions
  pub litesvm: LiteSVM,
  /// The escrow program ID
  pub program_id: Pubkey,
  /// The mint authority that can create and mint tokens
  pub _mint_authority: Keypair,
  /// Token mint A (the first token in escrow trades)
  pub token_mint_a: Keypair,
  /// Token mint B (the second token in escrow trades)
  pub token_mint_b: Keypair,
  /// Alice's keypair (typically the offer maker)
  pub alice: Keypair,
  /// Bob's keypair (typically the offer taker)
  pub bob: Keypair,
  /// Alice's token account for token A
  pub alice_token_account_a: Pubkey,
  /// Alice's token account for token B
  pub alice_token_account_b: Pubkey,
  /// Bob's token account for token A
  pub bob_token_account_a: Pubkey,
  /// Bob's token account for token B
  pub bob_token_account_b: Pubkey,
}
pub fn get_program_id() -> solana_pubkey::Pubkey {
  Pubkey::from_str_const(PROGRAM_ID)
  // [u8; 32])
  //new_from_array
  //Pubkey::from_str(PROGRAM_ID).unwrap()
}
//EscrowTestEnvironment
pub fn setup_escrow_test() {
  let mut litesvm = LiteSVM::new();
  let program_id = get_program_id();

  // Deploy the escrow program
  deploy_program(
    &mut litesvm,
    &program_id,
    "target/deploy/pinocchio_vault.so",
  )
  .unwrap();

  // Create and fund mint authority
  let mint_authority = Keypair::new();
  litesvm
    .airdrop(&mint_authority.pubkey(), 1_000_000_000)
    .unwrap();

  // Create token mints
  //let token_mint_a = create_token_mint(&mut litesvm, &mint_authority, 9).unwrap();
  //let token_mint_b = create_token_mint(&mut litesvm, &mint_authority, 9).unwrap();

  // Create and fund Alice and Bob
  let alice = Keypair::new();
  let bob = Keypair::new();
  litesvm.airdrop(&alice.pubkey(), 1_000_000_000).unwrap();
  litesvm.airdrop(&bob.pubkey(), 1_000_000_000).unwrap();
}
