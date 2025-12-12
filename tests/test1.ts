import { describe, expect, test } from "bun:test";
import { type Address, generateKeyPairSigner, lamports } from "@solana/kit";
import { SYSTEM_PROGRAM_ADDRESS } from "@solana-program/system";
import {
	findAssociatedTokenPda,
	TOKEN_PROGRAM_ADDRESS,
} from "@solana-program/token";
import { TOKEN_2022_PROGRAM_ADDRESS } from "@solana-program/token-2022";
import * as vault from "../clients/js/src/generated/index";
import {
	adminAddr,
	adminKp,
	hackerKp,
	mint,
	mintAuthority,
	mintAuthorityKp,
	mintKp,
	rpc,
	rpcSubscriptions,
	user1Addr,
	user1Kp,
} from "./setup";
import { makeATA } from "./tokens";
import { findPda, ll, makeSolAmt, sendTxn, vaultProgAddr } from "./utils";

const ACCOUNT_DISCRIMINATOR_SIZE = 8; // same as Anchor/Rust
const U64_SIZE = 8; // u64 is 8 bytes
const VAULT_SIZE = ACCOUNT_DISCRIMINATOR_SIZE + U64_SIZE; // 16

const amtDeposit = makeSolAmt(10);
const amtWithdraw = makeSolAmt(9);

const getSol = async (account: Address, name: string) => {
	const { value: balc } = await rpc.getBalance(account).send();
	ll(name, "balc:", balc);
	return balc;
};

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

// get vault rent
const vaultRent = await rpc
	.getMinimumBalanceForRentExemption(BigInt(VAULT_SIZE))
	.send();

// Get vault PDA
const pda_bump = await findPda(adminAddr, "vault");
const vaultPDA: Address = pda_bump.pda;
ll(`✅ - Vault PDA: ${vaultPDA}`);

