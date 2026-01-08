import type { Lamports } from "@solana/kit";
import {
	type Address,
	getAddressEncoder,
	getLamportsDecoder,
	getLamportsEncoder,
	getProgramDerivedAddress,
	getU8Encoder,
	getU16Decoder,
	getU32Encoder,
	getU64Decoder,
	getU64Encoder,
	getUtf8Encoder,
	lamports,
} from "@solana/kit";
import chalk from "chalk";
import * as vault from "../clients/js/src/generated/index";
import { Status } from "./decoder";

export const ll = console.log;
//-----------== General Config
export const network = "mainnet-beta"; //devnet
export const PROJECT_DIRECTORY = ""; // Leave empty if using default anchor project

export const USDC_DECIMALS = 6;
export const USDT_DECIMALS = 6;
export const LAMPORTS_PER_SOL = 1000000000;

export const MINIMUM_SLOT = 100;
export const USDC_BALANCE = 100_000_000_000; // 100k USDC
export const Transaction_Fee = 5000n;
export const day = 86400;
export const week = 604800;

export const decimalsSOL = BigInt(9);
export const baseSOL = BigInt(10) ** decimalsSOL;

//-----------==
export const bigintAmt = (amount: number, decimals: number) =>
	BigInt(amount) * 10n ** BigInt(decimals);

export const as9zBn = (amt: number) => {
	if (Number.isInteger(amt)) {
		return BigInt(amt) * baseSOL;
	}
	return BigInt(amt * 10 ** 9);
};
export const fromLam = (amt: number) => BigInt(amt) / baseSOL;

//-----------== SolanaKit setup
export const vaultProgAddr = vault.PINOCCHIO_VAULT_PROGRAM_ADDRESS;
ll("vaultProgAddr:", vaultProgAddr);
export const ATokenGPvbd =
	"ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL" as Address<"ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL">;
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

//-----------==
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
export const u16Bytes = [0, 0];
export const u32Bytes = [0, 0, 0, 0];
export const u32x4Bytes = [...u32Bytes, ...u32Bytes, ...u32Bytes, ...u32Bytes];
export const u64Bytes = [0, 0, 0, 0, 0, 0, 0, 0];
export const u64x4Bytes = [...u64Bytes, ...u64Bytes, ...u64Bytes, ...u64Bytes];
export const u32Max = 4294967295n;
export const u8Max = 255n;

export const bigintToBytes = (input: bigint | number, bit = 64) => {
	let amtBigint = 0n;
	if (typeof input === "number") {
		amtBigint = BigInt(input);
	} else {
		amtBigint = input;
	}
	const amtLam = lamports(amtBigint);
	// biome-ignore lint/suspicious/noExplicitAny: <>
	let lamportsEncoder: any;
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
	const u8Bytes: Uint8Array = lamportsEncoder.encode(amtLam);
	ll("u8Bytes", u8Bytes);
	return u8Bytes;
};
export const testByteConversion = () => {
	ll("\n------== inputNum to/from Bytes");
	const amountNum = as9zBn(1.23);
	const argData64 = bigintToBytes(amountNum);
	const _amtOut64 = bytesToBigint(argData64);

	const time1 = 1766946349;
	const argData32 = bigintToBytes(time1, 32);
	const _amtOut32 = bytesToBigint(argData32);

	const u8Num = 37;
	const argDataU8 = bigintToBytes(u8Num, 8);
	const _amtOut8 = bytesToBigint(argDataU8);
};
export const bytesToBigint = (bytes: Uint8Array) => {
	let bigint: Lamports = lamports(0n);
	const length = bytes.length;
	// bytes = decoder.decode(new Uint8Array([0x2a, 0x00, 0x00, 0x00]));
	if (length === 8) {
		//u64. Returns a decoder that you can use to decode a byte array representing a 64-bit little endian number to a {@link Lamports} value.
		const lamportsDecoder = getLamportsDecoder(getU64Decoder()); //getDefaultLamportsDecoder()
		bigint = lamportsDecoder.decode(bytes);
	} else if (length === 4) {
		//u32
		const newBytes = new Uint8Array([...bytes, 0, 0, 0, 0]);
		const lamportsDecoder = getLamportsDecoder(getU64Decoder());
		bigint = lamportsDecoder.decode(newBytes);
		/*const _decoder = getU32Decoder();
		const _lamportsDecoder = getLamportsEncoder(decoder);*/
	} else if (length === 2) {
		//u16
		const lamportsDecoder = getLamportsDecoder(getU16Decoder());
		bigint = lamportsDecoder.decode(bytes);
	} else if (length === 1) {
		//u8
		const newBytes = new Uint8Array([...bytes, 0, 0, 0, 0, 0, 0, 0]);
		const lamportsDecoder = getLamportsDecoder(getU64Decoder());
		bigint = lamportsDecoder.decode(newBytes);
		/*const _decoder = getU8Decoder();
		const _lamportsDecoder = getLamportsEncoder(decoder);*/
	} else {
		throw new Error("bit unknown");
		//lamportsEncoder = getDefaultLamportsCodec()
	}
	ll("bytesToBigint:", bigint);
	return bigint;
};

export const strToU8Fixed = (str: string, size = 32) => {
	const u8input = strToU8Array(str);
	const inputLen = u8input.length;
	if (inputLen > size) throw new Error("fixed size is too small");
	if (inputLen === size) return u8input;

	const u8out = new Uint8Array(size);
	//ll("u8out:", u8out);
	for (let i = 0; i < inputLen; i++) {
		if (u8out[i] !== undefined && u8input[i] !== undefined) {
			// biome-ignore lint/style/noNonNullAssertion: <>
			u8out[i] = u8input[i]!;
		} else {
			throw new Error("undefined detected");
		}
	}
	ll("u8out:", u8out);
	return u8out;
};
//ASCII: Each char uses exactly 1 byte(8 bits)
export const strToU8Array = (str: string) => {
	const u8array = Uint8Array.from(
		Array.from(str).map((letter) => letter.charCodeAt(0)),
	);
	ll(str, "to u8:", u8array);
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
export const boolToByte = (input: boolean) => (input ? 1 : 0);
export const statusToByte = (status: Status) => {
	let out = -1;
	switch (status) {
		case Status.Waiting:
			{
				out = 0;
			}
			break;
		case Status.Active:
			{
				out = 1;
			}
			break;
		case Status.Expired:
			{
				out = 2;
			}
			break;
		case Status.Paused:
			{
				out = 3;
			}
			break;
		case Status.Canceled:
			{
				out = 4;
			}
			break;
		default: {
			throw new Error("status invalid");
		}
	}
	return out;
};

export const getTime = () => {
	const time = Math.floor(Date.now() / 1000);
	ll("time:", time);
	return time;
};
export const getTimeB = () => {
	const time = getTime();
	return BigInt(time);
};
