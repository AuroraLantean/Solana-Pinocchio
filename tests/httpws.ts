import {
	type Address,
	airdropFactory,
	appendTransactionMessageInstruction,
	assertIsTransactionWithBlockhashLifetime,
	createSolanaRpc,
	createSolanaRpcSubscriptions,
	createTransactionMessage,
	generateKeyPairSigner,
	getSignatureFromTransaction,
	lamports,
	pipe,
	sendAndConfirmTransactionFactory,
	setTransactionMessageFeePayer,
	setTransactionMessageLifetimeUsingBlockhash,
	signTransactionMessageWithSigners,
} from "@solana/kit";
import { LAMPORTS_PER_SOL } from "gill";
import * as vault from "../clients/js/src/generated/index";
import { getAta } from "./tokens";
import type { Data1 } from "./types";
import { findPda, llbalc } from "./utils";

export const vaultProgAddr = vault.PINOCCHIO_VAULT_PROGRAM_ADDRESS;

const httpProvider = "http://127.0.0.1:8899";
const wssProvider = "ws://127.0.0.1:8900";

export const rpc = createSolanaRpc(httpProvider);
export const rpcSubscriptions = createSolanaRpcSubscriptions(wssProvider);
console.log(`✅ - Established connection to ${httpProvider}`);

//https://www.solanakit.com/docs/getting-started/signers
export const adminKp = await generateKeyPairSigner();
export const mintAuthorityKp = await generateKeyPairSigner();
export const user1Kp = await generateKeyPairSigner();
export const user2Kp = await generateKeyPairSigner();
export const user3Kp = await generateKeyPairSigner();
export const hackerKp = await generateKeyPairSigner();
export const mintKp = await generateKeyPairSigner();
export const mint22Kp = await generateKeyPairSigner();
//import secret from './my-keypair.json';
//const user2 = await createKeyPairSignerFromBytes(new Uint8Array(secret));
export const adminAddr = adminKp.address;
export const mint = mintKp.address;
export const mint22 = mint22Kp.address;
export const mintAuthority = mintAuthorityKp.address;
export const user1Addr = user1Kp.address;
export const user2Addr = user2Kp.address;
export const user3Addr = user3Kp.address;
export const hackerAddr = hackerKp.address;

const ll = console.log;
ll(`✅ mint: ${mint}`);
ll(`✅ mintAuthority: ${mintAuthority}`);
ll(`✅ adminAddr ${adminAddr}`);
ll(`✅ user1Addr: ${user1Addr}`);
ll(`✅ user2Addr: ${user2Addr}`);
ll(`✅ user3Addr: ${user3Addr}`);
ll(`✅ hackerAddr: ${hackerAddr}`);

export const decimalsSOL = BigInt(9);
export const baseSOL = BigInt(10) ** decimalsSOL;
export const amtAirdrop = BigInt(100) * baseSOL;

// Airdrop SOL to admin
const airdrop = airdropFactory({ rpc, rpcSubscriptions });
await airdrop({
	commitment: "confirmed",
	lamports: lamports(amtAirdrop),
	recipientAddress: adminAddr,
});
await airdrop({
	commitment: "confirmed",
	lamports: lamports(amtAirdrop),
	recipientAddress: mintAuthorityKp.address,
});
await airdrop({
	commitment: "confirmed",
	lamports: lamports(amtAirdrop),
	recipientAddress: user1Addr,
});
await airdrop({
	commitment: "confirmed",
	lamports: lamports(amtAirdrop),
	recipientAddress: user2Addr,
});
await airdrop({
	commitment: "confirmed",
	lamports: lamports(amtAirdrop),
	recipientAddress: user3Addr,
});
await airdrop({
	commitment: "confirmed",
	lamports: lamports(amtAirdrop),
	recipientAddress: hackerKp.address,
});
ll(`✅ - Airdropped SOL to Admin and user1Addr`);

const ACCOUNT_DISCRIMINATOR_SIZE = 8; // same as Anchor/Rust
const U64_SIZE = 8; // u64 is 8 bytes
const VAULT_SIZE = ACCOUNT_DISCRIMINATOR_SIZE + U64_SIZE; // 16

