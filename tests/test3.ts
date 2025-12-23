import { test } from "bun:test";
import { SYSTEM_PROGRAM_ADDRESS } from "@solana-program/system";
import * as vault from "../clients/js/src/generated/index";
import {
	adminAddr,
	adminKp,
	configPDA,
	readAcctData,
	sendTxn,
	vaultProgAddr,
} from "./httpws";
import {
	ATokenGPvbd,
	getLam,
	getTime,
	ll,
	strToU8Array,
	u8ArrayToStr,
} from "./utils";

//describe("Vault Program", () => {});
test("programs exist", async () => {
	const out1 = await readAcctData(vaultProgAddr, "Vault");
	const out2 = await readAcctData(ATokenGPvbd, "ATokenGPvbd");
	if (!out1.data || !out2.data) {
		throw new Error(`Program does not exist`);
	}
});

test("InitConfig", async () => {
	ll("------== InitConfig");
	ll("payer:", adminAddr);
	const fee = BigInt(100);

	const methodIx = vault.getInitConfigInstruction({
		authority: adminKp,
		configPda: configPDA,
		originalOwner: adminKp.address,
		systemProgram: SYSTEM_PROGRAM_ADDRESS,
		fee,
	});
	await sendTxn(methodIx, adminKp);
	ll("program execution successful");

	const acct = await readAcctData(configPDA, "configPDA");
	ll("configPDA:", acct.data);
}, 10000);

test("UpdateConfig", async () => {
	ll("------== UpdateConfig");
	ll("payer:", adminAddr);
	const bools = new Uint8Array([0, 1, 0, 1]);
	const u8s = new Uint8Array([1, 6, 7, 8]);
	const time = getTime();
	ll("time:", time, ", u64a", getLam(37));
	const str1 = "SOL to the moon!";
	const u8array = strToU8Array(str1);
	const _str1b = u8ArrayToStr(u8array);

	const methodIx = vault.getUpdateConfigInstruction({
		authority: adminKp,
		pda1: configPDA,
		pda2: configPDA,
		bools,
		u8s,
		u32s: [time, time + 1, time + 2, time + 3],
		u64s: [getLam(137), getLam(38), getLam(39), getLam(40)],
		strU8: u8array,
	});
	await sendTxn(methodIx, adminKp);
	ll("program execution successful");
}); //Timeouts
