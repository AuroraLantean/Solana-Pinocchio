//import * as borsh from "@coral-xyz/borsh";
import { cstr, struct, u8, u32 } from "@solana/buffer-layout";
import { bool, publicKey, u64 } from "@solana/buffer-layout-utils";
import type {
	Address,
	FixedSizeDecoder,
	ReadonlyUint8Array,
} from "@solana/kit";
import {
	fixDecoderSize,
	getAddressDecoder,
	getBooleanDecoder,
	getEnumDecoder,
	getStructDecoder,
	getU8Decoder,
	getU32Decoder,
	getU64Decoder,
	getUtf8Decoder,
} from "@solana/kit";
import { PublicKey } from "@solana/web3.js";

const ll = console.log;
//Methods to decode: use SolanaKit, Borsh, or BufferLayout, or separate str then decode the rest;

//converted from Rust code. XyzAcct, xyzAcctDecoder, DecodedXyzAcct should all match in field order and types!
export type ConfigAcct = {
	progOwner: Address;
	admin: Address;
	str: string;
	fee: bigint;
	solBalance: bigint;
	tokenBalance: bigint;
	updatedAt: number;
	isAuthorized: boolean;
	status: Status;
	bump: number;
};
export enum Status {
	Waiting,
	Active,
	Expired,
	Paused,
	Canceled,
}
/*const base64Encoder = getBase64Encoder();
let bytes = base64Encoder.encode(value.data[0]);
const decoded = ammConfigDecoder.decode(bytes);*/

//https://www.quicknode.com/guides/solana-development/tooling/web3-2/account-deserialization
//https://github.com/anza-xyz/kit/tree/main/packages/codecs-core#fixed-size-and-variable-size-codecs
export const configAcctDecoder: FixedSizeDecoder<ConfigAcct> = getStructDecoder(
	[
		//["discriminator", fixDecoderSize(getBytesDecoder(), 4)],//only for accounts made by Anchor
		["progOwner", getAddressDecoder()],
		["admin", getAddressDecoder()],
		["str", fixDecoderSize(getUtf8Decoder(), 32)],
		["fee", getU64Decoder()],
		["solBalance", getU64Decoder()],
		["tokenBalance", getU64Decoder()],
		["updatedAt", getU32Decoder()],
		["isAuthorized", getBooleanDecoder()],
		["status", getEnumDecoder(Status)],
		//https://github.com/anza-xyz/kit/tree/main/packages/codecs-data-structures#enum-codec
		["bump", getU8Decoder()],
		//["padding", getArrayDecoder(getU64Decoder(), { size: 3 })],
	],
);
export const solanaKitDecode = (
	bytes: ReadonlyUint8Array | Uint8Array<ArrayBufferLike>,
	isVerbose = false,
) => {
	const decoded = configAcctDecoder.decode(bytes);
	if (isVerbose) {
		ll("progOwner:", decoded.progOwner);
		ll("admin:", decoded.admin);
		ll("str:", decoded.str);
		ll("fee:", decoded.fee);
		ll("solBalance:", decoded.solBalance);
		ll("tokenBalance:", decoded.tokenBalance);
		ll("updatedAt:", decoded.updatedAt);
		ll("isAuthorized:", decoded.isAuthorized);
		ll("status:", decoded.status);
		ll("bump:", decoded.bump);
	}
	return decoded;
};
// This below is only used for testing as it is outputing PublicKey, not Address
export const solanaKitDecodeDev = (
	bytes: ReadonlyUint8Array | Uint8Array<ArrayBufferLike> | undefined,
) => {
	if (!bytes) throw new Error("bytes invalid");
	const decoded = solanaKitDecode(bytes, true);
	const decodedV1: ConfigAcctV1 = {
		progOwner: new PublicKey(decoded.progOwner.toString()),
		admin: new PublicKey(decoded.admin.toString()),
		str: decoded.str,
		fee: decoded.fee,
		solBalance: decoded.solBalance,
		tokenBalance: decoded.tokenBalance,
		updatedAt: decoded.updatedAt,
		isAuthorized: decoded.isAuthorized,
		status: decoded.status,
		bump: decoded.bump,
	};
	return decodedV1;
};
export type ConfigAcctV1 = {
	progOwner: PublicKey;
	admin: PublicKey;
	str: string;
	fee: bigint;
	solBalance: bigint;
	tokenBalance: bigint;
	updatedAt: number;
	isAuthorized: boolean;
	status: Status;
	bump: number;
};

export type DecodedConfigAcct = {
	executable: boolean;
	lamports: bigint;
	programAddress: string;
	space: bigint;
	address: string;
	data: {
		progOwner: string;
		admin: string;
		str: string;
		fee: bigint;
		solBalance: bigint;
		tokenBalance: bigint;
		isAuthorized: boolean;
		status: Status;
		bump: number;
	};
	exists: boolean;
};

//---------------== BufferLayout code below is replaced by SolanaKit decoder
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

//Set account via knowing its layout
export interface RawTokenAcct2022 {
	mint: PublicKey;
	owner: PublicKey;
	amount: bigint;
	delegateFlag: number;
	delegate: PublicKey;
	state: number;
	isNative: number;
	nativeAmount: bigint;
	delegatedAmount: bigint;
	closeAuthorityFlag: number;
	closeAuthority: PublicKey;
}
// Buffer layout for de/serializing a token account
export const AccountLayout2022 = struct<RawTokenAcct2022>([
	publicKey("mint"),
	publicKey("owner"),
	u64("amount"),
	u32("delegateFlag"),
	publicKey("delegate"),
	u8("state"),
	u32("isNative"),
	u64("nativeAmount"),
	u64("delegatedAmount"),
	u32("closeAuthorityFlag"),
	publicKey("closeAuthority"),
]);

// Byte length of a token account
export const ACCOUNT_SIZE2022 = AccountLayout2022.span;

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
