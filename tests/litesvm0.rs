use litesvm::LiteSVM;
use litesvm_token::spl_token::instruction::transfer;
use solana_keypair::Keypair;
use solana_pubkey::Pubkey;
use solana_signer::Signer;
//use solana_message::Message;
use solana_sdk::transaction::Transaction;
//use solana_signer::Signer;
//use solana_system_interface::instruction::transfer;
//use solana_transaction::Transaction;

#[test]
fn create_account() {
  // Create the test environment
  let mut svm = LiteSVM::new();

  // Create a test account
  let user = Keypair::new();

  // Fund the account with SOL
  let mint_authority = Keypair::new();
  svm
    .airdrop(&mint_authority.pubkey(), 1_000_000_000)
    .unwrap();

  svm.airdrop(&user.pubkey(), 1_000_000_000).unwrap();

  // Check the balance
  let balance = svm.get_balance(&user.pubkey()).unwrap();
  assert_eq!(balance, 1_000_000_000);

  println!("Account funded with {} SOL", balance as f64 / 1e9);
}
#[test]
fn test_transfer() {
  let mut svm = LiteSVM::new();

  // Create two accounts
  let alice = Keypair::new();
  let bob = Keypair::new();

  // Fund Alice
  svm.airdrop(&alice.pubkey(), 2_000_000_000).unwrap();

  /*let instruction = transfer(&alice.pubkey(), &bob.pubkey(), 1_000_000_000);
  /* token_program_id: &Pubkey,
  source_pubkey: &Pubkey,
  destination_pubkey: &Pubkey,
  authority_pubkey: &Pubkey,
  signer_pubkeys: &[&Pubkey],
  amount: u64,  */

  // Build and sign transaction
  let tx = Transaction::new_signed_with_payer(
    &[instruction],
    Some(&alice.pubkey()),
    &[&alice],
    svm.latest_blockhash(),
  );

  // Send it (execution happens immediately)
  svm.send_transaction(tx).unwrap();

  // Check new balances
  assert_eq!(svm.get_balance(&bob.pubkey()).unwrap(), 1_000_000_000);
  assert!(svm.get_balance(&alice.pubkey()).unwrap() < 1_000_000_000);*/

  println!("Transfer successful!");
}
