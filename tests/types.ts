export type Data1 = {
	program: string;
	parsed: {
		info: {
			isNative: boolean;
			mint: string;
			owner: string;
			state: string; // "initialized";
			tokenAmount: {
				amount: string;
				decimals: number;
				uiAmount: number;
				uiAmountString: string;
			};
		};
		type: string; // "account"
	};
	space: number;
};
export type GetTokenAccountsByOwnerValue = {
	pubkey: string;
	account: {
		data: Data1;
		executable: boolean;
		lamports: number;
		owner: string;
		rentEpoch: number;
		space: number;
	};
};

export type GetTokenAccountsByOwner = {
	context: { apiVersion: string; slot: bigint };
	value: [];
};
