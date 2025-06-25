import {
  LAMPORTS_PER_SOL,
  Transaction,
  SystemProgram,
  Connection,
  Keypair,
  sendAndConfirmTransaction,
  PublicKey,
} from "@solana/web3.js";

import wallet from "./dev-wallet.json";

const from = Keypair.fromSecretKey(new Uint8Array(wallet));
const to = new PublicKey("DxcRNdAVn9aarXvsTTCnJ4RboayLHXqGtGzf7g9NLt5D");

const connection = new Connection("https://api.devnet.solana.com");
(async () => {
  try {
    const transaction = new Transaction().add(
      SystemProgram.transfer({
        fromPubkey: from.publicKey,
        toPubkey: to,
        lamports: LAMPORTS_PER_SOL / 100,
      }),
    );

    transaction.recentBlockhash = (
      await connection.getLatestBlockhash("confirmed")
    ).blockhash;

    transaction.feePayer = from.publicKey;

    const signature = await sendAndConfirmTransaction(connection, transaction, [
      from,
    ]);
    console.log(
      `Success! Check out your TX here: https://explorer.solana.com/tx/${signature}?cluster=devnet`,
    );
  } catch (e) {
    console.error(e);
  }
})();
