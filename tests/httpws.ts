import {
	type Address,
	airdropFactory,
	appendTransactionMessageInstruction,
	assertAccountExists,
	assertIsTransactionWithBlockhashLifetime,
	createSolanaRpc,
	createSolanaRpcSubscriptions,
	createTransactionMessage,
	decodeAccount,
	fetchEncodedAccount,
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
import { configAcctDecoder, type DecodedConfigAcct } from "./decoder";
import { getAta } from "./tokens";
import { findPdaV2, LAMPORTS_PER_SOL, llbalc } from "./utils";

export const vaultProgAddr = vault.PINOCCHIO_VAULT_PROGRAM_ADDRESS;

const httpProvider = "http://127.0.0.1:8899";
const wssProvider = "ws://127.0.0.1:8900";

export const rpc = createSolanaRpc(httpProvider);
export const rpcSubscriptions = createSolanaRpcSubscriptions(wssProvider);
console.log(`✅ - Established connection to ${httpProvider}`);

//https://www.solanakit.com/docs/getting-started/signers
export const ownerKp = await generateKeyPairSigner();
export const adminKp = await generateKeyPairSigner();
//TODO: remove mintAuthority. use dragonCoinAuthority
export const mintAuthorityKp = await generateKeyPairSigner();
export const user1Kp = await generateKeyPairSigner();
export const user2Kp = await generateKeyPairSigner();
export const user3Kp = await generateKeyPairSigner();
export const hackerKp = await generateKeyPairSigner();
export const mintKp = await generateKeyPairSigner();
export const mint22Kp = await generateKeyPairSigner();
//import secret from './my-keypair.json';
//const user2 = await createKeyPairSignerFromBytes(new Uint8Array(secret));
export const ownerAddr = ownerKp.address;
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
	recipientAddress: ownerAddr,
});
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

const pda_bump = await findPdaV2(ownerAddr, "vault", "VaultO");
export const vaultO = pda_bump.pda;
ll(`✅ - Vault PDA: ${vaultO}`);
const pda_bump1 = await findPdaV2(user1Addr, "vault", "Vault1");
export const vault1 = pda_bump1.pda;
ll(`✅ - vault1: ${vault1}`);

const configPdaBump = await findPdaV2(adminAddr, "config", "Config");
export const configPDA = configPdaBump.pda;
ll(`✅ - configPDA: ${configPDA}`);

const vaultAtabump1 = await getAta(mint, vault1);
export const vaultAta1 = vaultAtabump1.ata;

//------------==
// get vault rent
export const vaultRent = await rpc
	.getMinimumBalanceForRentExemption(BigInt(VAULT_SIZE))
	.send();

export const acctExists = async (target: Address, name: string) => {
	const { value } = await rpc
		.getAccountInfo(target, { encoding: "base64" })
		.send();
	if (!value || !value?.data) {
		ll(`${name} does not exist`);
		return null;
	}
	ll(`${name} exits!`);
	return value;
};
export const readConfigData = async (target: Address, name: string) => {
	ll("readConfigData()...");
	const encodedAcct = await fetchEncodedAccount(rpc, target);
	assertAccountExists(encodedAcct);
	//encodedAcct satisfies MaybeEncodedAccount<>;
	ll("account exists");
	const decoded = decodeAccount(encodedAcct, configAcctDecoder);
	//decoded satisfies Account<ConfigAcct, target>;

	/*const _authorityBytes = value.data.slice(8, 8 + 32); const _authorityPubkey = Address(authorityBytes);
	const feeBytes = value.data.slice(40, 48); // bytes 40..48
	const _fee = Number(
		feeBytes.readBigUInt64LE(0), // read as little-endian
	);*/
	ll(name, "decoded:", decoded);
	const acct = decoded as unknown as DecodedConfigAcct;
	return acct.data;
};
/*export const getAccounts = async () => {
	const accounts = await rpc
		.getProgramAccounts(vaultProgAddr, {
			commitment: "finalized",
			encoding: "jsonParsed",
			filters: [
				// {
				// 	dataSize: BigInt(17),
				// },
				{
					memcmp: {
						bytes: "" as Base58EncodedBytes, //bs58.encode(Buffer.from(OFFER_DISCRIMINATOR))//"Base58EncodedBytes", // | Base64EncodedBytes'
						offset: 0n, //BigInt(4),
						//encoding: "base58" or "base64"
					},
				},
			],
		})
		.send();
	ll("accounts:", accounts);
};*/

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
//------------== Account Types
export type Data1 = {
	program: string;
	parsed: {
		info: {
			isNative: boolean;
			mint: string;
			owner: string;
			state: string; // "initialized";
			tokenAmount: {
				amount: string;
				decimals: number;
				uiAmount: number;
				uiAmountString: string;
			};
		};
		type: string; // "account"
	};
	space: number;
};
export type GetTokenAccountsByOwnerValue = {
	pubkey: string;
	account: {
		data: Data1;
		executable: boolean;
		lamports: number;
		owner: string;
		rentEpoch: number;
		space: number;
	};
};
export type GetTokenAccountsByOwner = {
	context: { apiVersion: string; slot: bigint };
	value: [];
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
	// biome-ignore lint/suspicious/noExplicitAny:<>
	methodIx: any,
	// biome-ignore lint/suspicious/noExplicitAny:<>
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