//BunJs Tests: https://bun.com/docs/test/writing-tests  expect(true).toBe(true);
describe("Vault Program", () => {
	test.skip("can deposit to vault", async () => {
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
		expect(vaultRent + amtDeposit).toEqual(balc1);
		//assert.equal(balc1, vaultRent + amtDeposit);
	}, 10000); //Timeouts

	test.skip("can withdraw from vault", async () => {
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
		expect(vaultRent + amtDeposit - amtWithdraw).toEqual(balc22);
	}); //can withdraw from vault

	//------------------==
	test.failing(
		"doesn't allow other users to withdraw from the vault",
		async () => {
			const methodIx = vault.getWithdrawInstruction({
				owner: hackerKp,
				vault: vaultPDA,
				program: vaultProgAddr,
				amount: lamports(amtWithdraw),
			});
			await sendTxn(methodIx, hackerKp, rpc, rpcSubscriptions);
		},
	);

	test("init lgc mint", async () => {
		ll("------== Init Lgc Mint");
		ll("payer:", adminAddr);
		ll("mint_auth:", mintAuthority);
		ll("mint:", mint);

		const methodIx = vault.getTokenLgcInitMintInstruction(
			{
				payer: adminKp,
				mint: mintKp,
				mintAuthority: mintAuthority,
				tokenProgram: TOKEN_PROGRAM_ADDRESS,
				freezeAuthorityOpt: mintAuthority,
				program: vaultProgAddr,
				systemProgram: SYSTEM_PROGRAM_ADDRESS,
				decimals: 9,
			},
			{
				programAddress: vaultProgAddr,
			},
		);
		await sendTxn(methodIx, adminKp, rpc, rpcSubscriptions);
	}, 10000);

	test("init Lgc token acct", async () => {
		ll("------== Init LgcTokenAcct");
		ll("payer:", adminAddr);
		const destAddr = user1Addr;
		ll("destAddr:", destAddr);
		ll("mint:", mint);

		const [ata, bump] = await findAssociatedTokenPda({
			mint: mint,
			owner: destAddr,
			tokenProgram: TOKEN_PROGRAM_ADDRESS,
		});
		ll("ata:", ata, "bump:", bump);

		const methodIx = vault.getTokenLgcInitTokAcctInstruction(
			{
				payer: user1Kp, // must be a system account ???
				toWallet: destAddr,
				mint: mint,
				tokenAccount: ata,
				tokenProgram: TOKEN_PROGRAM_ADDRESS,
				program: vaultProgAddr,
				systemProgram: SYSTEM_PROGRAM_ADDRESS,
				bump,
			},
			{
				programAddress: vaultProgAddr,
			},
		);
		await sendTxn(methodIx, user1Kp, rpc, rpcSubscriptions);
		const _balcTok = await rpc.getTokenAccountBalance(ata).send();
		//expect(balcTok.value.uiAmountString.toString()).toBe("100");
	});

	test("mint Lgc token", async () => {
		ll("------== Mint Lgc Token");
		ll("payer:", adminAddr);
		const destAddr = user1Addr;
		ll("destAddr:", destAddr);
		ll("mint:", mint);
		ll("mintAuthorityKp:", mintAuthorityKp.address);

		const out = await makeATA(mintAuthorityKp, destAddr, mint);
		const ata = out.ata;

		const methodIx = vault.getTokLgcMintTokenInstruction(
			{
				mintAuthority: mintAuthorityKp,
				mint: mint,
				toWallet: destAddr,
				tokenProgram: TOKEN_PROGRAM_ADDRESS,
				systemProgram: SYSTEM_PROGRAM_ADDRESS,
				tokenAccount: ata,
				decimals: 9,
				amount: 100,
			},
			{
				programAddress: vaultProgAddr,
			},
		);
		await sendTxn(methodIx, mintAuthorityKp, rpc, rpcSubscriptions);

		const balcTok = await rpc.getTokenAccountBalance(ata).send();
		expect(balcTok.value.uiAmountString.toString()).toBe("100");
	});
	//------------------==
	/*test.skip("init Lgc token acct LOW LEVEL", async () => {
		ll("------== Init LgcTokenAcct LOW LEVEL");
		ll("payer:", adminAddr);
		const destAddr = user1Addr;
		ll("destAddr:", destAddr);
		ll("mint:", mint);
		const payerKp = adminKp;

		const [ata, bump] = await findAssociatedTokenPda({
			mint: mint,
			owner: destAddr,
			tokenProgram: TOKEN_PROGRAM_ADDRESS,
		});
		ll("ata:", ata, "bump:", bump);

		const { value: latestBlockhash } = await rpc.getLatestBlockhash().send();

		const transaction = pipe(
			createTransactionMessage({
				version: 0,
			}),
			(tx) => setTransactionMessageFeePayer(payerKp.address, tx),
			(tx) => setTransactionMessageLifetimeUsingBlockhash(latestBlockhash, tx),
			(tx) =>
				appendTransactionMessageInstruction(
					vault.getTokenLgcInitTokAcctInstruction(
						{
							payer: payerKp,
							toWallet: user1Kp,
							mint: mint,
							tokenAccount: ata,
							tokenProgram: TOKEN_PROGRAM_ADDRESS,
							systemProgram: SYSTEM_PROGRAM_ADDRESS,
							bump,
						},
						{
							programAddress: vaultProgAddr,
						},
					),
					tx,
				),
			(tx) => addSignersToTransactionMessage([payerKp], tx),
		);

		const signedTransaction =
			await signTransactionMessageWithSigners(transaction);
		assertIsTransactionWithBlockhashLifetime(signedTransaction);

		const sendAndConfirmTransaction = sendAndConfirmTransactionFactory({
			rpc,
			rpcSubscriptions,
		});
		await sendAndConfirmTransaction(signedTransaction, {
			commitment: "confirmed",
		});
	});
	
	const _balcTok = await rpc.getTokenAccountBalance(ata).send();
	//expect(balcTok.value.uiAmountString.toString()).toBe("100");
  //amount: 100 * 10 ** 9,*/

	test("init Tok22 Mint", async () => {
		ll("------== Init Tok22 Mint");
		ll("payer:", adminAddr);
		ll("mint_auth:", mintAuthority);
		const mint22Kp = await generateKeyPairSigner();
		ll("mint22:", mint22Kp.address);

		const methodIx = vault.getToken2022InitMintInstruction(
			{
				payer: adminKp,
				mint: mint22Kp,
				mintAuthority: mintAuthority,
				freezeAuthorityOpt: mintAuthority,
				tokenProgram: TOKEN_2022_PROGRAM_ADDRESS,
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
	test("xyz", async () => {
		ll("------== To Xyz");
	});
});
//if error: Attempt to load a program that does not exist. You have to deploy the program first before running this test!
