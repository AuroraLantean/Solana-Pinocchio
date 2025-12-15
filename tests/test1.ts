import { describe, expect, test } from "bun:test";
import type { Address } from "@solana/kit";
import { lamports } from "@solana/kit";
import { SYSTEM_PROGRAM_ADDRESS } from "@solana-program/system";
import { TOKEN_PROGRAM_ADDRESS } from "@solana-program/token";
import { TOKEN_2022_PROGRAM_ADDRESS } from "@solana-program/token-2022";
import { sleep } from "bun";
import * as vault from "../clients/js/src/generated/index";
import {
	adminAddr,
	adminKp,
	checkAcct,
	getSol,
	getTokBalc,
	hackerKp,
	mint,
	mint22,
	mint22Kp,
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
		ll("------== To Deposit");
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
		expect(vaultRent + amtDeposit).toEqual(balc1);
		//assert.equal(balc1, vaultRent + amtDeposit);
	}, 10000); //Timeouts

	test.skip("can withdraw from vault", async () => {
		ll("------== To Withdraw");
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
		expect(vaultRent + amtDeposit - amtWithdraw).toEqual(balc22);
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
		ll("------== Lgc Init Mint");
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
	}, 10000);
	//------------------==
	test("Lgc init ata", async () => {
		ll("------== Lgc Init Ata");
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
		expect(balcTok).toBe("0");
	});
	//------------------==
	test("Lgc mint token", async () => {
		ll("------== Lgc Mint Token");
		ll("payer:", adminAddr);
		const destAddr = user1Addr;
		ll("destAddr:", destAddr);
		ll("mint:", mint);
		ll("mintAuthorityKp:", mintAuthorityKp.address);

		const atabump = await makeATA(user1Kp, destAddr, mint);
		const ata = atabump.ata;
		const balcTok1 = await getTokBalc(ata, "B4");
		expect(balcTok1).toBe("0");

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
				amount: 100,
			},
			{
				programAddress: vaultProgAddr,
			},
		);
		await sendTxn(methodIx, mintAuthorityKp);
		ll("program execution successful");

		const balcTok2 = await getTokBalc(ata, "AF");
		expect(balcTok2).toBe("100");
	});
	//------------------==
	//TODO: LiteSVM https://rareskills.io/post/litesvm ; Bankrun: https://www.quicknode.com/guides/solana-development/tooling/bankrun
	//amount: 100 * 10 ** 9,*/

	test("tok22 init mint", async () => {
		ll("------== Tok22 Init Mint");
		ll("payer:", adminAddr);
		ll("mint_auth:", mintAuthority);
		ll("mint22:", mint22);

		const methodIx = vault.getToken2022InitMintInstruction(
			{
				payer: adminKp,
				mint: mint22Kp,
				mintAuthority: mintAuthority,
				freezeAuthorityOpt: mintAuthority,
				tokenProgram: TOKEN_2022_PROGRAM_ADDRESS,
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
	});

	//------------------==
	test("tok22 init ata", async () => {
		ll("------== Tok22 Init Ata");
		const payer = adminKp;
		ll("payer:", payer.address);
		const destAddr = user1Addr;
		ll("destAddr:", destAddr);
		ll("mint22:", mint22);

		const atabump = await getAta(mint22, destAddr, true);
		const ata = atabump.ata;

		const methodIx = vault.getToken2022InitATAInstruction(
			{
				payer: payer,
				toWallet: destAddr,
				mint: mint22,
				tokenAccount: ata,
				tokenProgram: TOKEN_2022_PROGRAM_ADDRESS,
				systemProgram: SYSTEM_PROGRAM_ADDRESS,
				atokenProgram: ATokenGPvbd,
			},
			{
				programAddress: vaultProgAddr,
			},
		);
		await sendTxn(methodIx, payer);
		ll("program execution successful");
		await sleep(3000);
		const balcTok = await getTokBalc(ata);
		expect(balcTok).toBe("0");
	});

	//------------------==
	test("xyz", async () => {
		ll("------== To Xyz");
	});
});
//if error: Attempt to load a program that does not exist. You have to deploy the program first before running this test!
