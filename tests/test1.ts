import assert from "node:assert";
import { before, describe, it } from "node:test";
//import { test, expect, mock } from "bun:test";
import {
	airdropFactory,
	appendTransactionMessageInstruction,
	assertIsTransactionWithBlockhashLifetime,
	createSolanaRpc,
	createSolanaRpcSubscriptions,
	createTransactionMessage,
	generateKeyPairSigner,
	getAddressEncoder,
	getProgramDerivedAddress,
	getSignatureFromTransaction,
	getUtf8Encoder,
	lamports,
	pipe,
	sendAndConfirmTransactionFactory,
	setTransactionMessageFeePayer,
	setTransactionMessageLifetimeUsingBlockhash,
	signTransactionMessageWithSigners,
} from "@solana/kit";
import { SYSTEM_PROGRAM_ADDRESS } from "@solana-program/system";
import * as vault from "../clients/js/src/generated/index";

const vaultProgAddr = vault.PINOCCHIO_VAULT_PROGRAM_ADDRESS;
const ACCOUNT_DISCRIMINATOR_SIZE = 8; // same as Anchor/Rust
const U64_SIZE = 8; // u64 is 8 bytes
const VAULT_SIZE = ACCOUNT_DISCRIMINATOR_SIZE + U64_SIZE; // 16
const decimalsSOL = BigInt(9);
const baseSOL = BigInt(10) ** decimalsSOL;
//const LAMPORTS_PER_SOL = baseSOL;
const ll = console.log;
const amtAirdrop = BigInt(100) * baseSOL;
const amtDeposit = BigInt(10) * baseSOL;
const amtWithdraw = BigInt(9) * baseSOL;

