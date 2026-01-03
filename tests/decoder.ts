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
	getU64Decoder,
	getUtf8Decoder,
} from "@solana/kit";

const ll = console.log;
//Methods to decode: use SolanaKit, Borsh, or BufferLayout, or separate str then decode the rest;

//converted from Rust code. XyzAcct, xyzAcctDecoder, DecodedXyzAcct should all match in field order and types!
export type ConfigAcct = {
	progOwner: Address;
	admin: Address;
	strU8array: string;
	fee: bigint;
	solBalance: bigint;
	tokenBalance: bigint;
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
//https://www.quicknode.com/guides/solana-development/tooling/web3-2/account-deserialization
//https://github.com/anza-xyz/kit/tree/main/packages/codecs-core#fixed-size-and-variable-size-codecs
export const configAcctDecoder: FixedSizeDecoder<ConfigAcct> = getStructDecoder(
	[
		//["discriminator", fixDecoderSize(getBytesDecoder(), 4)],
		["progOwner", getAddressDecoder()],
		["admin", getAddressDecoder()],
		["strU8array", fixDecoderSize(getUtf8Decoder(), 32)],
		//["strU8array", getArrayDecoder(getU8Decoder(), { size: 32 })],
		["fee", getU64Decoder()],
		["solBalance", getU64Decoder()],
		["tokenBalance", getU64Decoder()],
		["isAuthorized", getBooleanDecoder()],
		["status", getEnumDecoder(Status)],
		//https://github.com/anza-xyz/kit/tree/main/packages/codecs-data-structures#enum-codec
		["bump", getU8Decoder()],
		//["padding", getArrayDecoder(getU64Decoder(), { size: 3 })],
	],
);
export const solanaKitDecode = (
	bytes: ReadonlyUint8Array | Uint8Array<ArrayBufferLike> | undefined,
) => {
	if (!bytes) throw new Error("bytes invalid");
	const decoded = configAcctDecoder.decode(bytes);
	ll("progOwner:", decoded.progOwner);
	ll("admin:", decoded.admin);
	ll("strU8array:", decoded.strU8array);
	ll("fee:", decoded.fee);
	ll("solBalance:", decoded.solBalance);
	ll("tokenBalance:", decoded.tokenBalance);
	ll("isAuthorized:", decoded.isAuthorized);
	ll("status:", decoded.status);
	ll("bump:", decoded.bump);
	return decoded;
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
		strU8array: string;
		fee: bigint;
		solBalance: bigint;
		tokenBalance: bigint;
		isAuthorized: boolean;
		status: Status;
		bump: number;
	};
	exists: boolean;
};
