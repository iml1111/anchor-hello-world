import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { SimpleNft } from "../target/types/simple_nft";

describe("simple-nft", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SimpleNft as Program<SimpleNft>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
