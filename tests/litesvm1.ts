/** biome-ignore-all lint/style/noNonNullAssertion: <> */
import { expect, test } from "bun:test";
//Tutorial: <https://litesvm.github.io/litesvm/tutorial.html>
import { Connection, type Keypair, type PublicKey } from "@solana/web3.js";
import {
	acctExists,
	acctIsNull,
	depositSol,
	getAta,
	initBalc,
	lgcInitAta,
	lgcInitMint,
	newAtaTest,
	sendSol,
	svm,
	vault1,
	vaultAta1,
	vaultO,
	withdrawSol,
} from "./litesvm-utils";
import { as9zBn, bigintToBytes, bytesToBigint, ll } from "./utils";
import {
	admin,
	adminKp,
	dragonCoin,
	dragonCoinAuthority,
	hackerKp,
	ownerKp,
	usdtMint,
	user1,
	user1Kp,
	user2,
} from "./web3jsSetup";

//let disc = 0; //discriminator
let signerKp: Keypair;
let _mintKp: Keypair;
let mint: PublicKey;
let mintAuthority: PublicKey;
let freezeAuthorityOpt: PublicKey;
let decimals = 9;
let amount: bigint;
let amtDeposit: bigint;
let amtWithdraw: bigint;
let amt: bigint;
let balcBf: bigint | null;
let balcAf: bigint | null;
let argData: Uint8Array<ArrayBufferLike>;
const vaultRent = 1002240n; //from Rust

balcBf = svm.getBalance(admin);
ll("admin SOL:", balcBf);
expect(balcBf).toStrictEqual(initBalc);

test("inittial conditions", () => {
	acctIsNull(vaultAta1);
});
test("transfer SOL", () => {
	amount = as9zBn(0.001);
	sendSol(user1, amount, adminKp);
	balcAf = svm.getBalance(user1);
	expect(balcAf).toStrictEqual(amount + initBalc);
});

test("Owner Deposits SOL to VaultPDA", () => {
	ll("\n------== Owner Deposits SOL to VaultPDA");
	ll("vaultO:", vaultO.toBase58());
	signerKp = ownerKp;
	amtDeposit = as9zBn(0.46);
	argData = bigintToBytes(amtDeposit);

	depositSol(vaultO, argData, signerKp);
	balcAf = svm.getBalance(vaultO);
	ll("vaultO SOL:", balcAf);
	expect(balcAf).toStrictEqual(vaultRent + amtDeposit);
});

test("User1 Deposits SOL to vault1", () => {
	ll("\n------== User1 Deposits SOL to vault1");
	ll("vault1:", vault1.toBase58());
	signerKp = user1Kp;
	amtDeposit = as9zBn(1.23); //1230000000n
	argData = bigintToBytes(amtDeposit);

	depositSol(vault1, argData, signerKp);
	balcAf = svm.getBalance(vault1);
	ll("vault1 SOL:", balcAf);
	expect(balcAf).toStrictEqual(vaultRent + amtDeposit);
});

test("User1 Withdraws SOL from vault1", () => {
	ll("\n------== User1 Withdraws SOL from vault1");
	ll("vault1:", vault1.toBase58());
	signerKp = user1Kp;
	amtWithdraw = as9zBn(0.48); //480000000n
	argData = bigintToBytes(amtWithdraw);

	withdrawSol(vault1, argData, signerKp);
	balcAf = svm.getBalance(vault1);
	ll("vault1 SOL:", balcAf);
	expect(balcAf).toStrictEqual(vaultRent + amtDeposit - amtWithdraw);
});
test.failing("hacker cannot withdraw SOL from  vault1", () => {
	ll("\n------== Hacker cannot withdraw SOL from vault1");
	signerKp = hackerKp;
	amtWithdraw = as9zBn(0.48); //480000000n
	argData = bigintToBytes(amtWithdraw);
	withdrawSol(vault1, argData, signerKp);
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
test("New user ATA with balance(set arbitrary account data)", () => {
	ll("\n------== New ATA with balance(set arbitrary account data)");
	amt = 1_000_000_000n;
	newAtaTest(usdtMint, admin, amt, "Admin USDT");
	newAtaTest(usdtMint, user1, amt, "User1 USDT");
	newAtaTest(usdtMint, user2, amt, "User2 USDT");
});

test("New DragonCoin Mint", () => {
	ll("\n------== New DragonCoin Mint");
	amt = 1_000_000_000n;
	signerKp = adminKp;
	mint = dragonCoin;
	mintAuthority = dragonCoinAuthority;
	freezeAuthorityOpt = dragonCoinAuthority;
	decimals = 9;
	ll("signer", signerKp.publicKey.toBase58());
	ll("mint", mint.toBase58());

	acctIsNull(mint);
	acctExists(mintAuthority);
	//TODO: mint -> mintKp & multi sign
	lgcInitMint(signerKp, mint, mintAuthority, freezeAuthorityOpt, decimals);
	//setNewMint(dragonCoin)
});
test("Set USDT Mint", () => {
	ll("\n------== Set USDT Mint");
	amt = 1_000_000_000n;
	signerKp = adminKp;
	freezeAuthorityOpt = admin;
	_mintKp = adminKp;
	//setNewMint(usdtMint); not working
});
test("New Vault ATA", () => {
	ll("\n------== New Vault ATA");
	amt = 1_000_000_000n;
	signerKp = user1Kp;
	const vaultAta1 = getAta(usdtMint, vault1);
	acctIsNull(vaultAta1);
	lgcInitAta(signerKp, vault1, usdtMint, vaultAta1);
});

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
