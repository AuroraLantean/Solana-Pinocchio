//import * as borsh from "@coral-xyz/borsh";
import { cstr, struct, u8, u32 } from "@solana/buffer-layout";
import { bool, publicKey, u64 } from "@solana/buffer-layout-utils";
import { Keypair, PublicKey } from "@solana/web3.js";

const ll = console.log;
ll("\n------== web3jsSetup");
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

//---------------==
export type RawConfig = {
	progOwner: PublicKey;
	admin: PublicKey;
	strU8array1: string;
	strU8array2: string;
	strU8array3: string;
	strU8array4: string;
	strU8array5: string;
	strU8array6: string;
	strU8array7: string;
	strU8array8: string;
	strU8array9: string;
	strU8array10: string;
	strU8array11: string;
	strU8array12: string;
	fee: bigint;
	solBalance: bigint;
	tokenBalance: bigint;
	isAuthorized: boolean;
	status: number;
	bump: number;
};
export const ConfigLayout = struct<RawConfig>([
	publicKey("progOwner"),
	publicKey("admin"),
	//blob(32, "strU8array1"),
	//utf8(Layout<string>),
	//utf8(32, "strU8array"), // how does this work?
	cstr("strU8array1"), // can decode but how to specify byte array length?   CString: Contain a NUL-terminated UTF8 string. Factory: :Layout.cstr|cstr NOTE Any UTF8 string that incorporates a zero-valued byte will not be correctly decoded by this layout.
	cstr("strU8array2"),
	cstr("strU8array3"),
	cstr("strU8array4"),
	cstr("strU8array5"),
	cstr("strU8array6"),
	cstr("strU8array7"),
	cstr("strU8array8"),
	cstr("strU8array9"),
	cstr("strU8array10"),
	cstr("strU8array11"),
	cstr("strU8array12"),
	u64("fee"),
	u64("solBalance"),
	u64("tokenBalance"),
	bool("isAuthorized"),
	u8("status"),
	u8("bump"),
]);
export const bufferLayoutDecodeConfig = (
	bytes: Uint8Array<ArrayBufferLike> | undefined,
) => {
	if (!bytes) throw new Error("bytes invalid");
	const decoded = ConfigLayout.decode(bytes);
	ll("progOwner:", decoded.progOwner.toBase58());
	ll("admin:", decoded.admin.toBase58());
	ll("strU8array:", decoded.strU8array1);
	ll("fee:", decoded.fee);
	ll("solBalance:", decoded.solBalance);
	ll("tokenBalance:", decoded.tokenBalance);
	ll("isAuthorized:", decoded.isAuthorized);
	ll("status:", decoded.status);
	ll("bump:", decoded.bump);
	return decoded;
};
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
//---------------== Borsh Decoder
/*export const borshAccountSchema = borsh.struct([
	borsh.publicKey("progOwner"),
	borsh.publicKey("admin"),
	borsh.str("strU8array"),
	borsh.u64("fee"),
	borsh.u64("solBalance"),
	borsh.u64("tokenBalance"),
	borsh.bool("isAuthorized"),
	borsh.u8("status"),
	//borsh.rustEnum([Status.Waiting], "status"),
	borsh.u8("bump"),
]); //const decodedBorsh = borshAccountSchema.decode(bytes);
*/
