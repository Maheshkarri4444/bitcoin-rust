export type Output = { amount: number; address: string };

export type Input = {
  timestamp: number;
  amount: number;
  address: string;
  signature: number[];
};

export type NormalTx = {
  id: string;
  input: Input;
  outputs: Output[];
};

export type RewardTx = {
  id: string;
  coinbase: string;
  output: Output;
};

export type ChainTransaction =
  | { kind: "Normal"; tx: NormalTx }
  | { kind: "Reward"; tx: RewardTx };

export type Block = {
  block_number: number;
  timestamp: number;
  last_hash: string;
  hash: string;
  data: ChainTransaction[];
  nonce: number;
  difficulty: number;
};

export type BalanceResp = { address: string; balance: number };
export type PublicKeyResp = { public_key: string };
