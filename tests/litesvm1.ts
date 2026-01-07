/** biome-ignore-all lint/style/noNonNullAssertion: <> */
import { expect, test } from "bun:test";
//Tutorial: <https://litesvm.github.io/litesvm/tutorial.html>
import { Connection, type Keypair, type PublicKey } from "@solana/web3.js";
import {
	depositSol,
	initBalc,
	newAtaTest,
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
	usdtMint,
	user1Addr,
	user1Kp,
	user2Addr,
} from "./web3jsSetup";

//let disc = 0; //discriminator
let payerKp: Keypair;
let _adminUsdtAta: PublicKey;
let _user1UsdtAta: PublicKey;
let _user2UsdtAta: PublicKey;
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
	sendSol(user1Addr, amount, adminKp);
	balcAf = svm.getBalance(user1Addr);
	expect(balcAf).toStrictEqual(amount + initBalc);
});

test("Owner Deposits SOL to VaultPDA", () => {
	ll("\n------== Owner Deposits SOL to VaultPDA");
	ll("vaultPDA:", vaultPDA.toBase58());
	payerKp = ownerKp;
	amtDeposit = as9zBn(0.46);
	argData = bigintToBytes(amtDeposit);

	depositSol(vaultPDA, argData, payerKp);
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
	depositSol(vaultPDA1, argData, payerKp);
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
	withdrawSol(vaultPDA1, argData, payerKp);
	balcAf = svm.getBalance(vaultPDA1);
	ll("vaultPDA1 SOL:", balcAf);
	expect(balcAf).toStrictEqual(vaultRent + amtDeposit - amtWithdraw);
});
test.failing("hacker cannot withdraw SOL from  vault1", () => {
	ll("\n------== Hacker cannot withdraw SOL from vault1");
	payerKp = hackerKp;
	amtWithdraw = as9zBn(0.48); //480000000n
	argData = bigintToBytes(amtWithdraw);
	withdrawSol(vaultPDA1, argData, payerKp);
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
test("New ATA with balance(set arbitrary account data)", () => {
	ll("\n------== New ATA with balance(set arbitrary account data)");
	amt = 1_000_000_000n;
	newAtaTest(usdtMint, adminAddr, amt, "Admin USDT");
	newAtaTest(usdtMint, user1Addr, amt, "User1 USDT");
	newAtaTest(usdtMint, user2Addr, amt, "User2 USDT");
});
//TODO: Lgc Init Vault PDA ATA1

test.skip("copy accounts from devnet", async () => {
	const connection = new Connection("https://api.devnet.solana.com");
	const accountInfo = await connection.getAccountInfo(usdtMint);
	// the rent epoch goes above 2**53 which breaks web3.js, so just set it to 0;
	if (!accountInfo) throw new Error("accountInfo is null");
	accountInfo.rentEpoch = 0;
	svm.setAccount(usdtMint, accountInfo);
	const rawAccount = svm.getAccount(usdtMint);
	expect(rawAccount).not.toBeNull();
});
