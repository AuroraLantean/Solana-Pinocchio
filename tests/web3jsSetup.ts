import { Keypair, PublicKey } from "@solana/web3.js";

const ll = console.log;
ll("\n------== web3jsSetup");
export const ownerKp = new Keypair();
export const adminKp = new Keypair();
export const user1Kp = new Keypair();
export const user2Kp = new Keypair();
export const user3Kp = new Keypair();
export const hackerKp = new Keypair();
export const dragonCoinAuthorityKp = new Keypair();
export const dragonCoinKp = new Keypair();
export const owner = ownerKp.publicKey;
export const admin = adminKp.publicKey;
export const user1 = user1Kp.publicKey;
export const user2 = user2Kp.publicKey;
export const user3 = user3Kp.publicKey;
export const hacker = hackerKp.publicKey;
export const dragonCoinAuthority = dragonCoinAuthorityKp.publicKey;
export const dragonCoin = dragonCoinKp.publicKey;

export const vaultProgAddr = new PublicKey(
	"7EKqBVYSCmJbt2T8tGSmwzNKnpL29RqcJcyUr9aEEr6e",
);
ll("vaultProgAddr:", vaultProgAddr.toBase58());

export const systemProgram = new PublicKey("11111111111111111111111111111111");
export const usdcMint = new PublicKey(
	"EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
);
export const usdtMint = new PublicKey(
	"Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB",
);
export const ATokenGPvbd = new PublicKey(
	"ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL",
);
