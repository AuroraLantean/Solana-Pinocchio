/** biome-ignore-all lint/style/noNonNullAssertion: <> */
import { expect, test } from "bun:test";
import type { Keypair, PublicKey } from "@solana/web3.js";
import { Transaction, TransactionInstruction } from "@solana/web3.js";
import type {
	FailedTransactionMetadata,
	SimulatedTransactionInfo,
	TransactionMetadata,
} from "litesvm";
import { Status, solanaKitDecode } from "./decoder";
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
	ll,
	statusToByte,
	strToU8Fixed,
} from "./utils";
import {
	adminAddr,
	adminKp,
	ownerAddr,
	systemProgram,
	vaultProgAddr,
} from "./web3jsSetup";

const adminBalc = svm.getBalance(adminAddr);
ll("admin SOL:", adminBalc);
expect(adminBalc).toStrictEqual(initBalc);

let disc = 0; //discriminator
let _payerKp: Keypair;
let _authorityKp: Keypair;
let _authority: PublicKey;
let progOwner: PublicKey;
let progAdmin: PublicKey;
let _amount: bigint;
let _amt: bigint;
let isAuthorized = false;
let status: Status;
let str: string;
let _strU8array: number[];
let argData: number[];
let blockhash: string;
let ix: TransactionInstruction;
let tx: Transaction;
let simRes: FailedTransactionMetadata | SimulatedTransactionInfo;
let sendRes: FailedTransactionMetadata | TransactionMetadata;

test("InitConfig", () => {
	ll("\n------== InitConfig");
	disc = 12; //discriminator
	ll("vaultPDA1:", vaultPDA1.toBase58());
	ll(`configPDA: ${configPDA}`);
	progOwner = ownerAddr;
	progAdmin = adminAddr;
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
			{ pubkey: adminAddr, isSigner: true, isWritable: true },
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
	tx.sign(adminKp);

	simRes = svm.simulateTransaction(tx);
	sendRes = svm.sendTransaction(tx);
	checkSuccess(simRes, sendRes, vaultProgAddr);

	const configPDAraw = svm.getAccount(configPDA);
	expect(configPDAraw).not.toBeNull();
	const rawAccountData = configPDAraw?.data;
	ll("rawAccountData:", rawAccountData);

	const decoded = solanaKitDecode(rawAccountData);
	expect(decoded.progOwner.toString()).toEqual(progOwner.toString());
});

test("inputNum to/from Bytes", () => {});

/*Test with Time Travel: https://litesvm.github.io/litesvm/tutorial.html#time-travel
const c = svm.getClock();
svm.setClock(
  new Clock(c.slot, c.epochStartTimestamp, c.epoch, c.leaderScheduleEpoch, BigInt(quarterTime))    );

Test with arbitrary accounts
https://litesvm.github.io/litesvm/tutorial.html#time-travel      

Copying Accounts from a live environment 
https://litesvm.github.io/litesvm/tutorial.html#copying-accounts-from-a-live-environment
*/

ll("LiteSVM3 finished");
