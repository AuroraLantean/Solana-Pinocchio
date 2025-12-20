import { test } from "bun:test";
import * as vault from "../clients/js/src/generated/index";
import {
	adminAddr,
	adminKp,
	checkAcct,
	configPDA,
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
	const out1 = await checkAcct(vaultProgAddr, "Vault");
	const out2 = await checkAcct(ATokenGPvbd, "ATokenGPvbd");
	if (!out1 || !out2) {
		throw new Error(`Program does not exist`);
	}
});

test("UpdateConfig", async () => {
	ll("------== UpdateConfig");
	ll("payer:", adminAddr);
	const bools = new Uint8Array([0, 1, 0, 1]);
	const u8s = new Uint8Array([5, 6, 7, 8]);
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
		u64s: [getLam(37), getLam(38), getLam(39), getLam(40)],
		strU8: u8array,
	});
	await sendTxn(methodIx, adminKp);
	ll("program execution successful");
}, 10000); //Timeouts
