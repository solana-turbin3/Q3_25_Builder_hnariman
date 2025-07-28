import wallet from "/home/hnariman/turbin3-wallet.json"; //with {type: "json"};
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createGenericFile, createSignerFromKeypair, signerIdentity } from "@metaplex-foundation/umi"
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys"
import { readFile } from "fs/promises"

// Create a devnet connection
const umi = createUmi('https://api.devnet.solana.com');

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

// umi.use(irysUploader({ address: "https://devnet.irys.xyz/" }));
umi.use(irysUploader());
umi.use(signerIdentity(signer));

(async () => {
    try {
        //1. Load image
        // const img = await readFile("/home/hnariman/Downloads/1.png");
        const img = await readFile("/home/hnariman/Pictures/duck1.jpg");
        //
        // //2. Convert image to generic file.
        const genericFile = createGenericFile(img, "random.png", { contentType: "image/png" });
        // const generic = createGenericFile(img, "duck1.png", { contentType: "image/png" });

        // //3. Upload image
        const [myUri] = await umi.uploader.upload([genericFile]);
        console.log("Your image URI: ", myUri);
    }
    catch (error) {
        console.log("Oops.. Something went wrong", error);
    }
})();
