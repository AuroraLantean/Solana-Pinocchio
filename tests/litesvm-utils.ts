import {
	ACCOUNT_SIZE,
	AccountLayout,
	TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import type { Keypair } from "@solana/web3.js";
import {
	LAMPORTS_PER_SOL,
	PublicKey,
	SystemProgram,
	Transaction,
} from "@solana/web3.js";
import { ComputeBudget, LiteSVM } from "litesvm";

export const vaultProgAddr = new PublicKey(
	"7EKqBVYSCmJbt2T8tGSmwzNKnpL29RqcJcyUr9aEEr6e",
);
export const systemProgram = new PublicKey("11111111111111111111111111111111");
export const usdcMint = new PublicKey(
	"EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
);
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
export const findVaultPda = (userAddr: PublicKey, pdaName: string) => {
	const [configPbk, _configBump] = PublicKey.findProgramAddressSync(
		[Buffer.from("vault"), userAddr.toBuffer()],
		vaultProgAddr,
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
export const makeUsdcMint = (
	owner: PublicKey,
	ata: PublicKey,
	usdcToOwn: bigint,
) => {
	const tokenAccData = Buffer.alloc(ACCOUNT_SIZE);
	AccountLayout.encode(
		{
			mint: usdcMint,
			owner,
			amount: usdcToOwn,
			delegateOption: 0,
			delegate: PublicKey.default,
			delegatedAmount: 0n,
			state: 1,
			isNativeOption: 0,
			isNative: 0n,
			closeAuthorityOption: 0,
			closeAuthority: PublicKey.default,
		},
		tokenAccData,
	);
	const svm = new LiteSVM();
	svm.setAccount(ata, {
		lamports: 1_000_000_000,
		data: tokenAccData,
		owner: TOKEN_PROGRAM_ID,
		executable: false,
	});
	const rawAccount = svm.getAccount(ata);
	return rawAccount;
};
//note-litesvm/tests/utils.ts...
export function getLamports(svm: LiteSVM, address: PublicKey): number | null {
	const acc = svm.getAccount(address);
	return acc === null ? null : acc.lamports;
}
export function vaultProgram(computeMaxUnits?: bigint): [LiteSVM, PublicKey] {
	const programId = vaultProgAddr; // PublicKey.unique();
	let svm = new LiteSVM();

	if (computeMaxUnits) {
		const computeBudget = new ComputeBudget();
		computeBudget.computeUnitLimit = computeMaxUnits;
		svm = svm.withComputeBudget(computeBudget);
	}
	const programPath = "target/deploy/pinocchio_vault.so";
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
	const programPath = "program_bytes/counter.so";
	svm.addProgramFromFile(programId, programPath);
	return [svm, programId, greetedPubkey];
}
