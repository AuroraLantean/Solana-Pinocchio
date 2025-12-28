import { expect, test } from "bun:test";
import { SYSTEM_PROGRAM_ADDRESS } from "@solana-program/system";
import * as vault from "../clients/js/src/generated/index";
import {
	acctExists,
	adminAddr,
	adminKp,
	configPDA,
	readConfigData,
	sendTxn,
	vaultProgAddr,
} from "./httpws";
import {
	ATokenGPvbd,
	getTime,
	ll,
	strToU8Array,
	toLam,
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
	ll("------== InitConfig");
	ll("payer:", adminAddr);
	const fee: bigint = toLam(111);

	const methodIx = vault.getInitConfigInstruction({
		authority: adminKp,
		configPda: configPDA,
		originalOwner: adminKp.address,
		systemProgram: SYSTEM_PROGRAM_ADDRESS,
		fee,
	});
	await sendTxn(methodIx, adminKp);
	ll("program execution successful");

	const configData = await readConfigData(configPDA, "configPDA");
	expect(configData.authority).toEqual(adminAddr);
	expect(configData.fee).toEqual(fee);
}, 10000); //Timeouts

test("UpdateConfig", async () => {
	ll("------== UpdateConfig");
	ll("payer:", adminAddr);
	const bools = new Uint8Array([0, 1, 0, 1]);
	const u8s = new Uint8Array([1, 2, 7, 8]);
	const time = getTime();
	ll("time:", time, ", u64a", toLam(37));
	const str1 = "SOL to the moon!";
	const u8array = strToU8Array(str1);
	const _str1b = u8ArrayToStr(u8array);
	const newFee = toLam(137);
	const newToken = toLam(243);

	const methodIx = vault.getUpdateConfigInstruction({
		authority: adminKp,
		configPda: configPDA,
		account1: configPDA,
		account2: configPDA,
		bools,
		u8s,
		u32s: [time, time + 1, time + 2, time + 3],
		u64s: [newFee, newToken, toLam(39), toLam(40)],
		strU8: u8array,
	});
	await sendTxn(methodIx, adminKp);
	ll("program execution successful");

	const configData = await readConfigData(configPDA, "configPDA");
	expect(configData.authority).toEqual(adminAddr);
	expect(configData.fee).toEqual(newFee);
	expect(configData.tokenBalance).toEqual(newToken);
});
