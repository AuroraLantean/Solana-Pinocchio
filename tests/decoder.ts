import type { Address, Decoder } from "@solana/kit";
import {
	getAddressDecoder,
	getStructDecoder,
	getU8Decoder,
	getU64Decoder,
} from "@solana/kit";

//converted from Rust code
export type ConfigAcct = {
	authority: Address;
	fee: bigint;
	solBalance: bigint;
	tokenBalance: bigint;
	bump: number;
};
export const configAcctDecoder: Decoder<ConfigAcct> = getStructDecoder([
	//["discriminator", fixDecoderSize(getBytesDecoder(), 4)],
	["authority", getAddressDecoder()],
	["fee", getU64Decoder()],
	["solBalance", getU64Decoder()],
	["tokenBalance", getU64Decoder()],
	["bump", getU8Decoder()],
	//["padding", getArrayDecoder(getU64Decoder(), { size: 3 })],
]);
