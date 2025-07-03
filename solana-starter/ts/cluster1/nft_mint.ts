import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createSignerFromKeypair, signerIdentity, generateSigner, percentAmount } from "@metaplex-foundation/umi"
import { createNft, mplTokenMetadata } from "@metaplex-foundation/mpl-token-metadata";

import wallet from "/home/hnariman/turbin3-wallet.json";// with {type: "json"};
import base58 from "bs58";
import { trace } from "console";

// const metadataURI = "https://gateway.irys.xyz/2iWw37ZfzCB1wgYHgHFyuH7xjhXA3hkyVzbr99r2TZYT";
const metadataURI = "https://gateway.irys.xyz/HGCZB6EovMtH6auXpV6hd7KmVBu92fNTSAjXS9geSY4z"

// const RPC_ENDPOINT = "https://api.devnet.solana.com";
const RPC_ENDPOINT = "https://devnet.helius-rpc.com/?api-key=71d05d9f-5d94-4548-9137-c6c3d9f69b3e"
const umi = createUmi(RPC_ENDPOINT);

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const myKeypairSigner = createSignerFromKeypair(umi, keypair);
umi.use(signerIdentity(myKeypairSigner));
umi.use(mplTokenMetadata());

const mint = generateSigner(umi);

(async () => {
    let tx = createNft(umi, {
        mint,//: mint.publicKey,
        name: "Tubin3 Duck One",
        symbol: "TRD1",
        uri: metadataURI,
        sellerFeeBasisPoints: percentAmount(1),
        isMutable: true,
        collectionDetails: null
    });
    let result = await tx.sendAndConfirm(umi);
    const signature = base58.encode(result.signature);

    console.log(`Succesfully Minted! Check out your TX here:\nhttps://explorer.solana.com/tx/${signature}?cluster=devnet`)

    console.log("Mint Address: ", mint.publicKey);
})();
