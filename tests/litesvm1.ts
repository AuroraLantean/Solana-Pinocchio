/** biome-ignore-all lint/style/noNonNullAssertion: <> */
import { expect, test } from "bun:test";

//Tutorial: <https://litesvm.github.io/litesvm/tutorial.html>
import {
	//	ACCOUNT_SIZE,	TOKEN_PROGRAM_ID,
	AccountLayout,
} from "@solana/spl-token";
import {
	Connection,
	type Keypair,
	SystemProgram,
	Transaction,
	TransactionInstruction,
} from "@solana/web3.js";
import type {
	FailedTransactionMetadata,
	SimulatedTransactionInfo,
	TransactionMetadata,
} from "litesvm";
import {
	checkSuccess,
	initBalc,
	makeMint,
	svm,
	vaultPDA,
	vaultPDA1,
} from "./litesvm-utils";
import { as9zBn, bigintToBytes, bytesToBigint, ll } from "./utils";
import {
	adminAddr,
	adminKp,
	ownerKp,
	systemProgram,
	usdcMint,
	user1Addr,
	user1Kp,
	vaultProgAddr,
} from "./web3jsSetup";

let disc = 0; //discriminator
let payerKp: Keypair;
let amount: bigint;
let amt: bigint;
let argData: Uint8Array<ArrayBufferLike>;
let blockhash: string;
let ix: TransactionInstruction;
let tx: Transaction;
let simRes: FailedTransactionMetadata | SimulatedTransactionInfo;
let sendRes: FailedTransactionMetadata | TransactionMetadata;

const adminBalc = svm.getBalance(adminAddr);
ll("admin SOL:", adminBalc);
expect(adminBalc).toStrictEqual(initBalc);

test("transfer SOL", () => {
	blockhash = svm.latestBlockhash();
	amount = as9zBn(0.001);
	const ixs = [
		SystemProgram.transfer({
			fromPubkey: adminKp.publicKey,
			toPubkey: user1Addr,
			lamports: amount,
		}),
	];
	tx = new Transaction();
	tx.recentBlockhash = blockhash;
	tx.add(...ixs);
	tx.sign(adminKp);
	svm.sendTransaction(tx);
	const balanceAfter = svm.getBalance(user1Addr);
	expect(balanceAfter).toStrictEqual(amount + initBalc);
});

test("Owner Deposits SOL to VaultPDA", () => {
	ll("\n------== Owner Deposits SOL to VaultPDA");
	disc = 0; //discriminator
	ll("vaultPDA:", vaultPDA.toBase58());
	payerKp = ownerKp;
	amount = as9zBn(0.46);
	//ll(toLam(amtSol));

	argData = bigintToBytes(amount);

	blockhash = svm.latestBlockhash();
	ix = new TransactionInstruction({
		keys: [
			{ pubkey: payerKp.publicKey, isSigner: true, isWritable: true },
			{ pubkey: vaultPDA, isSigner: false, isWritable: true },
			{ pubkey: systemProgram, isSigner: false, isWritable: false },
		],
		programId: vaultProgAddr,
		data: Buffer.from([disc, ...argData]),
	});
	tx = new Transaction();
	tx.recentBlockhash = blockhash;
	tx.add(ix); //tx.add(...ixs);
	tx.sign(payerKp);

	simRes = svm.simulateTransaction(tx);
	sendRes = svm.sendTransaction(tx);
	checkSuccess(simRes, sendRes, vaultProgAddr);
});

test("User1 Deposits SOL to vault1", () => {
	ll("\n------== User1 Deposits SOL to vault1");
	disc = 0; //discriminator
	ll("vaultPDA1:", vaultPDA1.toBase58());
	payerKp = user1Kp;
	amount = as9zBn(1.23);
	//ll(toLam(amtSol));1230000000n

	argData = bigintToBytes(amount);

	blockhash = svm.latestBlockhash();
	ix = new TransactionInstruction({
		keys: [
			{ pubkey: payerKp.publicKey, isSigner: true, isWritable: true },
			{ pubkey: vaultPDA1, isSigner: false, isWritable: true },
			{ pubkey: systemProgram, isSigner: false, isWritable: false },
		],
		programId: vaultProgAddr,
		data: Buffer.from([disc, ...argData]),
	});
	tx = new Transaction();
	tx.recentBlockhash = blockhash;
	tx.add(ix); //tx.add(...ixs);
	tx.sign(payerKp);

	simRes = svm.simulateTransaction(tx);
	sendRes = svm.sendTransaction(tx);
	checkSuccess(simRes, sendRes, vaultProgAddr);
});

test("inputNum to/from Bytes", () => {
	ll("\n------== inputNum to/from Bytes");
	const amountNum = as9zBn(1.23);
	const argData64 = bigintToBytes(amountNum);
	const _amtOut64 = bytesToBigint(argData64);

	const time1 = 1766946349;
	const argData32 = bigintToBytes(time1, 32);
	const _amtOut32 = bytesToBigint(argData32);

	const u8Num = 37;
	const argDataU8 = bigintToBytes(u8Num, 8);
	const _amtOut8 = bytesToBigint(argDataU8);
});

test("mint usdc(set arbitrary account data)", () => {
	amt = 1_000_000_000_000n;
	const { rawAccount, ata: _adminUsdcAta } = makeMint(
		svm,
		usdcMint,
		adminAddr,
		amt,
	);
	expect(rawAccount).not.toBeNull();
	const rawAccountData = rawAccount?.data;
	const decoded = AccountLayout.decode(rawAccountData!);
	expect(decoded.amount).toStrictEqual(amt);
});

test.skip("copy accounts from devnet", async () => {
	const connection = new Connection("https://api.devnet.solana.com");
	const accountInfo = await connection.getAccountInfo(usdcMint);
	// the rent epoch goes above 2**53 which breaks web3.js, so just set it to 0;
	if (!accountInfo) throw new Error("accountInfo is null");
	accountInfo.rentEpoch = 0;
	svm.setAccount(usdcMint, accountInfo);
	const rawAccount = svm.getAccount(usdcMint);
	expect(rawAccount).not.toBeNull();
});
