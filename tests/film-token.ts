import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { FilmToken } from "../target/types/film_token";

describe("film-token", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  it("Is initialized!", async () => {
    const counter = anchor.web3.Keypair.generate();
    const wallet = new anchor.web3.PublicKey(
      "CDrPrQLhbbWKshxkkHVwgENkQaLxKdXhz3XQTXGaAQjb"
    );

    // Add your test here.
    const destination = await anchor.utils.token.associatedAddress({
      mint: counter.publicKey,
      owner: wallet,
    });

    console.log("--------------------");
    console.log(destination.toBase58());
    console.log("--------------------");
  });
});
