import { describe, expect, test } from "bun:test";
import { lamports } from "@solana/kit";
import { SYSTEM_PROGRAM_ADDRESS } from "@solana-program/system";
import { TOKEN_PROGRAM_ADDRESS } from "@solana-program/token";
import * as vault from "../clients/js/src/generated/index";
import {
	adminAddr,
	adminKp,
	checkAcct,
	getSol,
	getTokBalc,
	hackerKp,
	mint,
	mintAuthority,
	mintAuthorityKp,
	mintKp,
	sendTxn,
	user1Addr,
	user1Kp,
	vaultProgAddr,
	vaultRent,
} from "./httpws";
import { getAta, makeATA } from "./tokens";
import { ATokenGPvbd, findPda, ll, makeSolAmt } from "./utils";

const pda_bump = await findPda(adminAddr, "vault");
const vaultPDA = pda_bump.pda;
ll(`✅ - Vault PDA: ${vaultPDA}`);
const pda_bump1 = await findPda(user1Addr, "vault");
const vaultPDA1 = pda_bump1.pda;
ll(`✅ - vaultPDA1: ${vaultPDA1}`);

const vaultAtabump = await getAta(mint, vaultPDA);
const _vaultPdaAta = vaultAtabump.ata;
const vaultAtabump1 = await getAta(mint, vaultPDA1);
const vaultAta1 = vaultAtabump1.ata;

const amtDeposit = makeSolAmt(10);
const amtWithdraw = makeSolAmt(9);

/*const base64Encoder = getBase64Encoder();
    let bytes = base64Encoder.encode(value.data[0]);
    const decoded = ammConfigDecoder.decode(bytes);
    ll(decoded);*/

