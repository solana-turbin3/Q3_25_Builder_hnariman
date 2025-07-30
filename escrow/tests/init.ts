import * as anchor from "@coral-xyz/anchor"
import {
  createAssociatedTokenAccountIdempotentInstruction as createATA,
  createInitializeMint2Instruction as createMintToInit,
  createMintToInstruction as createMintTo,
  getAssociatedTokenAddressSync,
  getMinimumBalanceForRentExemptMint,
  MINT_SIZE,
  TOKEN_2022_PROGRAM_ID
} from "@solana/spl-token";

import { Keypair, LAMPORTS_PER_SOL, PublicKey, SystemProgram, Transaction } from "@solana/web3.js";
import { Program, BN } from "@coral-xyz/anchor";
import { randomBytes } from "crypto";
import { Escrow } from "../target/types/escrow"

function setup() {
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.getProvider();
  const connection = provider.connection;
  const program = anchor.workspace.Escrow as Program<Escrow>;
  const tokenProgram = TOKEN_2022_PROGRAM_ID;
  return {
    provider,
    connection,
    program,
    tokenProgram
  }
}

describe("esrow-main-test", () => {
  const { provider, program, tokenProgram, connection } = setup()

  const confirm = async (signature: string): Promise<string> => {
    const block = await connection.getLatestBlockhash();
    await connection.confirmTransaction({ signature, ...block });
    return signature
  }

  const log = async (signature: string): Promise<string> => {
    console.log(
      "transaction signature"
      + `https://explorer.solana.com/transaction/${signature}`
      + `?cluster=custom&customUrl=${connection.rpcEndpoint}`
    );
    return signature
  }

  const seed = new BN(randomBytes(8));

  const [maker, taker, mintA, mintB] = Array.from({ length: 4 }, () => Keypair.generate());

  const [makerAtaA, makerAtaB, takerAtaA, takerAtaB] =
    [maker, taker].map((a) =>
      [mintA, mintB].map(m => getAssociatedTokenAddressSync(
        m.publicKey,
        a.publicKey,
        false,
        tokenProgram
      )));

  const escrow = PublicKey.findProgramAddressSync([
    Buffer.from("escrow"),
    maker.publicKey.toBuffer(),
    seed.toArrayLike(Buffer, 'le', 8)
  ], program.programId)[0];

  const vault = getAssociatedTokenAddressSync(mintA.publicKey, escrow, false, tokenProgram);

  const accounts = {
    maker: maker.publicKey,
    taker: taker.publicKey,
    mintA: mintA.publicKey,
    mintB: mintB.publicKey,
    makerAtaA,
    makerAtaB,
    takerAtaA,
    takerAtaB,
    tokenProgram,
    escrow,
    vault
  };

  it("Airdrop and create mints", async () => {

    const lamports = await getMinimumBalanceForRentExemptMint(connection);
    let tx = new Transaction();

    // good example of Solana tx composability
    tx.instructions = [
      // setup airdrop:
      ...[maker, taker].map((account) => SystemProgram.transfer({
        fromPubkey: provider.publicKey,
        toPubkey: account.publicKey,
        lamports: 10 * LAMPORTS_PER_SOL
      })),

      // create account:
      ...[mintA, mintB].map((mint) => SystemProgram.createAccount({
        fromPubkey: provider.publicKey,
        newAccountPubkey: mint.publicKey,
        lamports,
        space: MINT_SIZE,
        programId: tokenProgram
      })),

      // setup mint & ata:
      ...[
        { mint: mintA.publicKey, authority: maker.publicKey, ata: makerAtaA },
        { mint: mintB.publicKey, authority: taker.publicKey, ata: takerAtaB },
      ].flatMap((x) => [
        createMintToInit(x.mint, 6, x.authority, null, tokenProgram),
        createATA(provider.publicKey, x.ata, x.authority, x.mint, tokenProgram),
        createMintTo(x.mint, x.ata, x.authority, 1e9, undefined, tokenProgram),
      ])];
    // run instructions
    await provider.sendAndConfirm(tx, [mintA, mintB, maker, taker]).then(log);

  });

  it("Make Instruction", async () => {
    await program.methods
      .make(seed, new BN(1e9), new BN(1e9))
      .accounts({ ...accounts })
      .signers([maker])
      .rpc()
      .then(confirm)
      .then(log)
  });

  it("Take Instruction", async () => {
    await program.methods
      .take()
      .accounts({ ...accounts })
      .signers([maker])
      .rpc()
      .then(confirm)
      .then(log)
  });


  it("Make & Refund Instruction", async () => {
    await program.methods
      .make(seed, new BN(1e9), new BN(1e9))
      .accounts({ ...accounts })
      .signers([maker])
      .rpc()
      .then(confirm)
      .then(log)

    await program.methods
      .refund()
      .accounts({ ...accounts })
      .signers([maker])
      .rpc()
      .then(confirm)
      .then(log)

  });
})

