import { expect } from "bun:test";

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
import type {
	FailedTransactionMetadata,
	SimulatedTransactionInfo,
} from "litesvm";
import { ComputeBudget, LiteSVM, TransactionMetadata } from "litesvm";
import {
	adminAddr,
	hackerAddr,
	ownerAddr,
	user1Addr,
	user2Addr,
	user3Addr,
	vaultProgAddr,
} from "./web3jsSetup";

const ll = console.log;
export const svm = new LiteSVM();
export const initBalc = BigInt(LAMPORTS_PER_SOL) * BigInt(10);
svm.airdrop(ownerAddr, initBalc);
svm.airdrop(adminAddr, initBalc);
svm.airdrop(user1Addr, initBalc);
svm.airdrop(user2Addr, initBalc);
svm.airdrop(user3Addr, initBalc);
svm.airdrop(hackerAddr, initBalc);

export function getRawAccount(svm: LiteSVM, address: PublicKey) {
	const rawAccount = svm.getAccount(address);
	return rawAccount;
}

export const findPdaV1 = (
	userAddr: PublicKey,
	seedStr: string,
	pdaName: string,
	progAddr = vaultProgAddr,
) => {
	const [pda, bump] = PublicKey.findProgramAddressSync(
		[Buffer.from(seedStr), userAddr.toBuffer()],
		progAddr,
	);
	ll(`${pdaName} pda: ${pda.toBase58()}, bump: ${bump}`);
	return { pda, bump };
};
export const configPdaBump = findPdaV1(adminAddr, "config", "ConfigPDA");
export const vaultPdaBump = findPdaV1(ownerAddr, "vault", "VaultPDA");
export const vaultPdaBump1 = findPdaV1(user1Addr, "vault", "VaultPDA1");
export const vaultPdaBump2 = findPdaV1(user2Addr, "vault", "VaultPDA2");
export const vaultPdaBump3 = findPdaV1(user3Addr, "vault", "VaultPDA3");
export const configPDA = configPdaBump.pda;
export const vaultPDA = vaultPdaBump.pda;
export const vaultPDA1 = vaultPdaBump1.pda;
export const vaultPDA2 = vaultPdaBump2.pda;
export const vaultPDA3 = vaultPdaBump3.pda;

export const makeAccount = (
	svm: LiteSVM,
	payer: Keypair,
	acctKp: Keypair,
	programId: PublicKey,
) => {
	const ixs = [
		SystemProgram.createAccount({
			fromPubkey: payer.publicKey,
			newAccountPubkey: acctKp.publicKey,
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

//-------------==

//-------------== USDC or USDT
export const makeMint = (
	svm: LiteSVM,
	mint: PublicKey,
	owner: PublicKey,
	ata: PublicKey,
	tokenAmount: bigint,
) => {
	const tokenAccData = Buffer.alloc(ACCOUNT_SIZE);
	AccountLayout.encode(
		{
			mint,
			owner,
			//decimal: 6,
			amount: tokenAmount,
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
	svm.setAccount(ata, {
		lamports: 1_000_000_000,
		data: tokenAccData,
		owner: TOKEN_PROGRAM_ID,
		executable: false,
	});
	const rawAccount = svm.getAccount(ata);
	return rawAccount;
};

//---------------== Deployment
export const vaultProgram = (svm: LiteSVM, computeMaxUnits?: bigint) => {
	ll("load VaultProgram...");
	if (computeMaxUnits) {
		const computeBudget = new ComputeBudget();
		computeBudget.computeUnitLimit = computeMaxUnits;
		svm = svm.withComputeBudget(computeBudget);
	}
	const programPath = "target/deploy/pinocchio_vault.so";
	svm.addProgramFromFile(vaultProgAddr, programPath);
	//return [programId];
};
vaultProgram(svm);

export function helloworldProgram(
	svm: LiteSVM,
	computeMaxUnits?: bigint,
): [PublicKey, PublicKey] {
	const programId = PublicKey.unique();
	const greetedPubkey = PublicKey.unique();
	//let svm = new LiteSVM();

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
	return [programId, greetedPubkey];
}

export const checkSuccess = (
	simRes: FailedTransactionMetadata | SimulatedTransactionInfo,
	sendRes: TransactionMetadata | FailedTransactionMetadata,
	programId: PublicKey,
	isVerbose = false,
) => {
	ll("\nsimRes meta prettylogs:", simRes.meta().prettyLogs());
	if (isVerbose) {
		ll("\nsimRes.meta().logs():", simRes.meta().logs());
	}
	/** simRes.meta():
      computeUnitsConsumed: [class computeUnitsConsumed],
      innerInstructions: [class innerInstructions],
      logs: [class logs],
      prettyLogs: [class prettyLogs],
      returnData: [class returnData],
      signature: [class signature],
      toString: [class toString], */
	if (sendRes instanceof TransactionMetadata) {
		expect(simRes.meta().logs()).toStrictEqual(sendRes.logs());

		const logLength = simRes.meta().logs().length;
		//ll("logLength:", logLength);
		//ll("sendRes.logs()[logIndex]:", sendRes.logs()[logIndex]);
		expect(sendRes.logs()[logLength - 1]).toStrictEqual(
			`Program ${programId} success`,
		);
	} else {
		ll("sendRes.err:", sendRes.err());
		ll("sendRes.meta:", sendRes.meta());
		ll("sendRes.toString:", sendRes.toString());
		ll(
			"find error here: https://docs.rs/solana-sdk/latest/solana_sdk/transaction/enum.TransactionError.html",
		);
		throw new Error("Unexpected tx failure");
	}
};
