/** biome-ignore-all lint/style/noNonNullAssertion: <> */
import { expect, test } from "bun:test";
//Tutorial: <https://litesvm.github.io/litesvm/tutorial.html>
import { Connection, type Keypair, type PublicKey } from "@solana/web3.js";
import {
	acctExists,
	acctIsNull,
	ataBalcCheck,
	depositSol,
	findPdaV1,
	getAta,
	initBalc,
	lgcDeposit,
	lgcInitAta,
	lgcInitMint,
	lgcMintToken,
	sendSol,
	setAtaCheck,
	setMint,
	svm,
	vault1,
	vaultAta1,
	vaultO,
	withdrawSol,
} from "./litesvm-utils";
import { as9zBn, bigintAmt, ll } from "./utils";
import {
	admin,
	adminKp,
	dgcAuthorityKp,
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
let mintAuthorityKp: Keypair;
let signer: PublicKey;
let mint: PublicKey;
let mintAuthority: PublicKey;
let ata: PublicKey;
let decimals = 9;
let amount: bigint;
let amtDeposit: bigint;
let amtWithdraw: bigint;
let amt: bigint;
let balcBf: bigint | null;
let balcAf: bigint | null;
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

	depositSol(vaultO, amtDeposit, signerKp);
	balcAf = svm.getBalance(vaultO);
	ll("vaultO SOL:", balcAf);
	expect(balcAf).toStrictEqual(vaultRent + amtDeposit);
});

test("User1 Deposits SOL to vault1", () => {
	ll("\n------== User1 Deposits SOL to vault1");
	ll("vault1:", vault1.toBase58());
	signerKp = user1Kp;
	amtDeposit = as9zBn(1.23); //1230000000n

	depositSol(vault1, amtDeposit, signerKp);
	balcAf = svm.getBalance(vault1);
	ll("vault1 SOL:", balcAf);
	expect(balcAf).toStrictEqual(vaultRent + amtDeposit);
});

test("User1 Withdraws SOL from vault1", () => {
	ll("\n------== User1 Withdraws SOL from vault1");
	ll("vault1:", vault1.toBase58());
	signerKp = user1Kp;
	amtWithdraw = as9zBn(0.48); //480000000n

	withdrawSol(vault1, amtWithdraw, signerKp);
	balcAf = svm.getBalance(vault1);
	ll("vault1 SOL:", balcAf);
	expect(balcAf).toStrictEqual(vaultRent + amtDeposit - amtWithdraw);
});
test.failing("hacker cannot withdraw SOL from  vault1", () => {
	ll("\n------== Hacker cannot withdraw SOL from vault1");
	signerKp = hackerKp;
	amtWithdraw = as9zBn(0.48); //480000000n
	withdrawSol(vault1, amtWithdraw, signerKp);
});

//------------------==
test("Make DragonCoin Mint, ATA, Tokens", () => {
	ll("\n------== Make DragonCoin Mint, ATA, Tokens");
	signerKp = adminKp;
	mintKp = dragonCoinKp;
	mintAuthorityKp = dgcAuthorityKp;
	decimals = 9;
	amt = bigintAmt(1000, decimals);
	signer = signerKp.publicKey;
	mint = mintKp.publicKey;
	mintAuthority = mintAuthorityKp.publicKey;
	ll("signer", signerKp.publicKey.toBase58());
	ll("mint", mint.toBase58());
	//TODO: Codama to defined optional account
	acctIsNull(mint);
	lgcInitMint(signerKp, mintKp, mintAuthority, mintAuthority, decimals);
	acctExists(mint);

	ata = getAta(mint, signer);
	lgcInitAta(signerKp, signer, mint, ata);
	acctExists(ata);
	lgcMintToken(mintAuthorityKp, signer, mint, ata, decimals, amt);
	ataBalcCheck(ata, amt);
	ll("can mint to admin with ATA");

	ata = getAta(mint, user1);
	acctIsNull(ata);
	lgcMintToken(mintAuthorityKp, user1, mint, ata, decimals, amt);
	ataBalcCheck(ata, amt);
	ll("can mint to user1 without ATA");
	//TODO: transfer set minted tokens
});

test("Set USDT Mint and ATAs", () => {
	ll("\n------== Set USDT Mint and ATAs");
	setMint(usdtMint);
	acctExists(usdtMint);
	ll("usdtMint is set");

	amt = bigintAmt(1000, 6);
	setAtaCheck(usdtMint, admin, amt, "Admin USDT");
	setAtaCheck(usdtMint, user1, amt, "User1 USDT");
	setAtaCheck(usdtMint, user2, amt, "User2 USDT");
	//TODO: transfer set USDT
});

test("Deposit Tokens", () => {
	ll("\n------== Deposit Tokens");
	signerKp = user1Kp;
	mint = usdtMint;
	decimals = 6;
	amt = bigintAmt(370, decimals);

	signer = signerKp.publicKey;
	const fromAta = getAta(mint, signer);
	const vaultBump = findPdaV1(signer, "vault", "signerVault");
	const toAta = getAta(mint, vaultBump.pda);
	lgcDeposit(signerKp, fromAta, toAta, vaultBump.pda, mint, decimals, amt);
	ataBalcCheck(toAta, bigintAmt(370, 6));
	ataBalcCheck(fromAta, bigintAmt(630, 6));
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
