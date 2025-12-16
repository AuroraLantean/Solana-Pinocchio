import { describe, expect, test } from "bun:test";
import type { Address } from "@solana/kit";
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

export const pda_bump = await findPda(adminAddr, "vault");
export const vaultPDA: Address = pda_bump.pda;
ll(`✅ - Vault PDA: ${vaultPDA}`);
export const vaultAtabump = await getAta(mint, vaultPDA);
export const vaultAta = vaultAtabump.ata;

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
	test.skip("can deposit to vault", async () => {
		ll("\n------== To Deposit");
		const methodIx = vault.getDepositInstruction(
			{
				owner: adminKp,
				vault: vaultPDA,
				program: vaultProgAddr,
				systemProgram: SYSTEM_PROGRAM_ADDRESS,
				amount: lamports(amtDeposit),
			},
			{
				programAddress: vaultProgAddr,
			},
		);
		await sendTxn(methodIx, adminKp);
		ll("program execution successful");

		ll("Vault Rent:", vaultRent);
		ll("amtDeposit:", amtDeposit);
		const balc1 = await getSol(vaultPDA, "Vault");
		expect(vaultRent + amtDeposit).toEqual(balc1.lamports);
		//assert.equal(balc1, vaultRent + amtDeposit);
	}, 10000); //Timeouts

	test.skip("can withdraw from vault", async () => {
		ll("\n------== To Withdraw");
		await getSol(vaultPDA, "Vault");

		const methodIx = vault.getWithdrawInstruction({
			owner: adminKp,
			vault: vaultPDA,
			program: vaultProgAddr,
			amount: lamports(amtWithdraw),
		});
		await sendTxn(methodIx, adminKp);
		ll("program execution successful");

		ll("Vault Rent:", vaultRent);
		ll("Vault amtWithdraw:", amtWithdraw);
		const balc22 = await getSol(vaultPDA, "Vault");
		expect(vaultRent + amtDeposit - amtWithdraw).toEqual(balc22.lamports);
	}); //can withdraw from vault

	//------------------==
	test.failing(
		"doesn't allow other users to withdraw from the vault",
		async () => {
			const methodIx = vault.getWithdrawInstruction({
				owner: hackerKp,
				vault: vaultPDA,
				program: vaultProgAddr,
				amount: lamports(amtWithdraw),
			});
			await sendTxn(methodIx, hackerKp);
		},
	);
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
	test("Lgc init vault pda_ata", async () => {
		ll("\n------== Lgc Init Vault PDA ATA");
		const payer = adminKp;
		ll("payer:", payer.address);
		const destAddr = vaultPDA;
		ll("destAddr:", destAddr);
		ll("mint:", mint);

		const methodIx = vault.getTokenLgcInitATAInstruction(
			{
				payer: payer,
				toWallet: destAddr,
				mint: mint,
				tokenAccount: vaultAta,
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
		const balcTok = await getTokBalc(vaultAta, "vaultPDA ATA");
		expect(balcTok.amountUi).toBe("0");
	});

	//------------------==
	test("Lgc deposit token from user1", async () => {
		ll("\n------== Lgc Deposit Token from User1");
		ll("payer:", user1Addr);
		const destAddr = user1Addr;
		ll("destAddr:", destAddr);
		ll("mint:", mint);
		ll("mintAuthorityKp:", mintAuthorityKp.address);
		const amount = 271;
		const atabump = await makeATA(user1Kp, destAddr, mint);
		const user1Ata = atabump.ata;
		const balcTok1 = await getTokBalc(user1Ata, "B4");
		expect(balcTok1.amountUi).toBe("1000");

		ll("before calling program");
		const methodIx = vault.getTokLgcDepositInstruction(
			{
				authority: user1Kp,
				from: user1Ata,
				to: vaultAta,
				mint: mint,
				toWallet: vaultPDA,
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

		const _balcTok2a = await getTokBalc(user1Ata, "vaultPDA ATA");
		//expect(balcTok2a.amountUi).toBe("729");

		const _balcTok2b = await getTokBalc(vaultAta, "AF");
		//expect(balcTok2b.amountUi).toBe(amount.toString());
	});
	//------------------==
	//TODO: LiteSVM https://rareskills.io/post/litesvm ; Bankrun: https://www.quicknode.com/guides/solana-development/tooling/bankrun
	//amount: 100 * 10 ** 9,*/

	//------------------==
	test("xyz", async () => {
		ll("------== To Xyz");
	});
});
//if error: Attempt to load a program that does not exist. You have to deploy the program first before running this test!
