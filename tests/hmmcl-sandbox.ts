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
  bnToLeBytes,
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
  let tickStateUpper: PublicKey;
  let tickStateLower: PublicKey;
  let positionState: PublicKey;
  let tokenXVault: PublicKey;
  let tokenYVault: PublicKey;
  let lpTokenVault: PublicKey;

  let poolStateBump: number;
  let tokenXVaultBump: number;
  let tokenYVaultBump: number;
  let lpTokenVaultBump: number;
  let tickStateLowerBump: number;
  let tickStateUpperBump: number;
  let positionStateBump: number;

  let poolStateAccount: any;
  let tickStateUpperAccount: any;
  let tickStateLowerAccount: any;
  let positionStateAccount: any;

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
    [tokenXVault, tokenXVaultBump] =
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
    [tokenYVault, tokenYVaultBump] =
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
    // try {
    //   poolStateAccount = await program.account.poolState.fetch(poolState);
    //   console.log("PRE: pool-state found initialized");
    // } catch (error) {
    //   console.log("PRE: pool-state not found so not initialized?");
    // }

    await program.rpc.initializePool(new BN(15000), new BN(100), {
      accounts: {
        authority: provider.wallet.publicKey,
        payer: provider.wallet.publicKey,
        poolState: poolState,
        tokenXMint: btcdMint,
        tokenYMint: usddMint,
        lpTokenMint: lpTokenMint.publicKey,
        tokenXVault,
        tokenYVault,
        lpTokenVault,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      },
    });

    poolStateAccount = await program.account.poolState.fetch(poolState);
    // try {
    //   poolStateAccount = await program.account.poolState.fetch(poolState);
    //   console.log("POST: pool-state found initialized");
    // } catch (error) {
    //   console.log("POST: pool-state not found so not initialized?");
    // }

    assert.equal(
      poolStateAccount.authority.toString(),
      provider.wallet.publicKey.toString()
    );
    assert.equal(
      poolStateAccount.tokenXVault.toString(),
      tokenXVault.toString()
    );
    assert.equal(
      poolStateAccount.tokenYVault.toString(),
      tokenYVault.toString()
    );

    assert.equal(poolStateAccount.tokenXMint.toString(), btcdMint.toString());
    assert.equal(poolStateAccount.tokenYMint.toString(), usddMint.toString());
    assert.equal(
      poolStateAccount.lpTokenMint.toString(),
      lpTokenMint.publicKey.toString()
    );

    // check that canonical bumps from client-side and program-side match
    assert.equal(poolStateAccount.poolStateBump, poolStateBump);
    assert.equal(poolStateAccount.tokenXVaultBump, tokenXVaultBump);
    assert.equal(poolStateAccount.tokenYVaultBump, tokenYVaultBump);

    // check globalState rp and tick
    // expect(poolStateAccount.poolGlobalState.rp.value.toNumber()).to.equal(15000);
    // expect(poolStateAccount.poolGlobalState.tick.toNumber()).to.equal(100);
  });

  const lowerTick = new BN(100);
  const upperTick = new BN(200);
  const currentTick = new BN(100);

  it("should get the PDA for the TickStateLower", async () => {
    [tickStateLower, tickStateLowerBump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [utf8.encode("tick"), poolState.toBuffer(), bnToLeBytes(lowerTick)],
        program.programId
      );
  });
  it("should get the PDA for the TickStateUpper", async () => {
    [tickStateUpper, tickStateUpperBump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [utf8.encode("tick"), poolState.toBuffer(), bnToLeBytes(upperTick)],
        program.programId
      );
  });

  it("should initialize ticks A and B", async () => {
    try {
      tickStateLowerAccount = await program.account.tickState.fetch(
        tickStateLower
      );
      console.log("PRE: tick-state lower found initialized");
    } catch (error) {
      console.log("PRE: tick-state lower not found; initializing...");
      await program.rpc.initializeTick(lowerTick, {
        accounts: {
          poolState: poolState,
          tickState: tickStateLower,
          payer: anchor.getProvider().wallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        },
      });
      tickStateLowerAccount = await program.account.tickState.fetch(
        tickStateLower
      );
      // console.log(tickStateLowerAccount);
      expect(tickStateLowerAccount.tick.toNumber()).to.equal(
        lowerTick.toNumber()
      );
      expect(tickStateLowerAccount.authority.toString()).to.equal(
        poolState.toString()
      );
    }

    try {
      tickStateUpperAccount = await program.account.tickState.fetch(
        tickStateUpper
      );
      console.log("PRE: tick-state upper found initialized");
    } catch (error) {
      console.log("PRE: tick-state upper not found; initializing...");
      await program.rpc.initializeTick(upperTick, {
        accounts: {
          poolState: poolState,
          tickState: tickStateUpper,
          payer: anchor.getProvider().wallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        },
      });
      tickStateUpperAccount = await program.account.tickState.fetch(
        tickStateUpper
      );
      // console.log(tickStateUpperAccount);
      expect(tickStateUpperAccount.tick.toNumber()).to.equal(
        upperTick.toNumber()
      );
      expect(tickStateUpperAccount.authority.toString()).to.equal(
        poolState.toString()
      );
    }
  });

  it("should get the PDA for the PositionState", async () => {
    [positionState, positionStateBump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [
          utf8.encode("position"),
          poolState.toBuffer(),
          anchor.getProvider().wallet.publicKey.toBuffer(),
          bnToLeBytes(lowerTick),
          bnToLeBytes(upperTick),
        ],
        program.programId
      );
  });

  it("should create position (user,A,B)", async () => {
    try {
      positionStateAccount = await program.account.positionState.fetch(
        positionState
      );
      console.log("PRE: position-state found");
    } catch (error) {
      console.log("PRE: position-state not found; creating...");
      await program.rpc.createPosition(lowerTick, upperTick, {
        accounts: {
          poolState: poolState,
          positionState: positionState,
          lowerTickState: tickStateLower,
          upperTickState: tickStateUpper,
          user: anchor.getProvider().wallet.publicKey,
          payer: anchor.getProvider().wallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        },
      });
      positionStateAccount = await program.account.positionState.fetch(
        positionState
      );
      // console.log(positionStateAccount);
      expect(positionStateAccount.lowerTick.toNumber()).to.equal(
        lowerTick.toNumber()
      );
      expect(positionStateAccount.upperTick.toNumber()).to.equal(
        upperTick.toNumber()
      );
      expect(positionStateAccount.authority.toString()).to.equal(
        poolState.toString()
      );
    }
  });

  const x = new BN(100);
  const y = new BN(10000);
  const diff = new BN(10000);
  const liq3 = new BN(20000);

  it("should make a deposit for user in range (A,B) to liq1", async () => {
    console.log(
      "PRE: deposting (A,B) ",
      x.toString(),
      " and ",
      y.toString(),
      "..."
    );
    await program.rpc.deposit(lowerTick, upperTick, currentTick, x, y, {
      accounts: {
        poolState: poolState,
        positionState: positionState,
        lowerTickState: tickStateLower,
        upperTickState: tickStateUpper,
        currentTickState: tickStateLower,
        lpTokenMint: lpTokenMint.publicKey,
        userTokenX: btcdAccount,
        userTokenY: usddAccount,
        tokenXVault,
        tokenYVault,
        lpTokenVault,
        lpTokenTo: lpTokenAccount,
        tokenProgram: TOKEN_PROGRAM_ID,
        user: anchor.getProvider().wallet.publicKey,
        payer: anchor.getProvider().wallet.publicKey,
      },
    });
    positionStateAccount = await program.account.positionState.fetch(
      positionState
    );
    // console.log(positionStateAccount);
    expect(positionStateAccount.liquidity.value.toNumber()).to.equal(
      x.toNumber()
    );
    expect(positionStateAccount.liquidity.negative).to.equal(false);

    tickStateLowerAccount = await program.account.tickState.fetch(
      tickStateLower
    );
    tickStateUpperAccount = await program.account.tickState.fetch(
      tickStateUpper
    );
    // console.log("lower ", tickStateLowerAccount);
    // console.log("upper ", tickStateUpperAccount);
    // console.log("lower net", tickStateLowerAccount.liqNet.value.toNumber());
    // console.log("lower gross", tickStateLowerAccount.liqGross.value.toNumber());
    // console.log("upper net", tickStateUpperAccount.liqNet.value.toNumber());
    // console.log("upper gross", tickStateUpperAccount.liqGross.value.toNumber());
  });
  // const liq1 = new BN(12345);
  // const liq2 = new BN(2345);
  // const diff = new BN(10000);
  // const liq3 = new BN(20000);
  // it("should update position (user,A,B) to liq1", async () => {
  //   console.log("PRE: setting position (A,B) to ", liq1.toString(), "...");
  //   await program.rpc.updatePosition(liq1, false, lowerTick, upperTick, {
  //     accounts: {
  //       poolState: poolState,
  //       positionState: positionState,
  //       lowerTickState: tickStateLower,
  //       upperTickState: tickStateUpper,
  //       user: anchor.getProvider().wallet.publicKey,
  //       payer: anchor.getProvider().wallet.publicKey,
  //     },
  //   });
  //   positionStateAccount = await program.account.positionState.fetch(
  //     positionState
  //   );
  //   // console.log(positionStateAccount);
  //   expect(positionStateAccount.liquidity.value.toNumber()).to.equal(
  //     liq1.toNumber()
  //   );
  //   expect(positionStateAccount.liquidity.negative).to.equal(false);

  //   tickStateLowerAccount = await program.account.tickState.fetch(
  //     tickStateLower
  //   );
  //   tickStateUpperAccount = await program.account.tickState.fetch(
  //     tickStateUpper
  //   );
  //   // console.log("lower ", tickStateLowerAccount);
  //   // console.log("upper ", tickStateUpperAccount);
  //   // console.log("lower net", tickStateLowerAccount.liqNet.value.toNumber());
  //   // console.log("lower gross", tickStateLowerAccount.liqGross.value.toNumber());
  //   // console.log("upper net", tickStateUpperAccount.liqNet.value.toNumber());
  //   // console.log("upper gross", tickStateUpperAccount.liqGross.value.toNumber());
  // });

  // it("should update position (user,A,B) to liq1-liq2", async () => {
  //   console.log("PRE: adding to position (A,B): minus", liq2.toString(), "...");
  //   await program.rpc.updatePosition(liq2, true, lowerTick, upperTick, {
  //     accounts: {
  //       poolState: poolState,
  //       positionState: positionState,
  //       lowerTickState: tickStateLower,
  //       upperTickState: tickStateUpper,
  //       user: anchor.getProvider().wallet.publicKey,
  //       payer: anchor.getProvider().wallet.publicKey,
  //     },
  //   });
  //   positionStateAccount = await program.account.positionState.fetch(
  //     positionState
  //   );
  //   expect(positionStateAccount.liquidity.value.toNumber()).to.equal(
  //     diff.toNumber()
  //   );
  //   expect(positionStateAccount.liquidity.negative).to.equal(false);

  //   tickStateLowerAccount = await program.account.tickState.fetch(
  //     tickStateLower
  //   );
  //   tickStateUpperAccount = await program.account.tickState.fetch(
  //     tickStateUpper
  //   );
  //   // console.log("lower ", tickStateLowerAccount);
  //   // console.log("upper ", tickStateUpperAccount);
  // });

  // it("should fails to update (user,A,B) to negative", async () => {
  //   console.log("PRE: adding to position (A,B): minus", liq3.toString(), "...");
  //   try {
  //     await program.rpc.updatePosition(liq3, true, lowerTick, upperTick, {
  //       accounts: {
  //         poolState: poolState,
  //         positionState: positionState,
  //         lowerTickState: tickStateLower,
  //         upperTickState: tickStateUpper,
  //         user: anchor.getProvider().wallet.publicKey,
  //         payer: anchor.getProvider().wallet.publicKey,
  //       },
  //     });
  //     assert.ok(false);
  //   } catch (err: any) {
  //     const errMsg = "Insufficient Position Liquidity";
  //     assert.equal(err.toString(), errMsg);
  //     positionStateAccount = await program.account.positionState.fetch(
  //       positionState
  //     );
  //     expect(positionStateAccount.liquidity.value.toNumber()).to.equal(
  //       diff.toNumber()
  //     );
  //     expect(positionStateAccount.liquidity.negative).to.equal(false);
  //   }
  // });
});
