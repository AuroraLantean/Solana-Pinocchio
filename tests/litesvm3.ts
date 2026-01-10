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
	acctExists,
	checkSuccess,
	configBump,
	configPDA,
	initBalc,
	initConfig,
	setMint,
	svm,
	vault1,
	vaultO,
} from "./litesvm-utils";
import {
	as9zBn,
	bigintToBytes,
	getTime,
	ll,
	statusToByte,
	strToU8Fixed,
	u32Bytes,
	u64Bytes,
} from "./utils";
import {
	admin,
	owner,
	ownerKp,
	pyusdMint,
	usdcMint,
	usdgMint,
	usdtMint,
	user1,
	user1Kp,
	vaultProgAddr,
} from "./web3jsSetup";

const adminBalc = svm.getBalance(admin);
ll("admin SOL:", adminBalc);
expect(adminBalc).toStrictEqual(initBalc);

let disc = 0; //discriminator
let signerKp: Keypair;
let _authorityKp: Keypair;
let _authority: PublicKey;
let mints: PublicKey[];
let _vault: PublicKey;
let progOwner: PublicKey;
let progAdmin: PublicKey;
let tokenAmount: bigint;
let fee: bigint;
let isAuthorized = false;
let status: Status;
let str: string;
let funcSelector: number;
let time: number;
let bytes4bools: number[];
let bytes4u8s: number[];
let bytes4u32s: number[];
let bytes4u64s: number[];
let argData: number[];
let blockhash: string;
let clock: Clock;
let ix: TransactionInstruction;
let tx: Transaction;
let simRes: FailedTransactionMetadata | SimulatedTransactionInfo;
let sendRes: FailedTransactionMetadata | TransactionMetadata;

test("Set Mints", () => {
	ll("\n------== Set Mints");
	setMint(usdcMint);
	acctExists(usdcMint);
	setMint(usdtMint);
	acctExists(usdtMint);
	setMint(pyusdMint);
	acctExists(pyusdMint);
	setMint(usdgMint);
	acctExists(usdgMint);
});
test("InitConfig", () => {
	ll("\n------== InitConfig");
	disc = 12; //discriminator
	ll("vault1:", vault1.toBase58());
	ll(`configPDA: ${configPDA}`);
	signerKp = user1Kp;
	mints = [usdcMint, usdtMint, pyusdMint, usdgMint];
	progOwner = owner;
	progAdmin = user1;
	fee = 111000000n;
	isAuthorized = true;
	status = Status.Active;
	str = "MoonDog to the Moon!";

	ll("progOwner:", progOwner.toBase58(), progOwner.toBytes());
	ll("progAdmin:", progAdmin.toBase58(), progAdmin.toBytes());
	initConfig(
		mints,
		progOwner,
		progAdmin,
		isAuthorized,
		status,
		fee,
		str,
		signerKp,
	);

	const configPDAraw = svm.getAccount(configPDA);
	expect(configPDAraw).not.toBeNull();
	const rawAccountData = configPDAraw?.data;
	ll("rawAccountData:", rawAccountData);

	const decoded = solanaKitDecodeDev(rawAccountData);
	expect(decoded.mint0).toEqual(mints[0]!);
	expect(decoded.mint1).toEqual(mints[1]!);
	expect(decoded.mint2).toEqual(mints[2]!);
	expect(decoded.mint3).toEqual(mints[3]!);
	expect(decoded.vault).toEqual(vaultO);
	expect(decoded.progOwner).toEqual(progOwner);
	expect(decoded.admin).toEqual(progAdmin);
	expect(decoded.str).toEqual(str);
	expect(decoded.fee).toEqual(fee);
	expect(decoded.solBalance).toEqual(0n);
	expect(decoded.tokenBalance).toEqual(0n);
	ll("updatedAt:", decoded.updatedAt);
	expect(decoded.isAuthorized).toEqual(isAuthorized);
	expect(decoded.status).toEqual(status);
	expect(decoded.bump).toEqual(configBump);
});

test("updateConfig + time travel", () => {
	ll("\n------== updateConfig + time travel");
	disc = 13; //discriminator
	ll(`configPDA: ${configPDA}`);
	ll("vault1:", vault1.toBase58());
	signerKp = ownerKp;
	const acct1 = admin;
	const acct2 = admin;
	fee = 123000000n;
	//const fee2 = bytesToBigint(bigintToBytes(fee));	ll("fee2:", fee2);
	isAuthorized = true;
	status = Status.Paused;
	str = "MoonDog to the Marzzz!";
	funcSelector = 1; //0 status, 1 fee, 2 admin

	//TODO: wrap below into a func
	bytes4bools = [0, 0, 0, 0];
	bytes4u8s = [funcSelector, statusToByte(status), 0, 0];
	time = getTime();
	tokenAmount = as9zBn(274);
	bytes4u32s = [
		...bigintToBytes(time, 32),
		...u32Bytes,
		...u32Bytes,
		...u32Bytes,
	];
	bytes4u64s = [
		...bigintToBytes(fee),
		...bigintToBytes(tokenAmount),
		...u64Bytes,
		...u64Bytes,
	];
	argData = [
		...bytes4bools,
		...bytes4u8s,
		...bytes4u32s,
		...bytes4u64s,
		...strToU8Fixed(str),
	];
	ll("acct1:", acct1.toBase58());
	ll("acct2:", acct2.toBase58());

	clock = svm.getClock();
	clock.unixTimestamp = BigInt(time);
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
	expect(decoded.fee).toEqual(fee);
	expect(decoded.updatedAt).toEqual(time);
	expect(decoded.status).toEqual(status);
	expect(decoded.str).toEqual(str);
	expect(decoded.admin).toEqual(acct1);
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
