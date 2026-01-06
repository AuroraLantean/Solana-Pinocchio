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