//BunJs Tests: https://bun.com/docs/test/writing-tests
describe("Vault Program", () => {
	let rpc: any;
	let rpcSubscriptions: any;
	let signer: any;
	let vaultRent: bigint;
	let vaultPDA: any;
	let airdrop: any;

	//https://bun.com/docs/test: beforeAll, beforeEach
	before(async () => {
		// Establish connection to Solana cluster
		const httpProvider = "http://127.0.0.1:8899";
		const wssProvider = "ws://127.0.0.1:8900";
		rpc = createSolanaRpc(httpProvider);
		rpcSubscriptions = createSolanaRpcSubscriptions(wssProvider);
		ll(`✅ - Established connection to ${httpProvider}`);

		const { value } = await rpc
			.getAccountInfo(vaultProgAddr, { encoding: "base64" })
			.send();
		if (!value || !value?.data) {
			throw new Error(`Program does not exist: ${vaultProgAddr.toString()}`);
		}
		ll("✅ - Program exits!");
		/*const base64Encoder = getBase64Encoder();
    let bytes = base64Encoder.encode(value.data[0]);
    const decoded = ammConfigDecoder.decode(bytes);
    ll(decoded);*/

		//https://www.solanakit.com/docs/getting-started/signers
		// Generate signers
		signer = await generateKeyPairSigner();
		//import secret from './my-keypair.json';
		//const user2 = await createKeyPairSignerFromBytes(new Uint8Array(secret));
		const signerAddress = await signer.address;
		ll(`✅ - New signer address: ${signerAddress}`);

		// Airdrop SOL to signer
		airdrop = airdropFactory({ rpc, rpcSubscriptions });
		await airdrop({
			commitment: "confirmed",
			lamports: lamports(amtAirdrop),
			recipientAddress: signerAddress,
		});
		ll(`✅ - Airdropped SOL to Signer: ${signerAddress}`);

		// get vault rent
		vaultRent = await rpc.getMinimumBalanceForRentExemption(VAULT_SIZE).send();

		// Get vault PDA
		const seedSigner = getAddressEncoder().encode(await signer.address);
		const seedTag = getUtf8Encoder().encode("vault");

		ll("vaultProgAddr:", vaultProgAddr);
		const pdas = await getProgramDerivedAddress({
			programAddress: vaultProgAddr,
			seeds: [seedTag, seedSigner],
		});
		vaultPDA = pdas[0];
		ll(`✅ - Vault PDA: ${vaultPDA}`);
	});

	//------------------==
	it("can deposit to vault", async () => {
		ll("------== To Deposit");
		const depositIx = vault.getDepositInstruction(
			{
				owner: signer,
				vault: vaultPDA,
				program: vaultProgAddr,
				systemProgram: SYSTEM_PROGRAM_ADDRESS,
				amount: lamports(amtDeposit),
			},
			{
				programAddress: vaultProgAddr,
			},
		);

		ll("here deposit-02");
		const { value: latestBlockhash1 } = await rpc.getLatestBlockhash().send();

		ll("here deposit-03");
		const txnMesg1 = pipe(
			createTransactionMessage({ version: 0 }),
			(tx) => setTransactionMessageFeePayer(signer.address, tx),
			(tx) => setTransactionMessageLifetimeUsingBlockhash(latestBlockhash1, tx),
			(tx) => appendTransactionMessageInstruction(depositIx, tx),
		);

		ll("here deposit-04");
		//https://www.solanakit.com/docs/getting-started/send-transaction#confirmation-strategies
		// Sign and send transaction
		const signedTransaction1 =
			await signTransactionMessageWithSigners(txnMesg1);

		assertIsTransactionWithBlockhashLifetime(signedTransaction1);

		const sendAndConfirmTransaction = sendAndConfirmTransactionFactory({
			rpc,
			rpcSubscriptions,
		});
		//lastValidBlockHeight
		ll("here deposit-05");
		await sendAndConfirmTransaction(signedTransaction1, {
			commitment: "confirmed",
		}); //"confirmRecentTransaction" | "rpc" | "transaction"

		ll("here deposit-06");
		const signature1 = getSignatureFromTransaction(signedTransaction1);
		ll("Transaction signature:", signature1);

		ll("here deposit-07");
		const { value: balc1 } = await rpc.getBalance(vaultPDA.toString()).send();
		ll("Vault Rent:", vaultRent);
		ll("amtDeposit:", amtDeposit);
		ll("Vault balc:", balc1);
		assert.equal(balc1, vaultRent + amtDeposit);
		//expect(vaultRent + amtDeposit).toBe(value);
	}); //can deposit to vault
	/* BunJs
  test.serial("first test", ()=>{...})
  expect(true).toBe(true);
  expect(1 + 1).toBe(2);
  expect(sharedState).toBe(1);
 */
	//------------------==
	it("can withdraw from vault", async () => {
		ll("------== To Withdraw");
		const { value: balc21 } = await rpc.getBalance(vaultPDA.toString()).send();
		ll("Vault balc:", balc21);

		const withdrawIx = vault.getWithdrawInstruction({
			owner: signer,
			vault: vaultPDA,
			program: vaultProgAddr,
			amount: lamports(amtWithdraw),
		});

		const { value: latestBlockhash } = await rpc.getLatestBlockhash().send();
		const txnMesg = pipe(
			createTransactionMessage({ version: 0 }),
			(tx) => setTransactionMessageFeePayer(signer.address, tx),
			(tx) => setTransactionMessageLifetimeUsingBlockhash(latestBlockhash, tx),
			(tx) => appendTransactionMessageInstruction(withdrawIx, tx),
		);

		const signedTransaction = await signTransactionMessageWithSigners(txnMesg);
		assertIsTransactionWithBlockhashLifetime(signedTransaction);

		const sendAndConfirmTransaction = sendAndConfirmTransactionFactory({
			rpc,
			rpcSubscriptions,
		});

		await sendAndConfirmTransaction(signedTransaction, {
			commitment: "confirmed",
		});

		const signature2 = getSignatureFromTransaction(signedTransaction);
		ll("Transaction signature:", signature2);

		const { value: balc22 } = await rpc.getBalance(vaultPDA.toString()).send();
		ll("Vault Rent:", vaultRent);
		ll("Vault amtWithdraw:", amtWithdraw);
		ll("Vault balc:", balc22);
		assert.equal(balc22, vaultRent + amtDeposit - amtWithdraw);
	}); //can withdraw from vault

	//------------------==
	//test.failing("fail test",)_=>{...})
	it("doesn't allow other users to withdraw from the vault", async () => {
		// signer that DOES NOT own the vault
		const otherSigner = await generateKeyPairSigner();

		const withdrawIx = vault.getWithdrawInstruction({
			owner: otherSigner,
			vault: vaultPDA,
			program: vaultProgAddr,
			amount: lamports(amtWithdraw),
		});

		const { value: latestBlockhash } = await rpc.getLatestBlockhash().send();
		const tx = pipe(
			createTransactionMessage({ version: 0 }),
			(tx) => setTransactionMessageFeePayer(otherSigner.address, tx),
			(tx) => setTransactionMessageLifetimeUsingBlockhash(latestBlockhash, tx),
			(tx) => appendTransactionMessageInstruction(withdrawIx, tx),
		);

		const signedTransaction = await signTransactionMessageWithSigners(tx);
		assertIsTransactionWithBlockhashLifetime(signedTransaction);

		const sendAndConfirmTransaction = sendAndConfirmTransactionFactory({
			rpc,
			rpcSubscriptions,
		});

		await assert.rejects(
			sendAndConfirmTransaction(signedTransaction, {
				commitment: "confirmed",
			}),
			{
				message: "Transaction simulation failed",
			},
		);
	});
});
//if error: Attempt to load a program that does not exist. You have to deploy the program first before running this test!
