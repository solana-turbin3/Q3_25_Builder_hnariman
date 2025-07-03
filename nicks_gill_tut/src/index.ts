import { address, createSignerFromKeyPair, createSolanaClient, createTransaction, generateKeyPairSigner } from "gill";
import { buildCreateTokenTransaction, buildTransferTokensTransaction, getMintSize, TOKEN_PROGRAM_ADDRESS } from "gill/programs/token"
import { getTokenMetadataAddress } from "gill/programs";
import { loadKeypairFromFile } from "gill/node";

const { rpc, rpcSubscriptions, sendAndConfirmTransaction } = createSolanaClient({
  urlOrMoniker: "devnet"
});

(async () => {
  try {
    const { value: latestBlockhash } = await rpc.getLatestBlockhash().send();
    const keys = await loadKeypairFromFile("/home/hnariman/turbin3-wallet.json");
    const feePayer = await createSignerFromKeyPair(keys);
    const mint = await generateKeyPairSigner();

    const create = await buildCreateTokenTransaction({
      feePayer,
      latestBlockhash,
      mint,
      metadata: {
        name: "Gill Token Turbin3 Q3",
        symbol: "GGTQ3",
        isMutable: true,
        uri: "http://turbin3.com"
      },
      decimals: 6,
    });
    const result = await sendAndConfirmTransaction(create);
    console.log({ result });

    const destination = address("E3G8GD4Gt3MNf8sPaWprJUpsY32ciXDjipLKUX3PnMq7");

    const tx = await buildTransferTokensTransaction({
      feePayer,
      latestBlockhash,
      mint,
      authority: feePayer,
      amount: 1,
      destination,
      tokenProgram: TOKEN_PROGRAM_ADDRESS
    });

    const final = sendAndConfirmTransaction(tx);
    console.log({ final });



  } catch (e) { console.error(e) }
})();
