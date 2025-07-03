import wallet from "/home/hnariman/turbin3-wallet.json";// with {type: "json"};
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import { createGenericFile, createSignerFromKeypair, signerIdentity } from "@metaplex-foundation/umi"
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys";
import fs, { ReadStream } from 'fs';

// Create a devnet connection
const umi = createUmi('https://api.devnet.solana.com');

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

// umi.use(irysUploader({ address: "https://devnet.irys.xyz/", }));
umi.use(irysUploader());
umi.use(signerIdentity(signer));

(async () => {
    try {
        // Follow this JSON structure
        // https://docs.metaplex.com/programs/token-metadata/changelog/v1.0#json-structure
        // const p = fs.readFileSync("/home/hnariman/Pictures/1.jpg");
        // const f = createGenericFile(p,"1.jpg");

        // const imagePath = "https://gateway.irys.xyz/E6cGu23XZmfJ5nbdrPnwjcrM5yvehVcbQsGboZDudqQq"
        // const image = "https://devnet.irys.xyz/EsXXhuYi7RajYksKY8M2cgrhj9x2iYMPsT2tCdfq16Da";
        //
        const imagePath = "https://gateway.irys.xyz/AwsCLUK3B9zs5rN5eN9BScLx7CgRu3tLP5yygDyQepsP"
        const metadata = {
            name: "Turbin3 Duck One",
            symbol: "TRD1",
            description: "Ducks Everywhere",
            image: imagePath,
            attributes: [
                { trait_type: 'generated', value: 'with excitement!' }
            ],
            properties: {
                files: [
                    {
                        type: "image/png",
                        uri: imagePath,
                    },
                ],
                category: "image"
            },
            creators: [{
                addres: signer.publicKey.toString(),
                share: 100
            }]
        };
        const myUri = await umi.uploader.uploadJson(metadata);
        console.log("Your metadata URI: ", myUri);
    }
    catch (error) {
        console.log("Oops.. Something went wrong", error);
    }
})();
