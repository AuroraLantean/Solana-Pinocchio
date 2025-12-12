import {
	type Address,
	appendTransactionMessageInstruction,
	assertIsTransactionWithBlockhashLifetime,
	createTransactionMessage,
	getAddressEncoder,
	getProgramDerivedAddress,
	getSignatureFromTransaction,
	getUtf8Encoder,
	pipe,
	sendAndConfirmTransactionFactory,
	setTransactionMessageFeePayer,
	setTransactionMessageLifetimeUsingBlockhash,
	signTransactionMessageWithSigners,
} from "@solana/kit";
import * as vault from "../clients/js/src/generated/index";
import { baseSOL } from "./setup";

export const ll = console.log;

export const vaultProgAddr = vault.PINOCCHIO_VAULT_PROGRAM_ADDRESS;

export const makeSolAmt = (amt: number) => BigInt(amt) * baseSOL;

export const findPda = async (
	userAddr: Address<string>,
	str: string,
	progAddr = vaultProgAddr,
) => {
	const seedSigner = getAddressEncoder().encode(userAddr);
	const seedTag = getUtf8Encoder().encode(str);

	const pda_bump = await getProgramDerivedAddress({
		programAddress: progAddr,
		seeds: [seedTag, seedSigner],
	});
	ll(`${str} pda: ${pda_bump[0]}, bump: ${pda_bump[1]}`);
	return { pda: pda_bump[0], bump: pda_bump[1] };
};

//https://www.solanakit.com/docs/getting-started/send-transaction#confirmation-strategies
export const sendTxn = async (
	methodIx: any,
	signerKp: any,
	rpc: any,
	rpcSubscriptions: any,
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
