import {
	type Address,
	getAddressEncoder,
	getProgramDerivedAddress,
	getUtf8Encoder,
} from "@solana/kit";

export const ll = console.log;

import type { Lamports } from "@solana/kit";
import {
	getLamportsDecoder,
	getLamportsEncoder,
	getU8Decoder,
	getU8Encoder,
	getU16Decoder,
	getU32Decoder,
	getU32Encoder,
	getU64Decoder,
	getU64Encoder,
	lamports,
} from "@solana/kit";
import chalk from "chalk";
import * as vault from "../clients/js/src/generated/index";

export const vaultProgAddr = vault.PINOCCHIO_VAULT_PROGRAM_ADDRESS;
ll("vaultProgAddr:", vaultProgAddr);
export const ATokenGPvbd =
	"ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL" as Address<"ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL">;

export const network = "mainnet-beta"; //devnet
export const PROJECT_DIRECTORY = ""; // Leave empty if using default anchor project
export const USDC_DECIMALS = 6;
export const USDC_MINT_ADDRESS = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v";
export const USDT_MINT_ADDRESS = "Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB";
export const USDT_DECIMALS = 6;

export const MINIMUM_SLOT = 100;
export const USDC_BALANCE = 100_000_000_000; // 100k USDC
export const Transaction_Fee = 5000n;
export const day = 86400;
export const week = 604800;

export const decimalsSOL = BigInt(9);
export const baseSOL = BigInt(10) ** decimalsSOL;
export const amtAirdrop = BigInt(100) * baseSOL;
export const toLam = (amt: number) => {
	if (Number.isInteger(amt)) {
		return BigInt(amt) * baseSOL;
	}
	return BigInt(amt * 10 ** 9);
};
export const fromLam = (amt: number) => BigInt(amt) / baseSOL;

export const findPdaV2 = async (
	addr: Address<string>,
	seedStr: string,
	pdaName: string,
	progAddr = vaultProgAddr,
) => {
	const seedSigner = getAddressEncoder().encode(addr);
	const seedTag = getUtf8Encoder().encode(seedStr);

	const [pda, bump] = await getProgramDerivedAddress({
		programAddress: progAddr,
		seeds: [seedTag, seedSigner],
	});
	ll(`${pdaName} pda: ${pda}, bump: ${bump}`);
	return { pda, bump };
};

export const llBl = (txt: string) => {
	ll(chalk.blue(txt));
};
export const llGn = (txt: string) => {
	ll(chalk.green(txt));
};
export const llRd = (txt: string) => {
	ll(chalk.red(txt));
};
export const llYl = (txt: string) => {
	ll(chalk.yellow(txt));
};
export const llbalc = (name: string, amt: string) => {
	ll(`${chalk.bgBlue(name)} balc: ${chalk.yellow(amt)}`);
};
//--------------== Bytes
export const bigintToBytes = (amtLamport: bigint, bit = 64) => {
	const amtLam = lamports(amtLamport);
	// biome-ignore lint/suspicious/noExplicitAny: <>
	let lamportsEncoder: any;
	let lamportsBytes = [];
	if (bit === 64) {
		lamportsEncoder = getLamportsEncoder(getU64Encoder());
	} else if (bit === 32) {
		lamportsEncoder = getLamportsEncoder(getU32Encoder());
	} else if (bit === 8) {
		lamportsEncoder = getLamportsEncoder(getU8Encoder());
	} else {
		throw new Error("bit unknown");
		//lamportsEncoder = getDefaultLamportsEncoder()
	}
	lamportsBytes = lamportsEncoder.encode(amtLam);
	ll("lamportsBytes", lamportsBytes);
	return lamportsBytes;
};
export const bytesToBigint = (lamportsBytes: Uint8Array, bit = 64) => {
	let lam: Lamports = lamports(0n);
	// lamportsBytes = decoder.decode(new Uint8Array([0x2a, 0x00, 0x00, 0x00]));
	if (bit === 64) {
		//Returns a decoder that you can use to decode a byte array representing a 64-bit little endian number to a {@link Lamports} value.
		const lamportsDecoder = getLamportsDecoder(getU64Decoder()); //getDefaultLamportsDecoder()
		lam = lamportsDecoder.decode(lamportsBytes); // lamports(256n)
	} else if (bit === 32) {
		const _decoder = getU32Decoder();
		//const _lamportsDecoder = getLamportsEncoder(decoder);
		//lam = lamportsDecoder.decode(lamportsBytes); // lamports(256n)
	} else if (bit === 16) {
		const lamportsDecoder = getLamportsDecoder(getU16Decoder());
		lam = lamportsDecoder.decode(lamportsBytes); // lamports(256n)
	} else if (bit === 8) {
		const _decoder = getU8Decoder();
		//const _lamportsDecoder = getLamportsEncoder(decoder);
	} else {
		throw new Error("bit unknown");
		//lamportsEncoder = getDefaultLamportsCodec()
	}
	ll("bytesToBigint lamports:", lam);
	return lam;
};

export const strToU8Array = (str: string) => {
	const u8array = Uint8Array.from(
		Array.from(str).map((letter) => letter.charCodeAt(0)),
	);
	ll("u8array:", u8array);
	return u8array;
};
export const u8ArrayToStr = (u8Array: Uint8Array) => {
	const filterred = u8Array.filter((item) => item !== 0);
	const str = Buffer.from(filterred).toString();
	//ll("string:", str, str.length);
	//const str2 = String.fromCharCode.apply(null, filterred);
	//ll("string:", str2, str2.length);
	ll("string:", str);
	return str;
};
export const getTime = () => {
	return Math.floor(Date.now() / 1000);
};
