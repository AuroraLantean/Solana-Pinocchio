import { expect, test } from "bun:test";
import {
	Keypair,
	LAMPORTS_PER_SOL,
	PublicKey,
	SystemProgram,
	Transaction,
	TransactionInstruction,
} from "@solana/web3.js";
//Node-LiteSVM uses web3.js! https://github.com/LiteSVM/litesvm/tree/master/crates/node-litesvm
import { LiteSVM } from "litesvm";
/*import {
	getAssociatedTokenAddressSync,
	AccountLayout,
	ACCOUNT_SIZE,
	TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
*/
import * as vault from "../clients/js/src/generated/index";
import { getLamports, helloworldProgram, ll } from "./litesvm-utils";

export const vaultProgAddr = vault.PINOCCHIO_VAULT_PROGRAM_ADDRESS;

const adminKp = new Keypair();

test("transfer SOL", () => {
	const svm = new LiteSVM();
	svm.airdrop(adminKp.publicKey, BigInt(LAMPORTS_PER_SOL));
	const receiver = PublicKey.unique();
	const blockhash = svm.latestBlockhash();
	const transferLamports = 1_000_000n;
	const ixs = [
		SystemProgram.transfer({
			fromPubkey: adminKp.publicKey,
			toPubkey: receiver,
			lamports: transferLamports,
		}),
	];
	const tx = new Transaction();
	tx.recentBlockhash = blockhash;
	tx.add(...ixs);
	tx.sign(adminKp);
	svm.sendTransaction(tx);
	const balanceAfter = svm.getBalance(receiver);
	expect(balanceAfter).toStrictEqual(transferLamports);

	/*const c = svm.getClock();
    svm.setClock(
      new Clock(c.slot, c.epochStartTimestamp, c.epoch, c.leaderScheduleEpoch, BigInt(quarterTime))
    );*/
});

test("hello world", () => {
	const [svm, programId, greetedPubkey] = helloworldProgram();

	const payer = new Keypair();
	svm.airdrop(payer.publicKey, BigInt(LAMPORTS_PER_SOL));
	const lamports = getLamports(svm, greetedPubkey);
	ll("payer SOL balc:", lamports);
	expect(lamports).toEqual(LAMPORTS_PER_SOL);

	const blockhash = svm.latestBlockhash();

	const greetedAccountBefore = svm.getAccount(greetedPubkey);
	expect(greetedAccountBefore).not.toBeNull();
	expect(greetedAccountBefore?.data).toStrictEqual(
		new Uint8Array([0, 0, 0, 0]),
	);

	const ix = new TransactionInstruction({
		keys: [{ pubkey: greetedPubkey, isSigner: false, isWritable: true }],
		programId,
		data: Buffer.from([0]),
	});
	const tx = new Transaction();
	tx.recentBlockhash = blockhash;
	tx.add(ix);
	tx.sign(payer);
	svm.sendTransaction(tx);

	const greetedAccountAfter = svm.getAccount(greetedPubkey);
	expect(greetedAccountAfter).not.toBeNull();
	expect(greetedAccountAfter?.data).toStrictEqual(new Uint8Array([1, 0, 0, 0]));
});
/*
export const makeAccount = () => {
  const ixs = [SystemProgram.createAccount({
    fromPubkey: payer.publicKey,
    newAccountPubkey: dataAccount.publicKey,
    lamports: Number(svm.minimumBalanceForRentExamption(BigInt(4))),
    space: 4,
    programId: contractPubkey
  })]
}

*/
/*test("can deposit to vault", async () => {
	ll("------== To Deposit");
	const amtDeposit = makeSolAmt(10);
	const _methodIx = vault.getDepositInstruction(
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

	ll("Vault Rent:", vaultRent);
	ll("amtDeposit:", amtDeposit);
	const balc1 = svm.getBalance(vaultPDA);
	//const balc1 = await getSol(vaultPDA, "Vault");
	expect(vaultRent + amtDeposit).toEqual(balc1);
	//assert.equal(balc1, vaultRent + amtDeposit);
}, 10000);*/

ll("LiteSVM finished");
