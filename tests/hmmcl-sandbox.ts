import * as anchor from "@project-serum/anchor";
import { BN, Program } from "@project-serum/anchor";
import { HmmclSandbox } from "../target/types/hmmcl_sandbox";

import { Keypair, PublicKey } from "@solana/web3.js";

import { createMintAndVault, createTokenAccount } from "@project-serum/common";
import { TOKEN_PROGRAM_ID } from "@project-serum/serum/lib/token-instructions";

import assert from "assert";
import { expect } from "chai";

import {
  createMint,
  btcdMintAmount,
  usddMintAmount,
  getTokenBalance,
} from "./utils";

const utf8 = anchor.utils.bytes.utf8;

describe("hmmcl-sandbox", () => {
  // Configure the client to use the local cluster.
  // anchor.setProvider(anchor.Provider.env());
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.HmmclSandbox as Program<HmmclSandbox>;

  let btcdMint: PublicKey;
  let usddMint: PublicKey;
  let btcdAccount: PublicKey;
  let usddAccount: PublicKey;
  const lpTokenMint = Keypair.generate();
  let lpTokenAccount: PublicKey;

  let poolState: PublicKey;
  let baseTokenVault: PublicKey;
  let quoteTokenVault: PublicKey;
  let lpTokenVault: PublicKey;

  let poolStateBump: number;
  let baseTokenVaultBump: number;
  let quoteTokenVaultBump: number;
  let lpTokenVaultBump: number;
  let poolStateAccount: any;

  it("should create btcdMint (21 million)", async () => {
    [btcdMint, btcdAccount] = await createMintAndVault(
      provider,
      btcdMintAmount,
      provider.wallet.publicKey,
      6
    );
  });

  it("should create usddMint (100 million)", async () => {
    [usddMint, usddAccount] = await createMintAndVault(
      provider,
      usddMintAmount,
      provider.wallet.publicKey,
      6
    );
  });

  it("should create usddMint (100 million)", async () => {
    [usddMint, usddAccount] = await createMintAndVault(
      provider,
      usddMintAmount,
      provider.wallet.publicKey,
      6
    );
  });

  it("should get the PDA for the PoolState", async () => {
    [poolState, poolStateBump] = await anchor.web3.PublicKey.findProgramAddress(
      [utf8.encode("pool_state_seed"), lpTokenMint.publicKey.toBuffer()],
      program.programId
    );
  });

  it("should create lpTokenMint with poolState as the authority", async () => {
    const pb = await createMint(provider, lpTokenMint, poolState, 9);
    assert.equal(pb.toString(), lpTokenMint.publicKey.toString());
  });

  it("should create a lpTokenAccount", async () => {
    lpTokenAccount = await createTokenAccount(
      provider,
      lpTokenMint.publicKey,
      provider.wallet.publicKey
    );
  });

  it("should get the PDA for the TokenAVault", async () => {
    [baseTokenVault, baseTokenVaultBump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [
          utf8.encode("token_vault_seed"),
          btcdMint.toBuffer(),
          lpTokenMint.publicKey.toBuffer(),
        ],
        program.programId
      );
  });

  it("should get the PDA for the TokenBVault", async () => {
    [quoteTokenVault, quoteTokenVaultBump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [
          utf8.encode("token_vault_seed"),
          usddMint.toBuffer(),
          lpTokenMint.publicKey.toBuffer(),
        ],
        program.programId
      );
  });

  it("should get the PDA for lpTokenVault", async () => {
    [lpTokenVault, lpTokenVaultBump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [
          utf8.encode("lp_token_vault_seed"),
          poolState.toBuffer(),
          lpTokenMint.publicKey.toBuffer(),
        ],
        program.programId
      );
  });

  it("should initialize a liquidity-pool", async () => {
    await program.rpc.initializePool(
      baseTokenVaultBump,
      quoteTokenVaultBump,
      poolStateBump,
      lpTokenVaultBump,
      {
        accounts: {
          authority: provider.wallet.publicKey,
          payer: provider.wallet.publicKey,
          poolState: poolState,
          baseTokenMint: btcdMint,
          quoteTokenMint: usddMint,
          lpTokenMint: lpTokenMint.publicKey,
          baseTokenVault,
          quoteTokenVault,
          lpTokenVault,
          systemProgram: anchor.web3.SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        },
      }
    );

    poolStateAccount = await program.account.poolState.fetch(poolState);

    assert.equal(
      poolStateAccount.authority.toString(),
      provider.wallet.publicKey.toString()
    );
    assert.equal(
      poolStateAccount.baseTokenVault.toString(),
      baseTokenVault.toString()
    );
    assert.equal(
      poolStateAccount.quoteTokenVault.toString(),
      quoteTokenVault.toString()
    );

    assert.equal(
      poolStateAccount.baseTokenMint.toString(),
      btcdMint.toString()
    );
    assert.equal(
      poolStateAccount.quoteTokenMint.toString(),
      usddMint.toString()
    );
    assert.equal(
      poolStateAccount.lpTokenMint.toString(),
      lpTokenMint.publicKey.toString()
    );
    assert.equal(poolStateAccount.poolStateBump, poolStateBump);
    assert.equal(poolStateAccount.baseTokenVaultBump, baseTokenVaultBump);
    assert.equal(poolStateAccount.quoteTokenVaultBump, quoteTokenVaultBump);
  });
});
