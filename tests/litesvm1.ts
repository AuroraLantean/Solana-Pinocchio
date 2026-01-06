/** biome-ignore-all lint/style/noNonNullAssertion: <> */
import { expect, test } from "bun:test";

//Tutorial: <https://litesvm.github.io/litesvm/tutorial.html>
import {
	//	ACCOUNT_SIZE,	TOKEN_PROGRAM_ID,
	AccountLayout,
} from "@solana/spl-token";
import { Connection, type Keypair } from "@solana/web3.js";
import {
	depositSol,
	initBalc,
	newMint,
	sendSol,
	svm,
	vaultPDA,
	vaultPDA1,
	withdrawSol,
} from "./litesvm-utils";
import { as9zBn, bigintToBytes, bytesToBigint, ll } from "./utils";
import {
	adminAddr,
	adminKp,
	hackerKp,
	ownerKp,
	usdcMint,
	user1Addr,
	user1Kp,
} from "./web3jsSetup";

//let disc = 0; //discriminator
let payerKp: Keypair;
let amount: bigint;
let amtDeposit: bigint;
let amtWithdraw: bigint;
let amt: bigint;
let balcBf: bigint | null;
let balcAf: bigint | null;
let argData: Uint8Array<ArrayBufferLike>;
const vaultRent = 1002240n; //from Rust

balcBf = svm.getBalance(adminAddr);
ll("admin SOL:", balcBf);
expect(balcBf).toStrictEqual(initBalc);

test("transfer SOL", () => {
	amount = as9zBn(0.001);
	sendSol(svm, user1Addr, amount, adminKp);
	balcAf = svm.getBalance(user1Addr);
	expect(balcAf).toStrictEqual(amount + initBalc);
});

test("Owner Deposits SOL to VaultPDA", () => {
	ll("\n------== Owner Deposits SOL to VaultPDA");
	ll("vaultPDA:", vaultPDA.toBase58());
	payerKp = ownerKp;
	amtDeposit = as9zBn(0.46);
	argData = bigintToBytes(amtDeposit);

	depositSol(svm, vaultPDA, argData, payerKp);
	balcAf = svm.getBalance(vaultPDA);
	ll("vaultPDA SOL:", balcAf);
	expect(balcAf).toStrictEqual(vaultRent + amtDeposit);
});

test("User1 Deposits SOL to vault1", () => {
	ll("\n------== User1 Deposits SOL to vault1");
	ll("vaultPDA1:", vaultPDA1.toBase58());
	payerKp = user1Kp;
	amtDeposit = as9zBn(1.23); //1230000000n
	argData = bigintToBytes(amtDeposit);
	depositSol(svm, vaultPDA1, argData, payerKp);
	balcAf = svm.getBalance(vaultPDA1);
	ll("vaultPDA1 SOL:", balcAf);
	expect(balcAf).toStrictEqual(vaultRent + amtDeposit);
});

test("User1 Withdraws SOL from vault1", () => {
	ll("\n------== User1 Withdraws SOL from vault1");
	ll("vaultPDA1:", vaultPDA1.toBase58());
	payerKp = user1Kp;
	amtWithdraw = as9zBn(0.48); //480000000n
	argData = bigintToBytes(amtWithdraw);
	withdrawSol(svm, vaultPDA1, argData, payerKp);
	balcAf = svm.getBalance(vaultPDA1);
	ll("vaultPDA1 SOL:", balcAf);
	expect(balcAf).toStrictEqual(vaultRent + amtDeposit - amtWithdraw);
});
test.failing("hacker cannot withdraw SOL from  vault1", () => {
	ll("\n------== Hacker cannot withdraw SOL from vault1");
	payerKp = hackerKp;
	amtWithdraw = as9zBn(0.48); //480000000n
	argData = bigintToBytes(amtWithdraw);
	withdrawSol(svm, vaultPDA1, argData, payerKp);
});

//------------------==
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
	const usdc = usdcMint;
	const { rawAccount, ata: _adminUsdcAta } = newMint(svm, usdc, adminAddr, amt);
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