//BunJs Tests: https://bun.com/docs/test/writing-tests  expect(true).toBe(true);
describe("Vault Program", () => {
	test("programs exist", async () => {
		const out1 = await checkAcct(vaultProgAddr, "Vault");
		const out2 = await checkAcct(ATokenGPvbd, "ATokenGPvbd");
		if (!out1 || !out2) {
			throw new Error(`Program does not exist`);
		}
	});
	test("User1 deposits SOL to vault1", async () => {
		ll("\n------== User1 Deposits SOL to vault1");
		const methodIx = vault.getDepositInstruction(
			{
				user: user1Kp,
				vault: vaultPDA1,
				program: vaultProgAddr,
				systemProgram: SYSTEM_PROGRAM_ADDRESS,
				amount: lamports(amtDeposit),
			},
			{
				programAddress: vaultProgAddr,
			},
		);
		await sendTxn(methodIx, user1Kp);
		ll("program execution successful");

		ll("Vault Rent:", vaultRent);
		ll("amtDeposit:", amtDeposit);
		const balc1 = await getSol(vaultPDA1, "Vault");
		expect(vaultRent + amtDeposit).toEqual(balc1.lamports);
		//assert.equal(balc1, vaultRent + amtDeposit);
	}, 10000); //Timeouts

	test("User1 withdraws SOL from vault1", async () => {
		ll("\n------== User1 Withdraws SOL from vault1");
		await getSol(vaultPDA, "Vault");

		const methodIx = vault.getWithdrawInstruction({
			user: user1Kp,
			vault: vaultPDA1,
			program: vaultProgAddr,
			amount: lamports(amtWithdraw),
		});
		await sendTxn(methodIx, user1Kp);
		ll("program execution successful");

		ll("Vault Rent:", vaultRent);
		ll("Vault amtWithdraw:", amtWithdraw);
		const balc22 = await getSol(vaultPDA1, "Vault");
		expect(vaultRent + amtDeposit - amtWithdraw).toEqual(balc22.lamports);
	}); //can withdraw from vault

	//------------------==
	test.failing("hacker cannot withdraw SOL from  vault1", async () => {
		const methodIx = vault.getWithdrawInstruction({
			user: hackerKp,
			vault: vaultPDA1,
			program: vaultProgAddr,
			amount: lamports(amtWithdraw),
		});
		await sendTxn(methodIx, hackerKp);
	});
	//------------------==
	//TODO: users can redeem SOL from pool
	//------------------==
	test("lgc init mint", async () => {
		ll("\n------== Lgc Init Mint");
		ll("payer:", adminAddr);
		ll("mint_auth:", mintAuthority);
		ll("mint:", mint);

		const methodIx = vault.getTokenLgcInitMintInstruction(
			{
				payer: adminKp,
				mint: mintKp,
				mintAuthority: mintAuthority,
				tokenProgram: TOKEN_PROGRAM_ADDRESS,
				freezeAuthorityOpt: mintAuthority,
				program: vaultProgAddr,
				systemProgram: SYSTEM_PROGRAM_ADDRESS,
				decimals: 9,
			},
			{
				programAddress: vaultProgAddr,
			},
		);
		await sendTxn(methodIx, adminKp);
		ll("program execution successful");
	}, 10000); //Timeouts

	//------------------==
	test("Lgc init ata", async () => {
		ll("\n------== Lgc Init Ata");
		const payer = adminKp;
		ll("payer:", payer.address);
		const destAddr = user1Addr;
		ll("destAddr:", destAddr);
		ll("mint:", mint);

		const atabump = await getAta(mint, destAddr);
		const ata = atabump.ata;

		const methodIx = vault.getTokenLgcInitATAInstruction(
			{
				payer: payer,
				toWallet: destAddr,
				mint: mint,
				tokenAccount: ata,
				tokenProgram: TOKEN_PROGRAM_ADDRESS,
				systemProgram: SYSTEM_PROGRAM_ADDRESS,
				atokenProgram: ATokenGPvbd,
			},
			{
				programAddress: vaultProgAddr,
			},
		);
		await sendTxn(methodIx, payer);
		ll("program execution successful");
		const balcTok = await getTokBalc(ata);
		expect(balcTok.amountUi).toBe("0");
	});

	//------------------==
	test("Lgc mint token", async () => {
		ll("\n------== Lgc Mint Token");
		ll("payer:", adminAddr);
		const destAddr = user1Addr;
		ll("destAddr:", destAddr);
		ll("mint:", mint);
		ll("mintAuthorityKp:", mintAuthorityKp.address);
		const amount = 1000;
		const atabump = await makeATA(user1Kp, destAddr, mint);
		const ata = atabump.ata;
		const balcTok1 = await getTokBalc(ata, "B4");
		expect(balcTok1.amountUi).toBe("0");

		ll("before calling program");
		const methodIx = vault.getTokLgcMintTokenInstruction(
			{
				mintAuthority: mintAuthorityKp,
				toWallet: destAddr,
				mint: mint,
				tokenAccount: ata,
				tokenProgram: TOKEN_PROGRAM_ADDRESS,
				systemProgram: SYSTEM_PROGRAM_ADDRESS,
				atokenProgram: ATokenGPvbd,
				decimals: 9,
				amount: amount * 10 ** 9,
			},
			{
				programAddress: vaultProgAddr,
			},
		);
		await sendTxn(methodIx, mintAuthorityKp);
		ll("program execution successful");

		const balcTok2 = await getTokBalc(ata, "AF");
		expect(balcTok2.amountUi).toBe(amount.toString());
	});

	//------------------==
	test("Lgc init vault pdaAta1", async () => {
		ll("\n------== Lgc Init Vault PDA ATA1");
		const payer = user1Kp;
		ll("payer:", payer.address);
		ll("vaultPDA1:", vaultPDA1);
		ll("mint:", mint);

		const methodIx = vault.getTokenLgcInitATAInstruction(
			{
				payer: payer,
				toWallet: vaultPDA1,
				mint: mint,
				tokenAccount: vaultAta1,
				tokenProgram: TOKEN_PROGRAM_ADDRESS,
				systemProgram: SYSTEM_PROGRAM_ADDRESS,
				atokenProgram: ATokenGPvbd,
			},
			{
				programAddress: vaultProgAddr,
			},
		);
		await sendTxn(methodIx, payer);
		ll("program execution successful");
		const balcTok = await getTokBalc(vaultAta1, "vaultPDA ATA");
		expect(balcTok.amountUi).toBe("0");
	});

	//------------------==
	test("Lgc User1 deposits tokens", async () => {
		ll("\n------== Lgc User1 Deposits Tokens");
		ll("payer:", user1Addr);
		ll("destAddr:", user1Addr);
		ll("mint:", mint);
		const amount = 739;
		const atabump = await makeATA(user1Kp, user1Addr, mint);
		const user1Ata = atabump.ata;
		const balcTok1 = await getTokBalc(user1Ata, "B4");
		expect(balcTok1.amountUi).toBe("1000");

		ll("before calling program");
		const methodIx = vault.getTokLgcDepositInstruction(
			{
				user: user1Kp,
				from: user1Ata,
				to: vaultAta1,
				mint: mint,
				toWallet: vaultPDA1,
				tokenProgram: TOKEN_PROGRAM_ADDRESS,
				systemProgram: SYSTEM_PROGRAM_ADDRESS,
				atokenProgram: ATokenGPvbd,
				decimals: 9,
				amount: amount * 10 ** 9,
			},
			{
				programAddress: vaultProgAddr,
			},
		);
		await sendTxn(methodIx, user1Kp);
		ll("program execution successful");

		const balcTok2a = await getTokBalc(user1Ata, "user1 ATA");
		expect(balcTok2a.amountUi).toBe("261");

		const balcTok2b = await getTokBalc(vaultAta1, "vaultAta1");
		expect(balcTok2b.amountUi).toBe(amount.toString());
	});

	//------------------==
	test("Lgc User1 withdraws token from vaultPDA1", async () => {
		ll("\n------== Lgc User1 Withdraws Tokens from VaultPDA1");
		ll("payer:", user1Kp.address);
		const destAddr = user1Addr;
		ll("destAddr:", destAddr);
		ll("mint:", mint);
		const amount = 431;
		const atabump = await makeATA(user1Kp, destAddr, mint);
		const user1Ata = atabump.ata;
		const balcTok1 = await getTokBalc(user1Ata, "B4");
		expect(balcTok1.amountUi).toBe("261");

		ll("before calling program");
		const methodIx = vault.getTokLgcWithdrawInstruction(
			{
				user: user1Kp,
				from: vaultAta1,
				to: user1Ata,
				mint: mint,
				fromWallet: vaultPDA1,
				tokenProgram: TOKEN_PROGRAM_ADDRESS,
				systemProgram: SYSTEM_PROGRAM_ADDRESS,
				atokenProgram: ATokenGPvbd,
				decimals: 9,
				amount: amount * 10 ** 9,
			},
			{
				programAddress: vaultProgAddr,
			},
		);
		await sendTxn(methodIx, user1Kp);
		ll("program execution successful");

		const balcTok2a = await getTokBalc(user1Ata, "user1 ATA"); //1000−739+431= 692
		expect(balcTok2a.amountUi).toBe("692");

		const balcTok2b = await getTokBalc(vaultAta1, "vaultAta"); //732-431 = 308
		expect(balcTok2b.amountUi).toBe("308");
	});

	//------------------==
	test("Lgc User1 pays tokens to VaultPDA", async () => {
		ll("\n------== Lgc User1 pays tokens to VaultPDA");
		ll("payer:", user1Addr);
		ll("destAddr:", user1Addr);
		ll("mint:", mint);
		const amount = 126;
		const atabump1 = await makeATA(user1Kp, user1Addr, mint);
		const user1Ata = atabump1.ata;
		const atabumpVaultPDA = await makeATA(user1Kp, vaultPDA, mint);
		const vaultPdaAta = atabumpVaultPDA.ata;

		const _balcTok1a = await getTokBalc(user1Ata, "user1 ATA");
		const _balcTok1b = await getTokBalc(vaultPdaAta, "vaultPdaAta");

		ll("before calling program");
		const methodIx = vault.getTokLgcDepositInstruction(
			{
				user: user1Kp,
				from: user1Ata,
				to: vaultPdaAta,
				toWallet: vaultPDA,
				mint: mint,
				tokenProgram: TOKEN_PROGRAM_ADDRESS,
				systemProgram: SYSTEM_PROGRAM_ADDRESS,
				atokenProgram: ATokenGPvbd,
				decimals: 9,
				amount: amount * 10 ** 9,
			},
			{
				programAddress: vaultProgAddr,
			},
		);
		await sendTxn(methodIx, user1Kp);
		ll("program execution successful");

		const balcTok2a = await getTokBalc(user1Ata, "user1 ATA"); //692-126=566
		expect(balcTok2a.amountUi).toBe("566");

		const balcTok2b = await getTokBalc(vaultPdaAta, "vaultPdaAta");
		expect(balcTok2b.amountUi).toBe(amount.toString());
	});

	//------------------==
	//TODO: users can redeem tokens from pools
	//------------------==
	//TODO: LiteSVM https://rareskills.io/post/litesvm ; Bankrun: https://www.quicknode.com/guides/solana-development/tooling/bankrun
	//amount: 100 * 10 ** 9,*/

	//------------------==
	test("xyz", async () => {
		ll("------== To Xyz");
	});
});
//if error: Attempt to load a program that does not exist. You have to deploy the program first before running this test!
