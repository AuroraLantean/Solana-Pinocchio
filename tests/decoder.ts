import type { Address, Decoder } from "@solana/kit";
import {
	getAddressDecoder,
	getEnumDecoder,
	getStructDecoder,
	getU8Decoder,
	getU64Decoder,
} from "@solana/kit";

//converted from Rust code
export type ConfigAcct = {
	authority: Address;
	//status: number;
	fee: bigint;
	solBalance: bigint;
	tokenBalance: bigint;
	bump: number;
};
enum Status {
	Waiting,
	Active,
	Expired,
	Paused,
}
export const configAcctDecoder: Decoder<ConfigAcct> = getStructDecoder([
	//["discriminator", fixDecoderSize(getBytesDecoder(), 4)],
	["authority", getAddressDecoder()],
	["fee", getU64Decoder()],
	["solBalance", getU64Decoder()],
	["tokenBalance", getU64Decoder()],
	["status", getEnumDecoder(Status)],
	["bump", getU8Decoder()],
	//["padding", getArrayDecoder(getU64Decoder(), { size: 3 })],
]);

export type DecodedConfigAcct = {
	executable: boolean;
	lamports: bigint;
	programAddress: string;
	space: bigint;
	address: string;
	data: {
		authority: string;
		fee: bigint;
		solBalance: bigint;
		tokenBalance: bigint;
		status: number; //TODO: decode u8 to Enum
		bump: number;
	};
	exists: boolean;
};
