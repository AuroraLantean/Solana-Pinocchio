/** biome-ignore-all lint/style/noNonNullAssertion: <> */
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
	AccountLayout,
	getAssociatedTokenAddressSync,
} from "@solana/spl-token";
import { TransactionMetadata } from "litesvm";
import {
	findVaultPda,
	getLamports,
	helloworldProgram,
	ll,
	makeUsdcMint,
	systemProgram,
	usdcMint,
	vaultProgram,
} from "./litesvm-utils";
import { bytesToBigint, lamToBytes } from "./utils";

const ownerKp = new Keypair();
const adminKp = new Keypair();
const user1Kp = new Keypair();
const user2Kp = new Keypair();
const user3Kp = new Keypair();
const hackerKp = new Keypair();
const ownerAddr = ownerKp.publicKey;
const adminAddr = adminKp.publicKey;
const user1Addr = user1Kp.publicKey;
const user2Addr = user2Kp.publicKey;
const user3Addr = user3Kp.publicKey;
const hackerAddr = hackerKp.publicKey;

const initBalc = BigInt(LAMPORTS_PER_SOL) * BigInt(10);
const svm = new LiteSVM();
svm.airdrop(ownerAddr, initBalc);
svm.airdrop(adminAddr, initBalc);
svm.airdrop(user1Addr, initBalc);
svm.airdrop(user2Addr, initBalc);
svm.airdrop(user3Addr, initBalc);
svm.airdrop(hackerAddr, initBalc);
const adminBalc = svm.getBalance(adminAddr);
ll("adminBalc:", adminBalc);

const _vaultPDA = findVaultPda(ownerAddr, "VaultPDA");
const vaultPDA1 = findVaultPda(user1Addr, "VaultPDA1");

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
	const [programId, greetedPubkey] = helloworldProgram(svm);

	const payer = new Keypair();
	svm.airdrop(payer.publicKey, BigInt(LAMPORTS_PER_SOL));
	const amt = getLamports(svm, greetedPubkey);
	ll("payer SOL balc:", amt);
	expect(amt).toEqual(LAMPORTS_PER_SOL);

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

test("lamportsBytes", () => {
	ll("\n------== lamportsBytes");
	const amountNum = 1.23;
	const lamBytes64 = lamToBytes(amountNum);

	const lam = bytesToBigint(lamBytes64);
	ll("lam:", lam); //1230000000n
});

test("User1 Deposits SOL to vault1", () => {
	ll("\n------== User1 Deposits SOL to vault1");
	const discriminator = 0;
	ll("vaultPDA1:", vaultPDA1.toBase58());
	const payer = user1Kp;
	const amtSol = 1.23;
	//ll(toLam(amtSol));

	const [programId] = vaultProgram(svm);
	ll("programId:", programId.toBase58());

	const lamBytes64 = lamToBytes(amtSol);
	const bytes = [discriminator, ...lamBytes64];
	ll("bytes:", bytes);

	const blockhash = svm.latestBlockhash();
	const ix = new TransactionInstruction({
		keys: [
			{ pubkey: payer.publicKey, isSigner: true, isWritable: true },
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

	const simRes = svm.simulateTransaction(tx);
	ll("simRes meta logs:", simRes.meta().logs());
	//ll("simRes meta prettylogs:", simRes.meta().prettyLogs());
	ll("simRes meta returnData:", simRes.meta().returnData().toString()); //simRes.err(),
	/** simRes.meta():
  computeUnitsConsumed: [class computeUnitsConsumed],
  innerInstructions: [class innerInstructions],
  logs: [class logs],
  prettyLogs: [class prettyLogs],
  returnData: [class returnData],
  signature: [class signature],
  toString: [class toString], */

	const sendRes = svm.sendTransaction(tx);
	ll("\nsendRes:", sendRes.toString()); //sendRes.err(),sendRes.meta()
	//ll("sendRes:", sendRes);
	//ll("sendRes.logs():", sendRes.logs());

	if (sendRes instanceof TransactionMetadata) {
		expect(simRes.meta().logs()).toStrictEqual(sendRes.logs());
		expect(sendRes.logs()[15]).toStrictEqual(`Program ${programId} success`);
	} else {
		throw new Error("Unexpected tx failure");
	}
	ll("after simulation");

	const lamports2a = getLamports(svm, vaultPDA1);
	ll("lamports2a:", lamports2a);
	//expect(BigInt(lamports2a)).toEqual(amtLam);
});

test("infinite usdc mint", () => {
	const adminUsdcAta = getAssociatedTokenAddressSync(usdcMint, adminAddr, true);
	const usdcToOwn = 1_000_000_000_000n;
	const rawAccount = makeUsdcMint(adminAddr, adminUsdcAta, usdcToOwn);

	expect(rawAccount).not.toBeNull();
	const rawAccountData = rawAccount?.data;
	const decoded = AccountLayout.decode(rawAccountData!);
	expect(decoded.amount).toStrictEqual(usdcToOwn);
});
/*const c = svm.getClock();
    svm.setClock(
      new Clock(c.slot, c.epochStartTimestamp, c.epoch, c.leaderScheduleEpoch, BigInt(quarterTime))    );*/
ll("LiteSVM finished");