const pda_bump = await findPda(adminAddr, "vault");
export const vaultPDA = pda_bump.pda;
ll(`✅ - Vault PDA: ${vaultPDA}`);
const pda_bump1 = await findPda(user1Addr, "vault");
export const vaultPDA1 = pda_bump1.pda;
ll(`✅ - vaultPDA1: ${vaultPDA1}`);

const configPdaBump = await findPda(user1Addr, "vault");
export const configPDA = configPdaBump.pda;
ll(`✅ - configPDA: ${configPDA}`);

const vaultAtabump1 = await getAta(mint, vaultPDA1);
export const vaultAta1 = vaultAtabump1.ata;

// get vault rent
export const vaultRent = await rpc
	.getMinimumBalanceForRentExemption(BigInt(VAULT_SIZE))
	.send();

export const checkAcct = async (target: Address, name: string) => {
	const { value } = await rpc
		.getAccountInfo(target, { encoding: "base64" })
		.send();
	if (!value || !value?.data) {
		ll(`${name} does not exist`);
		return false;
	}
	ll(`${name} exits!`);
	return true;
};

export const getSol = async (account: Address, name: string) => {
	const { value: lamports } = await rpc.getBalance(account).send();
	ll(name, "balc:", lamports);
	return {
		lamports,
		balcUi: BigInt(lamports.toString()) / BigInt(LAMPORTS_PER_SOL),
	};
};
export const getTokBalc = async (ata: Address, name: string = "") => {
	const {
		value: { amount, decimals },
	} = await rpc.getTokenAccountBalance(ata, { commitment: "confirmed" }).send();
	const amountUi = (BigInt(amount) / BigInt(10 ** decimals)).toString();
	llbalc(name, amountUi);
	return {
		amount,
		decimals,
		amountUi,
	};
};
export const getTokBalc2 = async (
	owner: Address,
	tokenProgram: Address,
	tokenIndex = 0,
	name: string = "",
) => {
	const tokenAccounts = await rpc
		.getTokenAccountsByOwner(
			owner,
			{ programId: tokenProgram },
			{ encoding: "base64" },
		)
		.send();
	//const tokenAccounts = GetTokenAccountsByOwner
	ll("tokenAccounts:", tokenAccounts);
	const valuesLen = tokenAccounts.value.length;
	if (valuesLen === 0) {
		ll("no token is found");
		return "0";
	}
	if (tokenIndex >= valuesLen) {
		return "tokenIndex invalid";
	}
	const value0 = tokenAccounts.value[tokenIndex];
	const amt = value0?.account.data as unknown as Data1;
	ll(name, "balc:", amt);
	return amt;
};

//https://www.solanakit.com/docs/getting-started/send-transaction#confirmation-strategies
export const sendTxn = async (
	methodIx: any,
	signerKp: any,
	isVerbose = false,
) => {
	ll("sendTxn() ...");
	const { value: latestBlockhash } = await rpc.getLatestBlockhash().send();
	if (isVerbose) ll("latestBlockhash:", latestBlockhash);
	const txnMesg = pipe(
		createTransactionMessage({ version: 0 }),
		(tx) => setTransactionMessageFeePayer(signerKp.address, tx),
		(tx) => setTransactionMessageLifetimeUsingBlockhash(latestBlockhash, tx),
		(tx) => appendTransactionMessageInstruction(methodIx, tx),
		//(tx) => addSignersToTransactionMessage([signerKp], tx), //do we need this?
	);
	// Sign and send transaction
	const signedTransaction = await signTransactionMessageWithSigners(txnMesg);
	assertIsTransactionWithBlockhashLifetime(signedTransaction);

	const sendAndConfirmTransaction = sendAndConfirmTransactionFactory({
		rpc,
		rpcSubscriptions,
	});

	//lastValidBlockHeight

	await sendAndConfirmTransaction(signedTransaction, {
		commitment: "confirmed",
	}); //"confirmRecentTransaction" | "rpc" | "transaction"

	const signature = getSignatureFromTransaction(signedTransaction);
	ll("Transaction signature:", signature);
};
