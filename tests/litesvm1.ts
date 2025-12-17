import { expect, test } from "bun:test";
import {
	Keypair,
	LAMPORTS_PER_SOL,
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
//import * as vault from "../clients/js/src/generated/index";

import {
	bigintToBytes,
	findPda1,
	getLamports,
	helloworldProgram,
	ll,
	systemProgram,
	vaultProgram,
} from "./litesvm-utils";

const adminKp = new Keypair();
const user1Kp = new Keypair();
const user2Kp = new Keypair();
const user3Kp = new Keypair();
const hackerKp = new Keypair();
const adminAddr = adminKp.publicKey;
const user1Addr = user1Kp.publicKey;
const user2Addr = user2Kp.publicKey;
const user3Addr = user3Kp.publicKey;
const hackerAddr = hackerKp.publicKey;

const initBalc = BigInt(LAMPORTS_PER_SOL) * BigInt(1);
const svm = new LiteSVM();
svm.airdrop(adminAddr, initBalc);
svm.airdrop(user1Addr, initBalc);
svm.airdrop(user2Addr, initBalc);
svm.airdrop(user3Addr, initBalc);
svm.airdrop(hackerAddr, initBalc);
const adminBalc = svm.getBalance(adminAddr);
ll("adminBalc:", adminBalc);

const _vaultPDA = findPda1(adminAddr, "VaultPDA");
const vaultPDA1 = findPda1(user1Addr, "VaultPDA1");

test("transfer SOL", () => {
	const blockhash = svm.latestBlockhash();
	const transferLamports = 1_000_000n;
	const ixs = [
		SystemProgram.transfer({
			fromPubkey: adminKp.publicKey,
			toPubkey: user1Addr,
			lamports: transferLamports,
		}),
	];
	const tx = new Transaction();
	tx.recentBlockhash = blockhash;
	tx.add(...ixs);
	tx.sign(adminKp);
	svm.sendTransaction(tx);
	const balanceAfter = svm.getBalance(user1Addr);
	expect(balanceAfter).toStrictEqual(transferLamports + initBalc);
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

test("User1 Deposits SOL to vault1", () => {
	ll("\n------== User1 Deposits SOL to vault1");
	const [svm, programId] = vaultProgram();
	const payer = user1Kp;
	const amtLam = BigInt(10) * BigInt(10) ** BigInt(9);
	const bytes = bigintToBytes(amtLam);

	const blockhash = svm.latestBlockhash();
	const ix = new TransactionInstruction({
		keys: [
			{ pubkey: user1Addr, isSigner: true, isWritable: true },
			{ pubkey: vaultPDA1, isSigner: false, isWritable: true },
			{ pubkey: systemProgram, isSigner: false, isWritable: false },
		],
		programId,
		data: Buffer.from(bytes),
	});
	const tx = new Transaction();
	tx.recentBlockhash = blockhash;
	tx.add(ix);
	tx.sign(payer);
	svm.sendTransaction(tx);

	const lamports2a = getLamports(svm, vaultPDA1);
	ll("lamports2a:", lamports2a);
	//expect(BigInt(lamports2a)).toEqual(amtLam);
});

/*const c = svm.getClock();
    svm.setClock(
      new Clock(c.slot, c.epochStartTimestamp, c.epoch, c.leaderScheduleEpoch, BigInt(quarterTime))    );*/
ll("LiteSVM finished");
