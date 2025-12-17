import {
	type Address,
	getAddressEncoder,
	getProgramDerivedAddress,
	getUtf8Encoder,
} from "@solana/kit";

export const ll = console.log;

import chalk from "chalk";
import * as vault from "../clients/js/src/generated/index";

export const vaultProgAddr = vault.PINOCCHIO_VAULT_PROGRAM_ADDRESS;
ll("vaultProgAddr:", vaultProgAddr);
export const ATokenGPvbd =
	"ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL" as Address<"ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL">;

export const decimalsSOL = BigInt(9);
export const baseSOL = BigInt(10) ** decimalsSOL;
export const amtAirdrop = BigInt(100) * baseSOL;

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

export const makeSolAmt = (amt: number) => BigInt(amt) * baseSOL;

export const findPda = async (
	userAddr: Address<string>,
	str: string,
	progAddr = vaultProgAddr,
) => {
	const seedSigner = getAddressEncoder().encode(userAddr);
	const seedTag = getUtf8Encoder().encode(str);

	const pda_bump = await getProgramDerivedAddress({
		programAddress: progAddr,
		seeds: [seedTag, seedSigner],
	});
	ll(`${str} pda: ${pda_bump[0]}, bump: ${pda_bump[1]}`);
	return { pda: pda_bump[0], bump: pda_bump[1] };
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

export const uint8ToStr = (uint8Arr: number[]) => {
	const filterred = uint8Arr.filter((item) => item !== 0);
	const str = Buffer.from(filterred).toString();
	//ll("string:", str, str.length);
	//const str2 = String.fromCharCode.apply(null, filterred);
	//ll("string:", str2, str2.length);
	return str;
};
