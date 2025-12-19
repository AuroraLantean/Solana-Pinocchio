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
import { ATokenGPvbd, getLam, getTime, ll } from "./utils";

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
	const u8s = new Uint8Array([0, 1, 2, 3, 4, 5, 6, 7]);
	const time = getTime();
	ll("time:", time, ", u64a", getLam(37));

	const methodIx = vault.getUpdateConfigInstruction({
		authority: adminKp,
		pda1: configPDA,
		pda2: configPDA,
		u8s,
		u32s: [time, time + 1, time + 2, time + 3],
		u64s: [getLam(37), getLam(38), getLam(39), getLam(40)],
	});
	await sendTxn(methodIx, adminKp);
	ll("program execution successful");
}, 10000); //Timeouts
