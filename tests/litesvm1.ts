/** biome-ignore-all lint/style/noNonNullAssertion: <> */
import { expect, test } from "bun:test";
//Tutorial: <https://litesvm.github.io/litesvm/tutorial.html>
import { Connection, type Keypair, type PublicKey } from "@solana/web3.js";
import { Status, solanaKitDecodeEscrowDev } from "./decoder";
import {
	acctExists,
	acctIsNull,
	ataBalCk,
	configPDA,
	depositSol,
	findPdaV1,
	findPdaV2,
	getAta,
	initConfig,
	initSolBalc,
	lgcDeposit,
	lgcInitAta,
	lgcInitMint,
	lgcMintToken,
	lgcPay,
	lgcRedeem,
	lgcWithdraw,
	makeTokEscrow,
	type PdaOut,
	sendSol,
	setAtaCheck,
	setMint,
	svm,
	vault1,
	vaultAta1,
	vaultO,
	withdrawSol,
} from "./litesvm-utils";
import { as6zBn, as9zBn, bigintAmt, ll } from "./utils";
import {
	admin,
	adminKp,
	dgcAuthorityKp,
	dragonCoinKp,
	hacker,
	hackerKp,
	owner,
	ownerKp,
	pyusdMint,
	usdcMint,
	usdgMint,
	usdtMint,
	user1,
	user1Kp,
	user2,
	user3,
} from "./web3jsSetup";

//let disc = 0; //discriminator
let signerKp: Keypair;
let mintKp: Keypair;
let mintAuthorityKp: Keypair;
let signer: PublicKey;
let mint: PublicKey;
let mintX: PublicKey;
let mintY: PublicKey;
let mintAuthority: PublicKey;
let ata: PublicKey;
let fromAta: PublicKey;
let toAta: PublicKey;
let vaultOut: PdaOut;
let escrowOut: PdaOut;
let decimals = 9;
let amount: bigint;
let amtDeposit: bigint;
let amtWithdraw: bigint;
let amt: bigint;
let decimalsX: number;
let decimalsY: number;
let amountX: bigint;
let amountY: bigint;
let id: bigint;
let balcBf: bigint | null;
let balcAf: bigint | null;
const vaultRent = 1002240n; //from Rust
const decDgc = 9;
const initDgcBalc = bigintAmt(1000, decDgc);
const initUsdcBalc = bigintAmt(1000, 6);

balcBf = svm.getBalance(admin);
ll("admin SOL:", balcBf);
expect(balcBf).toStrictEqual(initSolBalc);

test("initial conditions", () => {
	acctIsNull(vaultAta1);
});
test("transfer SOL", () => {
	amount = as9zBn(0.001);
	sendSol(user1, amount, adminKp);
	balcAf = svm.getBalance(user1);
	expect(balcAf).toStrictEqual(amount + initSolBalc);
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
	decimals = decDgc;
	amt = initDgcBalc;
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
	ataBalCk(ata, amt, "admin", 9);
	ll("can mint to admin with ATA");

	ata = getAta(mint, user1);
	acctIsNull(ata);
	lgcMintToken(mintAuthorityKp, user1, mint, ata, decimals, amt);
	ataBalCk(ata, amt, "user1", 9);
	ll("can mint to user1 without ATA");
	//TODO: transfer set minted tokens
});

test("Set USDT Mint and ATAs", () => {
	ll("\n------== Set USDT Mint and ATAs");
	setMint(usdcMint);
	acctExists(usdcMint);
	setMint(usdtMint);
	acctExists(usdtMint);
	setMint(pyusdMint);
	acctExists(pyusdMint);
	setMint(usdgMint);
	acctExists(usdgMint);

	setAtaCheck(usdcMint, admin, initUsdcBalc, "Admin USDC");
	setAtaCheck(usdcMint, user1, initUsdcBalc, "User1 USDC");
	setAtaCheck(usdcMint, user2, initUsdcBalc, "User2 USDC");
	setAtaCheck(usdcMint, user3, initUsdcBalc, "User3 USDC");
	setAtaCheck(usdcMint, hacker, initUsdcBalc, "Hacker USDC");
});

test("InitConfig", () => {
	ll("\n------== InitConfig");
	ll("vault1:", vault1.toBase58());
	ll(`configPDA: ${configPDA}`);
	signerKp = user1Kp;
	const mints = [usdcMint, usdtMint, pyusdMint, usdgMint];
	const progOwner = owner;
	const progAdmin = user1;
	const fee = 111000000n;
	const isAuthorized = true;
	const status = Status.Active;
	const str = "MoonDog to the Moon!";

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
});

