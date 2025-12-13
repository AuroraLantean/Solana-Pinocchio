import type {
	Address,
	Blockhash,
	KeyPairSigner,
	TransactionSigner,
} from "@solana/kit";
import {
	appendTransactionMessageInstructions,
	assertIsTransactionWithBlockhashLifetime,
	createTransactionMessage,
	getSignatureFromTransaction,
	pipe,
	sendAndConfirmTransactionFactory,
	setTransactionMessageFeePayerSigner,
	setTransactionMessageLifetimeUsingBlockhash,
	signTransactionMessageWithSigners,
} from "@solana/kit";
import { getCreateAccountInstruction } from "@solana-program/system";
import {
	findAssociatedTokenPda,
	getCreateAssociatedTokenInstructionAsync,
	getInitializeAccount2Instruction,
	getInitializeMintInstruction,
	getMintSize,
	getTokenSize,
	TOKEN_PROGRAM_ADDRESS,
} from "@solana-program/token";
import { rpc, rpcSubscriptions } from "./httpws";
import { ll } from "./utils";

// https://solana.com/docs/tokens/basics/create-token-account

export const makeMint = async (
	feePayerKp: KeyPairSigner<string>,
	mintKp: KeyPairSigner<string>,
	latestBlockhash: Readonly<{
		blockhash: Blockhash;
		lastValidBlockHeight: bigint;
	}>,
) => {
	// Get default mint account size (in bytes), no extensions enabled
	const space = BigInt(getMintSize());

	// Get minimum balance for rent exemption
	const rent = await rpc.getMinimumBalanceForRentExemption(space).send();

	// Instruction to create new account for mint (token program)
	// Invokes the system program
	const createAccountInstruction = getCreateAccountInstruction({
		payer: feePayerKp,
		newAccount: mintKp,
		lamports: rent,
		space,
		programAddress: TOKEN_PROGRAM_ADDRESS,
	});

	// Instruction to initialize mint account data
	// Invokes the token program
	const initializeMintInstruction = getInitializeMintInstruction({
		mint: mintKp.address,
		decimals: 9,
		mintAuthority: feePayerKp.address,
	});

	const instructions = [createAccountInstruction, initializeMintInstruction];

	// Create transaction message
	const transactionMessage = pipe(
		createTransactionMessage({ version: 0 }), // Create transaction message
		(tx) => setTransactionMessageFeePayerSigner(feePayerKp, tx), // Set fee payer
		(tx) => setTransactionMessageLifetimeUsingBlockhash(latestBlockhash, tx), // Set transaction blockhash
		(tx) => appendTransactionMessageInstructions(instructions, tx), // Append instructions
	);

	// Sign transaction message with required signers (fee payer and mint keypair)
	const signedTransaction =
		await signTransactionMessageWithSigners(transactionMessage);
	assertIsTransactionWithBlockhashLifetime(signedTransaction);
	// Send and confirm transaction
	await sendAndConfirmTransactionFactory({ rpc, rpcSubscriptions })(
		signedTransaction,
		{ commitment: "confirmed" },
	);

	// Get transaction signature
	const transactionSignature = getSignatureFromTransaction(signedTransaction);

	ll("Mint Address:", mintKp.address);
	ll("Transaction Signature:", transactionSignature);
};

export const makeTokenAccount = async (
	tokenAccountKp: KeyPairSigner<string>,
	feePayerKp: TransactionSigner<string>,
	mint: Address,
	latestBlockhash: Readonly<{
		blockhash: Blockhash;
		lastValidBlockHeight: bigint;
	}>,
) => {
	// Get token account size (in bytes)
	const tokenAccountSpace = BigInt(getTokenSize());

	// Get minimum balance for rent exemption
	const tokenAccountRent = await rpc
		.getMinimumBalanceForRentExemption(tokenAccountSpace)
		.send();

	// Instruction to create new account for token account (token program)
	// Invokes the system program
	const createTokenAccountInstruction = getCreateAccountInstruction({
		payer: feePayerKp,
		newAccount: tokenAccountKp,
		lamports: tokenAccountRent,
		space: tokenAccountSpace,
		programAddress: TOKEN_PROGRAM_ADDRESS,
	});

	// Instruction to initialize token account data
	// Invokes the token program
	const initializeTokenAccountInstruction = getInitializeAccount2Instruction({
		account: tokenAccountKp.address,
		mint: mint,
		owner: feePayerKp.address,
	});

	const instructions2 = [
		createTokenAccountInstruction,
		initializeTokenAccountInstruction,
	];

	// Create transaction message for token account creation
	const tokenAccountMessage = pipe(
		createTransactionMessage({ version: 0 }), // Create transaction message
		(tx) => setTransactionMessageFeePayerSigner(feePayerKp, tx), // Set fee payer
		(tx) => setTransactionMessageLifetimeUsingBlockhash(latestBlockhash, tx), // Set transaction blockhash
		(tx) => appendTransactionMessageInstructions(instructions2, tx), // Append instructions
	);

	// Sign transaction message with required signers (fee payer and token account keypair)
	const signedTokenAccountTx =
		await signTransactionMessageWithSigners(tokenAccountMessage);
	assertIsTransactionWithBlockhashLifetime(signedTokenAccountTx);
	// Send and confirm transaction
	await sendAndConfirmTransactionFactory({ rpc, rpcSubscriptions })(
		signedTokenAccountTx,
		{ commitment: "confirmed" },
	);

	// Get transaction signature
	const transactionSignature2 =
		getSignatureFromTransaction(signedTokenAccountTx);

	ll("Token Account Address:", tokenAccountKp.address);
	ll("Transaction Signature:", transactionSignature2);
};

export const makeATA = async (
	feePayerKp: KeyPairSigner<string>,
	tokenOwner: Address,
	mint: Address,
) => {
	// Use findAssociatedTokenPda to derive the ATA address
	const [associatedTokenAddress, bump] = await findAssociatedTokenPda({
		mint: mint,
		owner: tokenOwner,
		tokenProgram: TOKEN_PROGRAM_ADDRESS,
	});

	ll("ATA:", associatedTokenAddress.toString());

	// Get a fresh blockhash for the second transaction
	const { value: latestBlockhash2 } = await rpc.getLatestBlockhash().send();

	// Create instruction to create the associated token account
	const createAtaInstruction = await getCreateAssociatedTokenInstructionAsync({
		payer: feePayerKp,
		mint: mint,
		owner: tokenOwner,
	});

	// Create transaction message
	const transactionMessage2 = pipe(
		createTransactionMessage({ version: 0 }),
		(tx) => setTransactionMessageFeePayerSigner(feePayerKp, tx),
		(tx) => setTransactionMessageLifetimeUsingBlockhash(latestBlockhash2, tx),
		(tx) => appendTransactionMessageInstructions([createAtaInstruction], tx),
	);

	// Sign transaction message with all required signers
	const signedTransaction2 =
		await signTransactionMessageWithSigners(transactionMessage2);
	assertIsTransactionWithBlockhashLifetime(signedTransaction2);
	// Send and confirm transaction
	await sendAndConfirmTransactionFactory({ rpc, rpcSubscriptions })(
		signedTransaction2,
		{ commitment: "confirmed" },
	);

	// Get transaction signature
	const transactionSignature2 = getSignatureFromTransaction(signedTransaction2);
	ll("Transaction Signature:", transactionSignature2);
	return { ata: associatedTokenAddress, bump };
};
