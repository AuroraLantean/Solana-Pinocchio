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
	sendSol,
	setAta,
	setAtaCheck,
	setMint,
	svm,
	vault1,
	vaultAta1,
	vaultO,
	withdrawSol,
} from "./litesvm-utils";
import { as9zBn, bigintToBytes, ll } from "./utils";
import {
	admin,
	adminKp,
	dragonCoin,
	dragonCoinAuthority,
	dragonCoinKp,
	hackerKp,
	ownerKp,
	usdtMint,
	user1,
	user1Kp,
	user2,
} from "./web3jsSetup";

//let disc = 0; //discriminator
let signerKp: Keypair;
let mintKp: Keypair;
let mint: PublicKey;
let signer: PublicKey;
let ata: PublicKey;
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

test("initial conditions", () => {
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
test("New user ATA with balance(set arbitrary account data)", () => {
	ll("\n------== New ATA with balance(set arbitrary account data)");
	amt = 1_000_000_000n;
	setAtaCheck(usdtMint, admin, amt, "Admin USDT");
	setAtaCheck(usdtMint, user1, amt, "User1 USDT");
	setAtaCheck(usdtMint, user2, amt, "User2 USDT");
});

test("New DragonCoin Mint", () => {
	ll("\n------== New DragonCoin Mint");
	amt = 1000_000_000_000n;
	signerKp = adminKp;
	mint = dragonCoin;
	mintKp = dragonCoinKp;
	mintAuthority = dragonCoinAuthority;
	freezeAuthorityOpt = dragonCoinAuthority;
	decimals = 9;
	signer = signerKp.publicKey;
	ll("signer", signerKp.publicKey.toBase58());
	ll("mint", mint.toBase58());

	acctIsNull(mint);
	acctExists(mintAuthority);
	lgcInitMint(signerKp, mintKp, mintAuthority, freezeAuthorityOpt, decimals);
	acctExists(mint);

	ata = getAta(mint, signer);
	lgcInitAta(signerKp, signer, mint, ata);
	acctExists(ata);

	//TODO: mint tokens
	//TODO: transfer set minted tokens
});

test("Set USDT Mint and ATAs", () => {
	ll("\n------== Set USDT Mint and ATAs");
	setMint(usdtMint);
	acctExists(usdtMint);
	ll("usdtMint is set");

	amt = 1_000_000_000n;
	const ataOutAdmin = setAta(usdtMint, admin, amount);
	acctExists(ataOutAdmin.ata);

	const ataOut1 = setAta(usdtMint, user1, amount);
	acctExists(ataOut1.ata);

	const { raw: _2, ata: ata2 } = setAta(usdtMint, user2, amount);
	acctExists(ata2);
});
//TODO: transfer set USDT
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
