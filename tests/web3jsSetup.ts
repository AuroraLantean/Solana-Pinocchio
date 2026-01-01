//import { struct, u8, u32 } from "@solana/buffer-layout";
//import { bool, publicKey, u64 } from "@solana/buffer-layout-utils";

import { struct, u8, u32 } from "@solana/buffer-layout";
import { bool, publicKey, u64 } from "@solana/buffer-layout-utils";
import { Keypair, PublicKey } from "@solana/web3.js";

export const ownerKp = new Keypair();
export const adminKp = new Keypair();
export const user1Kp = new Keypair();
export const user2Kp = new Keypair();
export const user3Kp = new Keypair();
export const hackerKp = new Keypair();
export const ownerAddr = ownerKp.publicKey;
export const adminAddr = adminKp.publicKey;
export const user1Addr = user1Kp.publicKey;
export const user2Addr = user2Kp.publicKey;
export const user3Addr = user3Kp.publicKey;
export const hackerAddr = hackerKp.publicKey;

const ll = console.log;

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

export interface RawConfig {
	authority: PublicKey;
	strU8array: number[]; // string;
	fee: number[]; // bigint;
	solBalance: number[]; // bigint;
	tokenBalance: number[]; // bigint;
	status: number;
	bump: number;
}
/*export const ConfigLayout = struct<RawConfig>([
	publicKey("authority"),
	//seq(Layout<number>(1), 32),
	cstr("strU8array"),
	u64("fee"),
	u64("solBalance"),
	u64("tokenBalance"),
	u8("status"),
	u8("bump"),
]);*/

export interface RawMint {
	mintAuthorityOption: 1 | 0;
	mintAuthority: PublicKey;
	supply: bigint;
	decimals: number;
	isInitialized: boolean;
	freezeAuthorityOption: 1 | 0;
	freezeAuthority: PublicKey;
}
export const MintLayout = struct<RawMint>([
	u32("mintAuthorityOption"),
	publicKey("mintAuthority"),
	u64("supply"),
	u8("decimals"),
	bool("isInitialized"),
	u32("freezeAuthorityOption"),
	publicKey("freezeAuthority"),
]);
