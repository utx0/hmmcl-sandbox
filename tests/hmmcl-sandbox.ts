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
  let tickBitmap: PublicKey;
  let tickStateUpper: PublicKey;
  let tickStateLower: PublicKey;
  let tickStateCurrent: PublicKey;
  let positionState: PublicKey;
  let tokenXVault: PublicKey;
  let tokenYVault: PublicKey;
  let lpTokenVault: PublicKey;

  let poolStateBump: number;
  let tickBitmapBump: number;
  let tokenXVaultBump: number;
  let tokenYVaultBump: number;
  let lpTokenVaultBump: number;
  let tickStateLowerBump: number;
  let tickStateUpperBump: number;
  let tickStateCurrentBump: number;
  let positionStateBump: number;

  let poolStateAccount: any;
  let tickBitmapAccount: any;
  let tickStateUpperAccount: any;
  let tickStateLowerAccount: any;
  let tickStateCurrentAccount: any;
  let positionStateAccount: any;

  const lowerTick = new BN(71955); // corres rP = sqrt(1333)
  const upperTick = new BN(80067); // corres rP = sqrt(4000)
  const currentTick = new BN(76012); // corres rP = sqrt(2000)
  const x = new BN(2);
  const y = new BN(4000);

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

  it("should get the PDA for the TickBitmap", async () => {
    [tickBitmap, tickBitmapBump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [utf8.encode("bitmap"), poolState.toBuffer()],
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
    await program.rpc.initializePool(new BN(15000), currentTick, {
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
    expect(poolStateAccount.poolGlobalState.tick.toNumber()).to.equal(
      currentTick.toNumber()
    );
    expect(poolStateAccount.poolGlobalState.rootPrice.toNumber()).to.equal(
      44719511943267
    );
    expect(poolStateAccount.poolGlobalState.rpScale).to.equal(12);
    expect(poolStateAccount.poolGlobalState.liquidity.toNumber()).to.equal(0);
    expect(poolStateAccount.poolGlobalState.liqScale).to.equal(12);

    expect(poolStateAccount.poolGlobalState.globalFee.fX.toNumber()).to.equal(
      0
    );
    expect(poolStateAccount.poolGlobalState.globalFee.feeScale).to.equal(12);
  });

  it("should initialize pool's tick bitmap ", async () => {
    try {
      tickBitmapAccount = await program.account.poolTickBitmap.fetch(
        tickBitmap
      );
      console.log("PRE: tick bitmap found initialized");
    } catch (error) {
      console.log("PRE: tick bitmap not found; initializing...");
      await program.rpc.initializeBitmap({
        accounts: {
          poolState: poolState,
          tickBitmap: tickBitmap,
          payer: anchor.getProvider().wallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        },
      });
      tickBitmapAccount = await program.account.poolTickBitmap.fetch(
        tickBitmap
      );
      console.log("bitmap at 0: %d", tickBitmapAccount.tickMap[0]);
      console.log("bitmap at 4000: %d", tickBitmapAccount.tickMap[500]); // 4000 / 8

      expect(tickBitmapAccount.bump).to.equal(tickBitmapBump);
      expect(tickBitmapAccount.tickMap[0]).to.equal(0);
      expect(tickBitmapAccount.tickMap[500]).to.equal(0); // 4000 / 8
    }
  });

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
  it("should get the PDA for the TickStateCurrent", async () => {
    [tickStateCurrent, tickStateCurrentBump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [utf8.encode("tick"), poolState.toBuffer(), bnToLeBytes(currentTick)],
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
      expect(tickStateLowerAccount.tick.toNumber()).to.equal(
        lowerTick.toNumber()
      );
      expect(tickStateLowerAccount.liqNet.toNumber()).to.equal(0);
      expect(tickStateLowerAccount.liqNetScale).to.equal(12);
      expect(tickStateLowerAccount.liqGross.toNumber()).to.equal(0);
      expect(tickStateLowerAccount.liqGrossScale).to.equal(12);
      expect(tickStateLowerAccount.tickFee.fX.toNumber()).to.equal(0);
      expect(tickStateLowerAccount.tickFee.feeScale).to.equal(12);
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

      expect(tickStateUpperAccount.tick.toNumber()).to.equal(
        upperTick.toNumber()
      );
    }
  });

  it("should initialize tick current ", async () => {
    try {
      tickStateCurrentAccount = await program.account.tickState.fetch(
        tickStateCurrent
      );
      console.log("PRE: tick-state current found initialized");
    } catch (error) {
      console.log("PRE: tick-state current not found; initializing...");
      await program.rpc.initializeTick(currentTick, {
        accounts: {
          poolState: poolState,
          tickState: tickStateCurrent,
          payer: anchor.getProvider().wallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        },
      });
      tickStateCurrentAccount = await program.account.tickState.fetch(
        tickStateCurrent
      );

      expect(tickStateCurrentAccount.tick.toNumber()).to.equal(
        currentTick.toNumber()
      );
      expect(tickStateCurrentAccount.tickFee.feeScale).to.equal(12);
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

      expect(positionStateAccount.lowerTick.toNumber()).to.equal(
        lowerTick.toNumber()
      );
      expect(positionStateAccount.upperTick.toNumber()).to.equal(
        upperTick.toNumber()
      );
      expect(positionStateAccount.liquidity.toNumber()).to.equal(0);
      expect(positionStateAccount.liqScale).to.equal(12);
      expect(positionStateAccount.lastCollectedFee.fX.toNumber()).to.equal(0);
      expect(positionStateAccount.lastCollectedFee.feeScale).to.equal(12);
    }
  });

  it("should make a deposit for user in range (A,B)", async () => {
    console.log(
      "PRE: depositing in (A,B) ",
      x.toString(),
      " and ",
      y.toString(),
      "..."
    );
    await program.rpc.deposit(lowerTick, upperTick, currentTick, x, y, {
      accounts: {
        poolState: poolState,
        tickBitmap: tickBitmap,
        positionState: positionState,
        lowerTickState: tickStateLower,
        upperTickState: tickStateUpper,
        currentTickState: tickStateCurrent,
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

    poolStateAccount = await program.account.poolState.fetch(poolState);
    positionStateAccount = await program.account.positionState.fetch(
      positionState
    );
    tickStateLowerAccount = await program.account.tickState.fetch(
      tickStateLower
    );
    tickStateUpperAccount = await program.account.tickState.fetch(
      tickStateUpper
    );
    tickStateCurrentAccount = await program.account.tickState.fetch(
      tickStateCurrent
    );
    tickBitmapAccount = await program.account.poolTickBitmap.fetch(tickBitmap);

    expect(poolStateAccount.poolGlobalState.tick.toNumber()).to.equal(
      currentTick.toNumber()
    );
    expect(poolStateAccount.poolGlobalState.rootPrice.toNumber()).to.equal(
      44719511943267
    );
    expect(poolStateAccount.poolGlobalState.liquidity.toNumber()).to.equal(
      487204723244326
    );
    expect(positionStateAccount.liquidity.toNumber()).to.equal(487204723244326);

    expect(tickStateLowerAccount.liqNet.toNumber()).to.equal(487204723244326);
    expect(tickStateLowerAccount.liqNetNeg).to.equal(0);
    expect(tickStateLowerAccount.liqGross.toNumber()).to.equal(487204723244326);

    expect(tickStateUpperAccount.liqNet.toNumber()).to.equal(487204723244326);
    expect(tickStateUpperAccount.liqNetNeg).to.equal(1);
    expect(tickStateUpperAccount.liqGross.toNumber()).to.equal(487204723244326);

    expect(tickStateCurrentAccount.liqNet.toNumber()).to.equal(0);
    expect(tickStateCurrentAccount.liqNetNeg).to.equal(0);
    expect(tickStateCurrentAccount.liqGross.toNumber()).to.equal(0);

    console.log("bitmap at 0: %d", tickBitmapAccount.tickMap[0]);
    console.log("bitmap at 40000: %d", tickBitmapAccount.tickMap[5000]); //40000/8

    expect(tickBitmapAccount.tickMap[0]).to.equal(1);
    expect(tickBitmapAccount.tickMap[5000]).to.equal(1); // 40000/8
  });

  // let userLiquidity: BN = await getTokenBalance(
  //   anchor.getProvider(),
  //   lpTokenAccount
  // );
  const userLiquidity: BN = new BN(400);

  it("should make a withdrawal for user in range (A,B)", async () => {
    console.log(
      "PRE: withdrawing from (A,B): ",
      userLiquidity.toString(),
      " lp tokens"
    );

    await program.rpc.withdraw(
      lowerTick,
      upperTick,
      currentTick,
      userLiquidity,
      {
        accounts: {
          poolState: poolState,
          tickBitmap: tickBitmap,
          positionState: positionState,
          lowerTickState: tickStateLower,
          upperTickState: tickStateUpper,
          currentTickState: tickStateCurrent,
          lpTokenMint: lpTokenMint.publicKey,
          userTokenX: btcdAccount,
          userTokenY: usddAccount,
          tokenXVault,
          tokenYVault,
          lpTokenVault,
          lpTokenUserAccount: lpTokenAccount,
          tokenProgram: TOKEN_PROGRAM_ID,
          user: anchor.getProvider().wallet.publicKey,
          payer: anchor.getProvider().wallet.publicKey,
        },
      }
    );

    poolStateAccount = await program.account.poolState.fetch(poolState);
    positionStateAccount = await program.account.positionState.fetch(
      positionState
    );
    tickStateLowerAccount = await program.account.tickState.fetch(
      tickStateLower
    );
    tickStateUpperAccount = await program.account.tickState.fetch(
      tickStateUpper
    );
    tickStateCurrentAccount = await program.account.tickState.fetch(
      tickStateCurrent
    );
    tickBitmapAccount = await program.account.poolTickBitmap.fetch(tickBitmap);

    expect(poolStateAccount.poolGlobalState.tick.toNumber()).to.equal(
      currentTick.toNumber()
    );
    expect(poolStateAccount.poolGlobalState.rootPrice.toNumber()).to.equal(
      44719511943267
    );
    expect(poolStateAccount.poolGlobalState.liquidity.toNumber()).to.equal(
      87204723244326
    );
    expect(positionStateAccount.liquidity.toNumber()).to.equal(87204723244326);

    expect(tickStateLowerAccount.liqNet.toNumber()).to.equal(87204723244326);
    expect(tickStateLowerAccount.liqNetNeg).to.equal(0);
    expect(tickStateLowerAccount.liqGross.toNumber()).to.equal(87204723244326);

    expect(tickStateUpperAccount.liqNet.toNumber()).to.equal(87204723244326);
    expect(tickStateUpperAccount.liqNetNeg).to.equal(1);
    expect(tickStateUpperAccount.liqGross.toNumber()).to.equal(87204723244326);

    expect(tickStateCurrentAccount.liqNet.toNumber()).to.equal(0);
    expect(tickStateCurrentAccount.liqNetNeg).to.equal(0);
    expect(tickStateCurrentAccount.liqGross.toNumber()).to.equal(0);

    console.log("bitmap at 0: %d", tickBitmapAccount.tickMap[0]);
    console.log("bitmap at 40000: %d", tickBitmapAccount.tickMap[5000]); //40000/8

    expect(tickBitmapAccount.tickMap[0]).to.equal(0);
    expect(tickBitmapAccount.tickMap[5000]).to.equal(0); // 40000/8
  });
});
