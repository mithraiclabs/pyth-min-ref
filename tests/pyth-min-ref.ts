import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PythMinRef } from "../target/types/pyth_min_ref";
import { PublicKey, Transaction } from "@solana/web3.js";

describe("pyth-min-ref", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.PythMinRef as Program<PythMinRef>;
  /**
   * This is SOL/USD.
   * See the complete list at: https://docs.pyth.network/price-feeds/sponsored-feeds
   * */
  const priceAcc = new PublicKey(
    "7UVimffxr9ow1uXYxsr4LHAcV58mLzhmwaeKvJ1pjLiE"
  );

  it("Echo price", async () => {
    let tx = new Transaction();
    tx.add(
      await program.methods
        .echoPrice()
        .accounts({
          priceAcc: priceAcc,
        })
        .instruction()
    );
    await program.provider.sendAndConfirm(tx);
  });
});