test("Deposit Lgc Tokens", () => {
	ll("\n------== Deposit Lgc Tokens");
	signerKp = user1Kp;
	mint = usdcMint;
	decimals = 6;
	amt = as6zBn(370);

	signer = signerKp.publicKey;
	fromAta = getAta(mint, signer);
	vaultOut = findPdaV1(signer, "vault", "signerVault");
	toAta = getAta(mint, vaultOut.pda);

	lgcDeposit(
		signerKp,
		fromAta,
		toAta,
		vaultOut.pda,
		mint,
		configPDA,
		decimals,
		amt,
	);
	ataBalCk(toAta, as6zBn(370), "vault1");
	ataBalCk(fromAta, as6zBn(630), "user1 ");
});
test("Withdraw Lgc Tokens", () => {
	ll("\n------== Withdraw Lgc Tokens");
	signerKp = user1Kp;
	mint = usdcMint;
	decimals = 6;
	amt = as6zBn(120);

	signer = signerKp.publicKey;
	toAta = getAta(mint, signer);
	vaultOut = findPdaV1(signer, "vault", "signerVault");
	fromAta = getAta(mint, vaultOut.pda);

	lgcWithdraw(signerKp, fromAta, toAta, vaultOut.pda, mint, decimals, amt);
	ataBalCk(fromAta, as6zBn(250), "vault1");
	ataBalCk(toAta, as6zBn(750), "user1 ");
});

test("Pay Lgc Tokens", () => {
	ll("\n------== Pay Lgc Tokens");
	signerKp = user1Kp;
	mint = usdcMint;
	decimals = 6;
	amt = as6zBn(326);

	signer = signerKp.publicKey;
	fromAta = getAta(mint, signer);
	toAta = getAta(mint, vaultO);

	lgcPay(signerKp, fromAta, toAta, vaultO, mint, configPDA, decimals, amt);
	ataBalCk(toAta, amt, "vaultO");
	ataBalCk(fromAta, as6zBn(424), "user1 ");
});
test("Redeem Lgc Tokens", () => {
	ll("\n------== Redeem Lgc Tokens");
	signerKp = user1Kp;
	mint = usdcMint;
	decimals = 6;
	amt = as6zBn(37);

	signer = signerKp.publicKey;
	toAta = getAta(mint, signer);
	vaultOut = findPdaV1(owner, "vault", "Vault");
	fromAta = getAta(mint, vaultOut.pda);

	lgcRedeem(
		signerKp,
		fromAta,
		toAta,
		vaultOut.pda,
		configPDA,
		mint,
		decimals,
		amt,
	);
	ataBalCk(fromAta, as6zBn(289), "vaultO");
	ataBalCk(toAta, as6zBn(461), "user1 ");
});

test("Make Token Escrow", () => {
	ll("\n------== Make Token Escrow");
	signerKp = user1Kp;
	mintX = usdcMint;
	mintY = usdtMint;
	decimalsX = 6;
	decimalsY = 6;
	amountX = as6zBn(326);
	amountY = as6zBn(299);
	id = BigInt(0);

	signer = signerKp.publicKey;
	escrowOut = findPdaV2(signer, id, "Escrow1-id00");
	fromAta = getAta(mintX, signer);
	toAta = getAta(mintX, escrowOut.pda);

	makeTokEscrow(
		signerKp,
		fromAta,
		toAta,
		escrowOut.pda,
		mintX,
		mintY,
		configPDA,
		decimalsX,
		amountX,
		decimalsY,
		amountY,
		id,
	);
	const pdaRaw = svm.getAccount(escrowOut.pda);
	expect(pdaRaw).not.toBeNull();
	const rawAccountData = pdaRaw?.data;
	ll("rawAccountData:", rawAccountData);

	const decoded = solanaKitDecodeEscrowDev(rawAccountData);
	expect(decoded.maker).toEqual(signer);
	expect(decoded.mintX).toEqual(mintX);
	expect(decoded.mintY).toEqual(mintY);
	expect(decoded.amountY).toEqual(amountY);
	expect(decoded.id).toEqual(id);
	expect(decoded.bump).toEqual(escrowOut.bump);
	//ataBalCk(toAta, amt, "vaultO");
	//ataBalCk(fromAta, as6zBn(424), "user1 ");
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
