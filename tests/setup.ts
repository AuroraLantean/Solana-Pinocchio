import {
	airdropFactory,
	createSolanaRpc,
	createSolanaRpcSubscriptions,
	generateKeyPairSigner,
	lamports,
} from "@solana/kit";

const httpProvider = "http://127.0.0.1:8899";
const wssProvider = "ws://127.0.0.1:8900";

export const decimalsSOL = BigInt(9);
export const baseSOL = BigInt(10) ** decimalsSOL;
export const amtAirdrop = BigInt(100) * baseSOL;

export const rpc = createSolanaRpc(httpProvider);
export const rpcSubscriptions = createSolanaRpcSubscriptions(wssProvider);
console.log(`✅ - Established connection to ${httpProvider}`);

//https://www.solanakit.com/docs/getting-started/signers
export const adminKp = await generateKeyPairSigner();
export const mintAuthorityKp = await generateKeyPairSigner();
export const user1Kp = await generateKeyPairSigner();
export const hackerKp = await generateKeyPairSigner();
export const mintKp = await generateKeyPairSigner();
//import secret from './my-keypair.json';
//const user2 = await createKeyPairSignerFromBytes(new Uint8Array(secret));
export const adminAddr = adminKp.address;
export const mint = mintKp.address;
export const mintAuthority = mintAuthorityKp.address;
export const user1Addr = user1Kp.address;

const ll = console.log;
ll(`✅ adminAddr ${adminAddr}`);
ll(`✅ mint: ${mint}`);
ll(`✅ mintAuthority: ${mintAuthority}`);
ll(`✅ user1Addr: ${user1Addr}`);

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
