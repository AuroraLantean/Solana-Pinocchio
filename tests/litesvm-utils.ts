import { expect } from "bun:test";
import {
	ACCOUNT_SIZE,
	AccountLayout,
	ASSOCIATED_TOKEN_PROGRAM_ID,
	getAssociatedTokenAddressSync,
	TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import {
	type Keypair,
	LAMPORTS_PER_SOL,
	PublicKey,
	SystemProgram,
	Transaction,
	TransactionInstruction,
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
	systemProgram,
	user1Addr,
	user2Addr,
	user3Addr,
	vaultProgAddr,
} from "./web3jsSetup";

const ll = console.log;
ll("\n------== litesvm-utils");
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
export const configPdaBump = findPdaV1(ownerAddr, "config", "ConfigPDA");
export const vaultPdaBump = findPdaV1(ownerAddr, "vault", "VaultPDA ");
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

//-------------== Program Methods
export const sendSol = (
	svm: LiteSVM,
	addrTo: PublicKey,
	amount: bigint,
	signer: Keypair,
) => {
	const blockhash = svm.latestBlockhash();
	const ixs = [
		SystemProgram.transfer({
			fromPubkey: signer.publicKey,
			toPubkey: addrTo,
			lamports: amount,
		}),
	];
	const tx = new Transaction();
	tx.recentBlockhash = blockhash;
	tx.add(...ixs);
	tx.sign(signer);
	svm.sendTransaction(tx);
};
export const depositSol = (
	svm: LiteSVM,
	vaultPdaX: PublicKey,
	argData: Uint8Array<ArrayBufferLike>,
	signer: Keypair,
) => {
	const disc = 0;
	const blockhash = svm.latestBlockhash();
	const ix = new TransactionInstruction({
		keys: [
			{ pubkey: signer.publicKey, isSigner: true, isWritable: true },
			{ pubkey: vaultPdaX, isSigner: false, isWritable: true },
			{ pubkey: systemProgram, isSigner: false, isWritable: false },
		],
		programId: vaultProgAddr,
		data: Buffer.from([disc, ...argData]),
	});
	const tx = new Transaction();
	tx.recentBlockhash = blockhash;
	tx.add(ix); //tx.add(...ixs);
	tx.sign(signer);
	const simRes = svm.simulateTransaction(tx);
	const sendRes = svm.sendTransaction(tx);
	checkSuccess(simRes, sendRes, vaultProgAddr);
};
export const withdrawSol = (
	svm: LiteSVM,
	vaultPdaX: PublicKey,
	argData: Uint8Array<ArrayBufferLike>,
	signer: Keypair,
) => {
	const disc = 1;
	const blockhash = svm.latestBlockhash();
	const ix = new TransactionInstruction({
		keys: [
			{ pubkey: signer.publicKey, isSigner: true, isWritable: true },
			{ pubkey: vaultPdaX, isSigner: false, isWritable: true },
		],
		programId: vaultProgAddr,
		data: Buffer.from([disc, ...argData]),
	});
	const tx = new Transaction();
	tx.recentBlockhash = blockhash;
	tx.add(ix); //tx.add(...ixs);
	tx.sign(signer);
	const simRes = svm.simulateTransaction(tx);
	const sendRes = svm.sendTransaction(tx);
	checkSuccess(simRes, sendRes, vaultProgAddr);
};
//-------------== USDC or USDT
export const newMint = (
	svm: LiteSVM,
	mint: PublicKey,
	owner: PublicKey,
	tokenAmount: bigint,
	programId = TOKEN_PROGRAM_ID,
	associatedTokenProgramId = ASSOCIATED_TOKEN_PROGRAM_ID,
) => {
	const ata = getAssociatedTokenAddressSync(
		mint,
		adminAddr,
		true,
		programId,
		associatedTokenProgramId,
	); //allowOwnerOffCurve?

	/* Set account via knowing its layout
  export interface RawAccount {
    mint: PublicKey;
    owner: PublicKey;
    amount: bigint;
    delegateOption: 1 | 0;
    delegate: PublicKey;
    state: AccountState;
    isNativeOption: 1 | 0;
    isNative: bigint;
    delegatedAmount: bigint;
    closeAuthorityOption: 1 | 0;
    closeAuthority: PublicKey;
}

// Buffer layout for de/serializing a token account
export const AccountLayout = struct<RawAccount>([
    publicKey('mint'),
    publicKey('owner'),
    u64('amount'),
    u32('delegateOption'),
    publicKey('delegate'),
    u8('state'),
    u32('isNativeOption'),
    u64('isNative'),
    u64('delegatedAmount'),
    u32('closeAuthorityOption'),
    publicKey('closeAuthority'),
]);

// Byte length of a token account 
export const ACCOUNT_SIZE = AccountLayout.span; */
	const tokenAccData = Buffer.alloc(ACCOUNT_SIZE);
	AccountLayout.encode(
		{
			mint,
			owner,
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
		owner: programId,
		executable: false,
	});
	const rawAccount = svm.getAccount(ata);
	return { rawAccount, ata };
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
		ll("sendRes.err():", sendRes.err());
		ll("sendRes.meta():", sendRes.meta());
		const errStr = sendRes.toString();
		ll("sendRes.toString():", errStr);
		const pos = errStr.search("custom program error: 0x");
		ll("pos:", pos);
		if (pos > -1) {
			let errCode = errStr.substring(pos + 22, pos + 26);
			if (errCode.slice(-1) === '"') {
				//ll("last char:", errCode.slice(-1));
				errCode = errCode.slice(0, -1);
			}
			ll("error code:", errCode, Number(errCode));
		}
		ll(
			"find error here: https://docs.rs/solana-sdk/latest/solana_sdk/transaction/enum.TransactionError.html",
		);
		throw new Error("Unexpected tx failure");
	}
};
