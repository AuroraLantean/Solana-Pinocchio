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
import * as vault from "../clients/js/src/generated/index";

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
//import secret from './my-keypair.json';
//const user2 = await createKeyPairSignerFromBytes(new Uint8Array(secret));
export const adminAddr = adminKp.address;
export const mint = mintKp.address;
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
	recipientAddress: hackerKp.address,
});
ll(`✅ - Airdropped SOL to Admin and user1Addr`);

const ACCOUNT_DISCRIMINATOR_SIZE = 8; // same as Anchor/Rust
const U64_SIZE = 8; // u64 is 8 bytes
const VAULT_SIZE = ACCOUNT_DISCRIMINATOR_SIZE + U64_SIZE; // 16

// get vault rent
export const vaultRent = await rpc
	.getMinimumBalanceForRentExemption(BigInt(VAULT_SIZE))
	.send();

const { value } = await rpc
	.getAccountInfo(vaultProgAddr, { encoding: "base64" })
	.send();
if (!value || !value?.data) {
	throw new Error(`Program does not exist: ${vaultProgAddr.toString()}`);
}
ll("✅ - Program exits!");

export const getSol = async (account: Address, name: string) => {
	const { value: balc } = await rpc.getBalance(account).send();
	ll(name, "balc:", balc);
	return balc;
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
