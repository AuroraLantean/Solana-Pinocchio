import assert from "node:assert";
import { before, describe, it } from "node:test";
//import { test, expect, mock } from "bun:test";
import {
	type Address,
	airdropFactory,
	createSolanaRpc,
	createSolanaRpcSubscriptions,
	generateKeyPairSigner,
	lamports,
} from "@solana/kit";
import { SYSTEM_PROGRAM_ADDRESS } from "@solana-program/system";
import {
	findAssociatedTokenPda,
	TOKEN_PROGRAM_ADDRESS,
} from "@solana-program/token";
import * as vault from "../clients/js/src/generated/index";
import { findPda, ll, sendTxn, vaultProgAddr } from "./utils";

const ACCOUNT_DISCRIMINATOR_SIZE = 8; // same as Anchor/Rust
const U64_SIZE = 8; // u64 is 8 bytes
const VAULT_SIZE = ACCOUNT_DISCRIMINATOR_SIZE + U64_SIZE; // 16
const decimalsSOL = BigInt(9);
const baseSOL = BigInt(10) ** decimalsSOL;
//const LAMPORTS_PER_SOL = baseSOL;

const amtAirdrop = BigInt(100) * baseSOL;
const amtDeposit = BigInt(10) * baseSOL;
const amtWithdraw = BigInt(9) * baseSOL;

// biome-ignore lint/suspicious/noExplicitAny: <>
let rpc: any;
// biome-ignore lint/suspicious/noExplicitAny: <>
let rpcSubscriptions: any;
const httpProvider = "http://127.0.0.1:8899";
const wssProvider = "ws://127.0.0.1:8900";

//https://www.solanakit.com/docs/getting-started/signers
const adminKp = await generateKeyPairSigner(); //KeyPairSigner<string>;
const mintAuthorityKp = await generateKeyPairSigner();
const user1Kp = await generateKeyPairSigner();
const hackerKp = await generateKeyPairSigner();
//import secret from './my-keypair.json';
//const user2 = await createKeyPairSignerFromBytes(new Uint8Array(secret));
const adminAddr = adminKp.address;
const mintAuthority = mintAuthorityKp.address;
const user1Addr = user1Kp.address;
ll(`✅ adminAddr ${adminAddr}`);
ll(`✅ mintAuthority: ${mintAuthority}`);
ll(`✅ user1Addr: ${user1Addr}`);

let vaultPDA: Address;
let vaultRent: bigint;

