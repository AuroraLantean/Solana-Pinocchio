import { describe, expect, test } from "bun:test";
import type { Address } from "@solana/kit";
import { SYSTEM_PROGRAM_ADDRESS } from "@solana-program/system";
import { TOKEN_2022_PROGRAM_ADDRESS } from "@solana-program/token-2022";
import * as vault from "../clients/js/src/generated/index";
import {
	adminAddr,
	adminKp,
	getTokBalc,
	mint22,
	mint22Kp,
	mintAuthority,
	mintAuthorityKp,
	sendTxn,
	user1Addr,
	user1Kp,
} from "./httpws";
import { getAta, makeATA } from "./tokens";
import { ATokenGPvbd, findPdaV2, ll } from "./utils";

export const pda_bump = await findPdaV2(adminAddr, "vault", "Vault");
export const vaultPDA: Address = pda_bump.pda;
ll(`✅ - Vault PDA: ${vaultPDA}`);

describe("Vault Program", () => {
	test("tok22 init mint", async () => {
		ll("------== Tok22 Init Mint");
		ll("payer:", adminAddr);
		ll("mint_auth:", mintAuthority);
		ll("mint22:", mint22);

		const methodIx = vault.getToken2022InitMintInstruction({
			payer: adminKp,
			mint: mint22Kp,
			mintAuthority: mintAuthority,
			freezeAuthorityOpt: mintAuthority,
			tokenProgram: TOKEN_2022_PROGRAM_ADDRESS,
			systemProgram: SYSTEM_PROGRAM_ADDRESS,
			decimals: 9,
		});
		await sendTxn(methodIx, adminKp);
		ll("program execution successful");
	}, 10000); //Timeouts

	//------------------==
	test("tok22 init ata", async () => {
		ll("------== Tok22 Init Ata");
		const payer = adminKp;
		ll("payer:", payer.address);
		const destAddr = user1Addr;
		ll("destAddr:", destAddr);
		ll("mint22:", mint22);

		const tokenProg = TOKEN_2022_PROGRAM_ADDRESS;
		const atabump = await getAta(mint22, destAddr, true);
		const ata = atabump.ata;

		const methodIx = vault.getToken2022InitATAInstruction({
			payer: payer,
			toWallet: destAddr,
			mint: mint22,
			tokenAccount: ata,
			tokenProgram: tokenProg,
			systemProgram: SYSTEM_PROGRAM_ADDRESS,
			atokenProgram: ATokenGPvbd,
		});
		await sendTxn(methodIx, payer);
		ll("program execution successful");
		//await sleep(3000);
		const balcTok2 = await getTokBalc(ata, "AF");
		expect(balcTok2.amountUi).toBe("0");
		//const balcTok = await getTokBalc2(destAddr, tokenProg);
	});

	//------------------==
	test("tok22 mint token", async () => {
		ll("------== Tok22 Mint Token");
		ll("payer:", adminAddr);
		const destAddr = user1Addr;
		ll("destAddr:", destAddr);
		ll("mint22:", mint22);
		ll("mintAuthorityKp:", mintAuthorityKp.address);
		const amount = 1022;
		const tokenProg = TOKEN_2022_PROGRAM_ADDRESS;
		const atabump = await makeATA(user1Kp, destAddr, mint22, true);
		const ata = atabump.ata;
		ll("after makeATA");

		ll("before calling program");
		const methodIx = vault.getTok22MintTokenInstruction({
			mintAuthority: mintAuthorityKp,
			toWallet: destAddr,
			mint: mint22,
			tokenAccount: ata,
			tokenProgram: tokenProg,
			systemProgram: SYSTEM_PROGRAM_ADDRESS,
			atokenProgram: ATokenGPvbd,
			decimals: 9,
			amount: amount * 10 ** 9,
		});
		await sendTxn(methodIx, mintAuthorityKp);
		ll("program execution successful");

		const balcTok2 = await getTokBalc(ata, "AF");
		expect(balcTok2.amountUi).toBe(amount.toString());
		//const balcTok2 = await getTokBalc2(destAddr, tokenProg);
	});

	//------------------==
	test("xyz", async () => {
		ll("------== To Xyz");
	});
});
//if error: Attempt to load a program that does not exist. You have to deploy the program first before running this test!
