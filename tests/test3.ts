import { expect, test } from "bun:test";
import { SYSTEM_PROGRAM_ADDRESS } from "@solana-program/system";
import * as vault from "../clients/js/src/generated/index";
import {
	acctExists,
	adminAddr,
	adminKp,
	configPDA,
	ownerKp,
	readConfigData,
	sendTxn,
	vaultProgAddr,
} from "./httpws";
import {
	ATokenGPvbd,
	as9zBn,
	getTime,
	ll,
	strToU8Array,
	u8ArrayToStr,
} from "./utils";

//describe("Vault Program", () => {});
test("programs exist", async () => {
	const out1 = await acctExists(vaultProgAddr, "Vault");
	const out2 = await acctExists(ATokenGPvbd, "ATokenGPvbd");
	if (!out1 || !out2) {
		throw new Error(`Program does not exist`);
	}
});

test("InitConfig", async () => {
	ll("\n------== InitConfig");
	ll("payer:", adminAddr);
	const fee: bigint = as9zBn(111);
	const isAuthorized = true;

	const methodIx = vault.getInitConfigInstruction({
		signer: adminKp,
		configPda: configPDA,
		progOwner: ownerKp.address,
		progAdmin: adminKp.address,
		systemProgram: SYSTEM_PROGRAM_ADDRESS,
		fee,
		isAuthorized,
	});
	await sendTxn(methodIx, adminKp);
	ll("program execution successful");

	const configData = await readConfigData(configPDA, "configPDA");
	expect(configData.progOwner).toEqual(adminAddr);
	expect(configData.admin).toEqual(adminAddr);
	expect(configData.fee).toEqual(fee);
}, 10000); //Timeouts

test("UpdateConfig", async () => {
	ll("\n------== UpdateConfig");
	ll("payer:", adminAddr);
	const bools = new Uint8Array([0, 1, 0, 1]);
	const u8s = new Uint8Array([1, 2, 7, 8]);
	const time = getTime();
	ll("time:", time, ", u64a", as9zBn(37));
	const str1 = "SOL to the moon!";
	const u8array = strToU8Array(str1);
	const _str1b = u8ArrayToStr(u8array);
	const newFee = as9zBn(112);
	const newTokBalc = as9zBn(243);

	const methodIx = vault.getUpdateConfigInstruction({
		authority: adminKp,
		configPda: configPDA,
		account1: configPDA,
		account2: configPDA,
		bools,
		u8s,
		u32s: [time, time + 1, time + 2, time + 3],
		u64s: [newFee, newTokBalc, as9zBn(39), as9zBn(40)],
		strU8: u8array,
	});
	await sendTxn(methodIx, adminKp);
	ll("program execution successful");

	const configData = await readConfigData(configPDA, "configPDA");
	expect(configData.admin).toEqual(adminAddr);
	expect(configData.fee).toEqual(newFee);
	expect(configData.tokenBalance).toEqual(newTokBalc);
});