const getSol = async (account: Address, name: string) => {
	const { value: balc } = await rpc.getBalance(account.toString()).send();
	ll(name, "balc:", balc);
	return balc;
};
//BunJs Tests: https://bun.com/docs/test/writing-tests
describe("Vault Program", () => {
	//https://bun.com/docs/test: beforeAll, beforeEach
	before(async () => {
		// Establish connection to Solana cluster
		rpc = createSolanaRpc(httpProvider);
		rpcSubscriptions = createSolanaRpcSubscriptions(wssProvider);
		ll(`✅ - Established connection to ${httpProvider}`);

		ll("vaultProgAddr:", vaultProgAddr);
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

		// Airdrop SOL to admin
		const airdrop = airdropFactory({ rpc, rpcSubscriptions });
		await airdrop({
			commitment: "confirmed",
			lamports: lamports(amtAirdrop),
			recipientAddress: adminAddr,
		});
		await airdrop({
			commitment: "confirmed",
			lamports: lamports(amtAirdrop),
			recipientAddress: user1Addr,
		});
		await airdrop({
			commitment: "confirmed",
			lamports: lamports(amtAirdrop),
			recipientAddress: hackerKp.address,
		});
		ll(`✅ - Airdropped SOL to Admin and user1Addr`);

		// get vault rent
		vaultRent = await rpc.getMinimumBalanceForRentExemption(VAULT_SIZE).send();

		// Get vault PDA
		const pda_bump = await findPda(adminAddr, "vault");
		vaultPDA = pda_bump.pda;
		ll(`✅ - Vault PDA: ${vaultPDA}`);
	});

	//------------------==
	it("can deposit to vault", async () => {
		ll("------== To Deposit");
		const methodIx = vault.getDepositInstruction(
			{
				owner: adminKp,
				vault: vaultPDA,
				program: vaultProgAddr,
				systemProgram: SYSTEM_PROGRAM_ADDRESS,
				amount: lamports(amtDeposit),
			},
			{
				programAddress: vaultProgAddr,
			},
		);

		await sendTxn(methodIx, adminKp, rpc, rpcSubscriptions);

		ll("Vault Rent:", vaultRent);
		ll("amtDeposit:", amtDeposit);
		const balc1 = await getSol(vaultPDA, "Vault");
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
		await getSol(vaultPDA, "Vault");

		const methodIx = vault.getWithdrawInstruction({
			owner: adminKp,
			vault: vaultPDA,
			program: vaultProgAddr,
			amount: lamports(amtWithdraw),
		});

		await sendTxn(methodIx, adminKp, rpc, rpcSubscriptions);

		ll("Vault Rent:", vaultRent);
		ll("Vault amtWithdraw:", amtWithdraw);
		const balc22 = await getSol(vaultPDA, "Vault");
		assert.equal(balc22, vaultRent + amtDeposit - amtWithdraw);
	}); //can withdraw from vault

	//------------------==
	//test.failing("fail test",)_=>{...})
	it("doesn't allow other users to withdraw from the vault", async () => {
		const methodIx = vault.getWithdrawInstruction({
			owner: hackerKp,
			vault: vaultPDA,
			program: vaultProgAddr,
			amount: lamports(amtWithdraw),
		});

		await sendTxn(methodIx, hackerKp, rpc, rpcSubscriptions, false);
	});

	it("init LgcMint", async () => {
		ll("------== Init LgcMint");
		ll("payer:", adminAddr);
		ll("mint_auth:", mintAuthority);
		const mintKp = await generateKeyPairSigner();
		ll("mint:", mintKp.address);

		const methodIx = vault.getTokenLgcInitMintInstruction(
			{
				payer: adminKp,
				mint: mintKp,
				mintAuthority: mintAuthority,
				freezeAuthorityOpt: mintAuthority,
				tokenProgram: TOKEN_PROGRAM_ADDRESS,
				program: vaultProgAddr,
				systemProgram: SYSTEM_PROGRAM_ADDRESS,
				decimals: 9,
			},
			{
				programAddress: vaultProgAddr,
			},
		);
		await sendTxn(methodIx, adminKp, rpc, rpcSubscriptions);
	});

	it("init Lgc token acct", async () => {
		ll("------== Init LgcTokenAcct");
		ll("payer:", adminAddr);
		ll("mint_auth:", mintAuthority);
		const pda_bump = await findPda(adminAddr, "mint");
		const mint = pda_bump.pda;
		const owner = adminAddr;
		ll("owner:", owner);
		const [ata] = await findAssociatedTokenPda({
			mint,
			owner,
			tokenProgram: TOKEN_PROGRAM_ADDRESS,
		});
		ll("token_account ata:", ata);

		const methodIx = vault.getTokenLgcInitTokAcctInstruction(
			{
				payer: adminKp,
				mint: mint,
				owner: owner,
				tokenAccount: ata,
				tokenProgram: TOKEN_PROGRAM_ADDRESS,
				program: vaultProgAddr,
				systemProgram: SYSTEM_PROGRAM_ADDRESS,
			},
			{
				programAddress: vaultProgAddr,
			},
		);
		await sendTxn(methodIx, adminKp, rpc, rpcSubscriptions);
	});

	/*it("init Tok22 Mint", async () => {
		ll("------== Init Tok22 Mint");
		const mintKp = await generateKeyPairSigner();

		const [ata] = await findAssociatedTokenPda({
			mint: mintKp.address,
			owner: adminAddr,
			tokenProgram: TOKEN_PROGRAM_LEGACY,
		});
		ll("ata: ", ata);

		// unauthorized signer or writable account
		const methodIx = vault.getToken2022InitMintInstruction({
			mintAuthority: adminKp,
			mint: mintKp.address,
			tokenProgram: TOKEN_PROGRAM_LEGACY,
			freezeAuthorityOpt: adminAddr,
			decimals: 9,
		});

		await sendTxn(methodIx, adminKp, rpc, rpcSubscriptions);
	});
	it("xyz", async () => {
		ll("------== To Xyz");
		await getSol(vaultPDA, "Vault");
	});*/
});
//if error: Attempt to load a program that does not exist. You have to deploy the program first before running this test!
