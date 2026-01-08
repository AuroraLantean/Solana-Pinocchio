import { expect } from "bun:test";
import {
	ACCOUNT_SIZE,
	AccountLayout,
	ASSOCIATED_TOKEN_PROGRAM_ID,
	getAssociatedTokenAddressSync,
	MINT_SIZE,
	MintLayout,
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
	ATokenGPvbd,
	admin,
	dragonCoinAuthority,
	hacker,
	owner,
	systemProgram,
	usdtMint,
	user1,
	user2,
	user3,
	vaultProgAddr,
} from "./web3jsSetup";

const ll = console.log;
ll("\n------== litesvm-utils");
export let svm = new LiteSVM();
export const initBalc = BigInt(LAMPORTS_PER_SOL) * BigInt(10);
svm.airdrop(owner, initBalc);
svm.airdrop(admin, initBalc);
svm.airdrop(user1, initBalc);
svm.airdrop(user2, initBalc);
svm.airdrop(user3, initBalc);
svm.airdrop(hacker, initBalc);
svm.airdrop(dragonCoinAuthority, initBalc);

export function getRawAccount(address: PublicKey) {
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
export const configPdaBump = findPdaV1(owner, "config", "ConfigPDA");
export const vaultPdaBump = findPdaV1(owner, "vault", "VaultO ");
export const vaultPdaBump1 = findPdaV1(user1, "vault", "Vault1");
export const vaultPdaBump2 = findPdaV1(user2, "vault", "Vault2");
export const vaultPdaBump3 = findPdaV1(user3, "vault", "Vault3");
export const configPDA = configPdaBump.pda;
export const vaultO = vaultPdaBump.pda;
export const vault1 = vaultPdaBump1.pda;
export const vault2 = vaultPdaBump2.pda;
export const vault3 = vaultPdaBump3.pda;

//Or just send some SOL
export const makeAccount = (
	payer: Keypair,
	newAccount: PublicKey,
	programId: PublicKey,
) => {
	const ixs = [
		SystemProgram.createAccount({
			fromPubkey: payer.publicKey,
			newAccountPubkey: newAccount,
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

//-------------== LiteSVM Methods
export const sendSol = (addrTo: PublicKey, amount: bigint, signer: Keypair) => {
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
//-------------== Program Methods
export const depositSol = (
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
export const lgcInitMint = (
	signer: Keypair,
	mintKp: Keypair,
	mintAuthority: PublicKey,
	freezeAuthorityOpt: PublicKey,
	decimals: number,
	//argData: Uint8Array<ArrayBufferLike>,
	tokenProg = TOKEN_PROGRAM_ID,
) => {
	const disc = 2;
	const blockhash = svm.latestBlockhash();
	const ix = new TransactionInstruction({
		keys: [
			{ pubkey: signer.publicKey, isSigner: true, isWritable: true },
			{ pubkey: mintKp.publicKey, isSigner: true, isWritable: true },
			{ pubkey: mintAuthority, isSigner: false, isWritable: false },
			{ pubkey: tokenProg, isSigner: false, isWritable: false },
			{ pubkey: freezeAuthorityOpt, isSigner: false, isWritable: false },
			{ pubkey: systemProgram, isSigner: false, isWritable: false },
		],
		programId: vaultProgAddr,
		data: Buffer.from([disc, decimals]),
	});
	const tx = new Transaction();
	tx.recentBlockhash = blockhash;
	tx.add(ix); //tx.add(...ixs);
	tx.sign(signer, mintKp); //first signature is considered "primary" and is used identify and confirm transactions.
	const simRes = svm.simulateTransaction(tx);
	const sendRes = svm.sendTransaction(tx);
	checkSuccess(simRes, sendRes, vaultProgAddr);
};
export const lgcInitAta = (
	signer: Keypair,
	toWallet: PublicKey,
	mint: PublicKey,
	tokenAcct: PublicKey,
	tokenProg = TOKEN_PROGRAM_ID,
	atokenProg = ATokenGPvbd,
) => {
	const disc = 3;
	const blockhash = svm.latestBlockhash();
	const ix = new TransactionInstruction({
		keys: [
			{ pubkey: signer.publicKey, isSigner: true, isWritable: true },
			{ pubkey: toWallet, isSigner: false, isWritable: false },
			{ pubkey: mint, isSigner: false, isWritable: false },
			{ pubkey: tokenAcct, isSigner: false, isWritable: true },
			{ pubkey: tokenProg, isSigner: false, isWritable: false },
			{ pubkey: systemProgram, isSigner: false, isWritable: false },
			{ pubkey: atokenProg, isSigner: false, isWritable: false },
		],
		programId: vaultProgAddr,
		data: Buffer.from([disc]),
	});
	const tx = new Transaction();
	tx.recentBlockhash = blockhash;
	tx.add(ix); //tx.add(...ixs);
	tx.sign(signer);
	const simRes = svm.simulateTransaction(tx);
	const sendRes = svm.sendTransaction(tx);
	checkSuccess(simRes, sendRes, vaultProgAddr);
};

//-------------==
//When you want to make Mint without the Mint Keypair. E.g. UsdtMintKp;
//https://solana.com/docs/tokens/basics/create-mint
export const setMint = (
	mint: PublicKey,
	decimals = 6,
	supply = 9_000_000_000_000n,
	mintAuthority = owner,
	freezeAuthority = owner,
	programId = TOKEN_PROGRAM_ID,
) => {
	const rawMintAcctData = Buffer.alloc(MINT_SIZE);
	MintLayout.encode(
		{
			mintAuthorityOption: 1, //0,
			mintAuthority: mintAuthority, // PublicKey.default,
			supply: supply, // 0n
			decimals: decimals, //0
			isInitialized: true, //false,
			freezeAuthorityOption: 1, //0,
			freezeAuthority: freezeAuthority, // PublicKey.default,
		},
		rawMintAcctData,
	);
	svm.setAccount(mint, {
		lamports: 1_000_000_000,
		data: rawMintAcctData,
		owner: programId,
		executable: false,
	});
};
//-------------== USDC or USDT
export const acctIsNull = (account: PublicKey) => {
	const raw = svm.getAccount(account);
	expect(raw).toBeNull();
};
export const acctExists = (account: PublicKey) => {
	const raw = svm.getAccount(account);
	expect(raw).not.toBeNull();
};
export const getAta = (
	mint: PublicKey,
	owner: PublicKey,
	allowOwnerOffCurve = true,
	programId = TOKEN_PROGRAM_ID,
	associatedTokenProgramId = ASSOCIATED_TOKEN_PROGRAM_ID,
) => {
	const ata = getAssociatedTokenAddressSync(
		mint,
		owner,
		allowOwnerOffCurve,
		programId,
		associatedTokenProgramId,
	);
	return ata;
};
export const vaultAtaO = getAta(usdtMint, vaultO);
export const vaultAta1 = getAta(usdtMint, vault1);
export const vaultAta2 = getAta(usdtMint, vault2);
export const vaultAta3 = getAta(usdtMint, vault3);

export const setAta = (
	mint: PublicKey,
	owner: PublicKey,
	tokenAmount: bigint,
	allowOwnerOffCurve = true,
	programId = TOKEN_PROGRAM_ID,
	associatedTokenProgramId = ASSOCIATED_TOKEN_PROGRAM_ID,
) => {
	const ata = getAssociatedTokenAddressSync(
		mint,
		owner,
		allowOwnerOffCurve,
		programId,
		associatedTokenProgramId,
	);

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
	const rawTokenAcctData = Buffer.alloc(ACCOUNT_SIZE);
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
		rawTokenAcctData,
	);
	svm.setAccount(ata, {
		lamports: 1_000_000_000,
		data: rawTokenAcctData,
		owner: programId,
		executable: false,
	});
	const raw = svm.getAccount(ata);
	return { raw, ata };
};
export const tokBalc = (
	mint: PublicKey,
	owner: PublicKey,
	programId = TOKEN_PROGRAM_ID,
	associatedTokenProgramId = ASSOCIATED_TOKEN_PROGRAM_ID,
) => {
	const ata = getAssociatedTokenAddressSync(
		mint,
		owner,
		true, //allowOwnerOffCurve?
		programId,
		associatedTokenProgramId,
	);
	const raw = svm.getAccount(ata);
	if (!raw) throw new Error("Ata is null");
	const rawAcctData = raw?.data;
	const decoded = AccountLayout.decode(rawAcctData);
	return decoded.amount;
};
export const setAtaCheck = (
	mint: PublicKey,
	user: PublicKey,
	amt: bigint,
	user_and_mint: string,
) => {
	const { raw: rawData, ata } = setAta(mint, user, amt);
	ll(user_and_mint, "ata:", ata.toBase58());
	expect(rawData).not.toBeNull();
	const rawAcctData = rawData?.data;
	if (rawAcctData) {
		const decoded = AccountLayout.decode(rawAcctData);
		ll(user_and_mint, "balc:", decoded.amount);
		expect(decoded.amount).toStrictEqual(amt);
	} else {
		ll(user_and_mint, "rawAcctData is undefined");
	}
};
//---------------== Deployment
export const vaultProgram = (computeMaxUnits?: bigint) => {
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
vaultProgram();

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
