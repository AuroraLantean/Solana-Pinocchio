import type { Keypair } from "@solana/web3.js";
import {
	LAMPORTS_PER_SOL,
	PublicKey,
	SystemProgram,
	Transaction,
} from "@solana/web3.js";
import { ComputeBudget, LiteSVM } from "litesvm";

//const programPath = "target/deploy/pinocchio_vault.so";
const programPath = "program_bytes/counter.so";

export const vaultProgAddr = new PublicKey(
	"7EKqBVYSCmJbt2T8tGSmwzNKnpL29RqcJcyUr9aEEr6e",
);
export const systemProgram = new PublicKey("11111111111111111111111111111111");
export const ll = console.log;
ll("vaultProgAddr:", vaultProgAddr.toBase58());

export const bigintToBytes = (bint: bigint) => {
	const q: string = bint.toString();
	const bytes = [];
	for (let i = 0; i < q.length; i += 2) {
		let byte = parseInt(q.substring(i, i + 2), 16);
		if (byte > 127) {
			byte = -(~byte & 0xff) - 1;
		}
		bytes.push(byte);
	}
	return bytes;
};
export const findPda1 = (
	userAddr: PublicKey,
	pdaName: string,
	programId = vaultProgAddr,
) => {
	const [configPbk, _configBump] = PublicKey.findProgramAddressSync(
		[Buffer.from("vault"), userAddr.toBuffer()],
		programId,
	);
	ll(pdaName, ":", configPbk.toBase58());
	return configPbk;
};

export type ConfigT = {
	owner: PublicKey;
	deadline: number;
	deposit: bigint;
};
export const getConfigAcct = (
	programId: PublicKey,
	pdaName: string,
): PublicKey => {
	const [configPbk, _configBump] = PublicKey.findProgramAddressSync(
		[Buffer.from("proj_config")],
		programId,
	);
	ll(pdaName, ":", configPbk.toBase58());
	return configPbk;
};

export const makeAccount = (
	payer: Keypair,
	dataAccount: Keypair,
	programId: PublicKey,
	svm: LiteSVM,
) => {
	const ixs = [
		SystemProgram.createAccount({
			fromPubkey: payer.publicKey,
			newAccountPubkey: dataAccount.publicKey,
			lamports: Number(svm.minimumBalanceForRentExemption(BigInt(4))),
			space: 4,
			programId: programId,
		}),
	];
	const tx = new Transaction();
	const blockhash = svm.latestBlockhash();
	tx.recentBlockhash = blockhash;
	tx.add(...ixs);
	tx.sign(payer);
	svm.sendTransaction(tx);
};
//note-litesvm/tests/utils.ts...
export function getLamports(svm: LiteSVM, address: PublicKey): number | null {
	const acc = svm.getAccount(address);
	return acc === null ? null : acc.lamports;
}
export function vaultProgram(computeMaxUnits?: bigint): [LiteSVM, PublicKey] {
	const programId = PublicKey.unique();
	let svm = new LiteSVM();

	if (computeMaxUnits) {
		const computeBudget = new ComputeBudget();
		computeBudget.computeUnitLimit = computeMaxUnits;
		svm = svm.withComputeBudget(computeBudget);
	}
	svm.addProgramFromFile(programId, programPath);
	return [svm, programId];
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
