import {
	type Address,
	getAddressEncoder,
	getProgramDerivedAddress,
	getUtf8Encoder,
} from "@solana/kit";

export const ll = console.log;

import * as vault from "../clients/js/src/generated/index";

export const vaultProgAddr = vault.PINOCCHIO_VAULT_PROGRAM_ADDRESS;
ll("vaultProgAddr:", vaultProgAddr);

export const decimalsSOL = BigInt(9);
export const baseSOL = BigInt(10) ** decimalsSOL;
export const amtAirdrop = BigInt(100) * baseSOL;

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
