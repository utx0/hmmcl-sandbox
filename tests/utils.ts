import { Provider, BN } from "@project-serum/anchor";
import {
  PublicKey,
  Keypair,
  Transaction,
  SystemProgram,
} from "@solana/web3.js";
import * as TokenInstructions from "@project-serum/serum/lib/token-instructions";
import { TransactionInstruction } from "@solana/web3.js";

const btcdDecimals = 6;
const usddDecimals = 6;
export const btcdMintAmount = new BN(21_000_000 * 10 ** btcdDecimals);
export const usddMintAmount = new BN(100_000_000 * 10 ** usddDecimals);

export const getTokenBalance = async (
  provider: Provider,
  pubkey: PublicKey
) => {
  return new BN(
    (await provider.connection.getTokenAccountBalance(pubkey)).value.amount
  );
};

export async function createMintInstructions(
  provider: Provider,
  authority: PublicKey,
  mint: PublicKey,
  decimals?: number
): Promise<TransactionInstruction[]> {
  let instructions = [
    SystemProgram.createAccount({
      fromPubkey: provider.wallet.publicKey,
      newAccountPubkey: mint,
      space: 82,
      lamports: await provider.connection.getMinimumBalanceForRentExemption(82),
      programId: TokenInstructions.TOKEN_PROGRAM_ID,
    }),
    TokenInstructions.initializeMint({
      mint,
      decimals: decimals ?? 0,
      mintAuthority: authority,
    }),
  ];
  return instructions;
}

export async function createTokenAccountInstrs(
  provider: Provider,
  newAccountPubkey: PublicKey,
  mint: PublicKey,
  owner: PublicKey,
  lamports?: number
): Promise<TransactionInstruction[]> {
  if (lamports === undefined) {
    lamports = await provider.connection.getMinimumBalanceForRentExemption(165);
  }
  return [
    SystemProgram.createAccount({
      fromPubkey: provider.wallet.publicKey,
      newAccountPubkey,
      space: 165,
      lamports,
      programId: TokenInstructions.TOKEN_PROGRAM_ID,
    }),
    TokenInstructions.initializeAccount({
      account: newAccountPubkey,
      mint,
      owner,
    }),
  ];
}

export async function createMint(
  provider: Provider,
  mint: Keypair,
  authority: PublicKey,
  decimals: number
): Promise<PublicKey> {
  const instructions = await createMintInstructions(
    provider,
    authority,
    mint.publicKey,
    decimals
  );

  const tx = new Transaction();
  tx.add(...instructions);

  await provider.send(tx, [mint]);

  return mint.publicKey;
}
