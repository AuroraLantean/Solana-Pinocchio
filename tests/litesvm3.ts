/** biome-ignore-all lint/style/noNonNullAssertion: <> */
import { expect, test } from "bun:test";
import type { Keypair, PublicKey } from "@solana/web3.js";
import type { Clock } from "litesvm";
import { Status, solanaKitDecodeDev } from "./decoder";
import {
	acctExists,
	closeConfig,
	configBump,
	configPDA,
	configResize,
	initConfig,
	initSolBalc,
	setMint,
	svm,
	updateConfig,
	vault1,
	vaultO,
} from "./litesvm-utils";
import {
	as9zBn,
	bigintToBytes,
	getTime,
	ll,
	statusToByte,
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
expect(adminBalc).toStrictEqual(initSolBalc);

let signerKp: Keypair;
let _authorityKp: Keypair;
let _authority: PublicKey;
let mints: PublicKey[];
let _vault: PublicKey;
let progOwner: PublicKey;
let firstProgOwner: PublicKey;
let progAdmin: PublicKey;
let dest: PublicKey;
let tokenAmount: bigint;
let fee: bigint;
let newSize: bigint;
let isAuthorized = false;
let status: Status;
let str: string;
let funcSelector: number;
let time: number;
let bytes4bools: number[];
let bytes4u8s: number[];
let bytes4u32s: number[];
let bytes4u64s: number[];

let clock: Clock;

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

	const pdaRaw = svm.getAccount(configPDA);
	expect(pdaRaw).not.toBeNull();
	const rawAccountData = pdaRaw?.data;
	ll("rawAccountData:", rawAccountData);
	expect(pdaRaw?.owner).toEqual(vaultProgAddr);

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

	bytes4bools = [0, 0, 0, 0];
	bytes4u8s = [funcSelector, statusToByte(status), 0, 0];
	tokenAmount = as9zBn(274);
	time = getTime();
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
	clock = svm.getClock();
	clock.unixTimestamp = BigInt(time);
	svm.setClock(clock);

	updateConfig(
		bytes4bools,
		bytes4u8s,
		bytes4u32s,
		bytes4u64s,
		acct1,
		acct2,
		str,
		signerKp,
	);

	const pdaRaw = svm.getAccount(configPDA);
	expect(pdaRaw).not.toBeNull();
	const rawAccountData = pdaRaw?.data;
	ll("rawAccountData:", rawAccountData);

	const decoded = solanaKitDecodeDev(rawAccountData);
	expect(decoded.fee).toEqual(fee);
	expect(decoded.updatedAt).toEqual(time);
	expect(decoded.status).toEqual(status);
	expect(decoded.str).toEqual(str);
	expect(decoded.admin).toEqual(acct1);
});

test("extend configPDA", () => {
	ll("\n------== Extend configPDA");
	let rawAccount = svm.getAccount(configPDA);
	ll("account size:", rawAccount?.data.byteLength);

	signerKp = user1Kp;
	firstProgOwner = owner;
	newSize = 1000000n;
	configResize(signerKp, configPDA, firstProgOwner, newSize);
	rawAccount = svm.getAccount(configPDA);
	ll("account size:", rawAccount?.data.byteLength);
});

test("close configPDA", () => {
	ll("\n------== Close configPDA");
	signerKp = ownerKp;
	dest = signerKp.publicKey;
	closeConfig(signerKp, configPDA, dest);
	const rawAccount = svm.getAccount(configPDA);
	expect(rawAccount).toBeNull();
});
