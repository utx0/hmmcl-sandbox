import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { HmmclSandbox } from "../target/types/hmmcl_sandbox";

describe("hmmcl-sandbox", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.HmmclSandbox as Program<HmmclSandbox>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
