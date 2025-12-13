import { LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import { ComputeBudget, LiteSVM } from "litesvm";

const programPath = "target/deploy/pinocchio_vault.so";
export function getLamports(svm: LiteSVM, address: PublicKey): number | null {
	const acc = svm.getAccount(address);
	return acc === null ? null : acc.lamports;
}
export function helloworldProgram(
	computeMaxUnits?: bigint,
): [LiteSVM, PublicKey, PublicKey] {
	const programId = PublicKey.unique();
	const greetedPubkey = PublicKey.unique();
	let svm = new LiteSVM();
	if (computeMaxUnits) {
		const computeBudget = new ComputeBudget();
		computeBudget.computeUnitLimit = computeMaxUnits;
		svm = svm.withComputeBudget(computeBudget);
	}
	svm.setAccount(greetedPubkey, {
		executable: false,
		owner: programId,
		lamports: LAMPORTS_PER_SOL,
		data: new Uint8Array([0, 0, 0, 0]),
	});
	svm.addProgramFromFile(programId, programPath);
	return [svm, programId, greetedPubkey];
}
