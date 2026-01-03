/** biome-ignore-all lint/style/noNonNullAssertion: <> */
import { expect, test } from "bun:test";
import type { Keypair, PublicKey } from "@solana/web3.js";
import { Transaction, TransactionInstruction } from "@solana/web3.js";
import type {
	Clock,
	FailedTransactionMetadata,
	SimulatedTransactionInfo,
	TransactionMetadata,
} from "litesvm";
import { Status, solanaKitDecodeDev } from "./decoder";
import {
	checkSuccess,
	configPDA,
	initBalc,
	svm,
	vaultPDA1,
} from "./litesvm-utils";
import {
	as9zBn,
	bigintToBytes,
	boolToByte,
	getTime,
	ll,
	statusToByte,
	strToU8Fixed,
} from "./utils";
import {
	adminAddr,
	ownerAddr,
	ownerKp,
	systemProgram,
	user1Addr,
	user1Kp,
	vaultProgAddr,
} from "./web3jsSetup";

const adminBalc = svm.getBalance(adminAddr);
ll("admin SOL:", adminBalc);
expect(adminBalc).toStrictEqual(initBalc);

let disc = 0; //discriminator
let signerKp: Keypair;
let _authorityKp: Keypair;
let _authority: PublicKey;
let progOwner: PublicKey;
let progAdmin: PublicKey;
let _amount: bigint;
let _amt: bigint;
let isAuthorized = false;
let status: Status;
let str: string;
let time32: number;
let zeros: number[];
let _u32s: number[];
let _u64s: number[];
let argData: number[];
let blockhash: string;
let clock: Clock;
let ix: TransactionInstruction;
let tx: Transaction;
let simRes: FailedTransactionMetadata | SimulatedTransactionInfo;
let sendRes: FailedTransactionMetadata | TransactionMetadata;

test("InitConfig", () => {
	ll("\n------== InitConfig");
	disc = 12; //discriminator
	ll("vaultPDA1:", vaultPDA1.toBase58());
	ll(`configPDA: ${configPDA}`);
	signerKp = user1Kp;
	progOwner = ownerAddr;
	progAdmin = user1Addr;
	const fee = as9zBn(111);
	isAuthorized = true;
	status = Status.Active;
	str = "MoonDog to the Moon!";
	argData = [
		boolToByte(isAuthorized),
		statusToByte(status),
		...bigintToBytes(fee),
		...strToU8Fixed(str),
	];
	ll("progOwner:", progOwner.toBase58(), progOwner.toBytes());
	ll("progAdmin:", progAdmin.toBase58(), progAdmin.toBytes());

	blockhash = svm.latestBlockhash();
	ix = new TransactionInstruction({
		keys: [
			{ pubkey: signerKp.publicKey, isSigner: true, isWritable: true },
			{ pubkey: configPDA, isSigner: false, isWritable: true },
			{ pubkey: progOwner, isSigner: false, isWritable: false },
			{ pubkey: progAdmin, isSigner: false, isWritable: false },
			{ pubkey: systemProgram, isSigner: false, isWritable: false },
		],
		programId: vaultProgAddr,
		data: Buffer.from([disc, ...argData]),
	});
	tx = new Transaction();
	tx.recentBlockhash = blockhash;
	tx.add(ix); //tx.add(...ixs);
	tx.sign(signerKp);

	simRes = svm.simulateTransaction(tx);
	sendRes = svm.sendTransaction(tx);
	checkSuccess(simRes, sendRes, vaultProgAddr);

	const configPDAraw = svm.getAccount(configPDA);
	expect(configPDAraw).not.toBeNull();
	const rawAccountData = configPDAraw?.data;
	ll("rawAccountData:", rawAccountData);

	const decoded = solanaKitDecodeDev(rawAccountData);
	expect(decoded.progOwner).toEqual(progOwner);
	expect(decoded.admin).toEqual(progAdmin);
	expect(decoded.strU8array).toEqual(str);
	expect(decoded.fee).toEqual(fee);
	expect(decoded.solBalance).toEqual(0n);
	expect(decoded.tokenBalance).toEqual(0n);
	expect(decoded.isAuthorized).toEqual(isAuthorized);
	expect(decoded.status).toEqual(status);
	ll("updatedAt:", decoded.updatedAt);
	//expect(decoded.bump).toEqual(bump);
});

test("updateConfig + time travel", () => {
	ll("\n------== updateConfig + time travel");
	disc = 13; //discriminator
	ll("vaultPDA1:", vaultPDA1.toBase58());
	ll(`configPDA: ${configPDA}`);
	signerKp = ownerKp;
	const acct1 = adminAddr;
	const acct2 = adminAddr;
	const fee = as9zBn(111);
	isAuthorized = true;
	status = Status.Active;
	str = "MoonDog to the Moon!";
	zeros = [0, 0, 0, 0, 0, 0, 0, 0];
	time32 = getTime();
	_u32s = [time32, 0, 0, 0];
	_u64s = [0, 0, 0, 0];
	argData = [
		...zeros,
		statusToByte(status),
		...bigintToBytes(fee),
		...strToU8Fixed(str),
	];
	ll("acct1:", acct1.toBase58());
	ll("acct2:", acct2.toBase58());

	clock = svm.getClock();
	clock.unixTimestamp = BigInt(time32);
	svm.setClock(clock);

	blockhash = svm.latestBlockhash();
	ix = new TransactionInstruction({
		keys: [
			{ pubkey: signerKp.publicKey, isSigner: true, isWritable: true },
			{ pubkey: configPDA, isSigner: false, isWritable: true },
			{ pubkey: acct1, isSigner: false, isWritable: false },
			{ pubkey: acct2, isSigner: false, isWritable: false },
		],
		programId: vaultProgAddr,
		data: Buffer.from([disc, ...argData]),
	});
	tx = new Transaction();
	tx.recentBlockhash = blockhash;
	tx.add(ix); //tx.add(...ixs);
	tx.sign(signerKp);

	simRes = svm.simulateTransaction(tx);
	sendRes = svm.sendTransaction(tx);
	checkSuccess(simRes, sendRes, vaultProgAddr);

	const configPDAraw = svm.getAccount(configPDA);
	expect(configPDAraw).not.toBeNull();
	const rawAccountData = configPDAraw?.data;
	ll("rawAccountData:", rawAccountData);

	const decoded = solanaKitDecodeDev(rawAccountData);
	expect(decoded.admin).toEqual(acct1);
	expect(decoded.updatedAt).toEqual(time32);
});

/*Failure Test:
const failed = svm.sendTransaction(tx);
	if (failed instanceof FailedTransactionMetadata) {
		assert.ok(failed.err().toString().includes("ProgramFailedToComplete"));
	} else {
		throw new Error("Expected transaction failure here");
	}
    
Test with arbitrary accounts
https://litesvm.github.io/litesvm/tutorial.html#time-travel      

Copying Accounts from a live environment 
https://litesvm.github.io/litesvm/tutorial.html#copying-accounts-from-a-live-environment
*/

ll("LiteSVM3 finished");
