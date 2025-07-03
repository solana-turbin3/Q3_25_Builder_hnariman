import wallet from "/home/hnariman/turbin3-wallet.json"; //with {type: "json"};
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createGenericFile, createSignerFromKeypair, signerIdentity, generateSigner, percentAmount } from "@metaplex-foundation/umi"
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys"
import { readFile } from "fs/promises"
import { createNft, mplTokenMetadata } from "@metaplex-foundation/mpl-token-metadata";
import base58 from "bs58";

// Create a devnet connection
const RPC_URL = 'https://devnet.helius-rpc.com/?api-key=71d05d9f-5d94-4548-9137-c6c3d9f69b3e';
// const umi = createUmi('https://api.devnet.solana.com');
const umi = createUmi(RPC_URL);

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

// umi.use(irysUploader({ address: "https://devnet.irys.xyz/" }));
umi.use(irysUploader());
umi.use(signerIdentity(signer));
const mint = generateSigner(umi);

interface IUploadImage {
    path: string,
    fileName: string,
    contentType: string
}

const uploadImage = async ({ path, fileName, contentType }: IUploadImage): string => {
    try {
        //1. Load image
        // const img = await readFile("/home/hnariman/Downloads/1.png");
        // const img = await readFile("/home/hnariman/Pictures/duck1.jpg");
        const img = await readFile(path);
        //
        // //2. Convert image to generic file.
        const genericFile = createGenericFile(img, fileName, { contentType });
        // const generic = createGenericFile(img, "duck1.png", { contentType: "image/png" });

        // //3. Upload image
        const [myUri] = await umi.uploader.upload([genericFile]);
        console.log("Your image URI: ", myUri);
        return myUri
    }
    catch (error) {
        console.log("Oops.. Something went wrong", error);
    }
};

interface IMetadata {
    imageURI: string,
    name: string,
    symbol: string,
    description: string,
};

const createMetadata = async ({ imageURI, name, symbol, description }: IMetadata): string => {
    try {
        // Follow this JSON structure
        // https://docs.metaplex.com/programs/token-metadata/changelog/v1.0#json-structure
        // const p = fs.readFileSync("/home/hnariman/Pictures/1.jpg");
        // const f = createGenericFile(p,"1.jpg");

        // const imagePath = "https://gateway.irys.xyz/E6cGu23XZmfJ5nbdrPnwjcrM5yvehVcbQsGboZDudqQq"
        // const image = "https://devnet.irys.xyz/EsXXhuYi7RajYksKY8M2cgrhj9x2iYMPsT2tCdfq16Da";
        //
        // const imagePath = "https://gateway.irys.xyz/AwsCLUK3B9zs5rN5eN9BScLx7CgRu3tLP5yygDyQepsP"
        const metadata = {
            name,
            symbol,
            description,
            image: imageURI,
            attributes: [
                { trait_type: 'generated', value: 'with excitement!' }
            ],
            properties: {
                files: [
                    {
                        type: "image/png",
                        uri: imageURI,
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
        return myUri;
    }
    catch (error) {
        console.log("Oops.. Something went wrong", error);
    }
};

interface IMintNFT {
    name: string,
    symbol: string,
    metadataURI: string,
}

const mintNFT = async ({ name, symbol, metadataURI }: IMintNFT) => {
    let tx = createNft(umi, {
        mint,
        name,
        symbol,
        uri: metadataURI,
        sellerFeeBasisPoints: percentAmount(1),
        isMutable: true,
        collectionDetails: null
    });
    let result = await tx.sendAndConfirm(umi);
    const signature = base58.encode(result.signature);

    console.log(`Succesfully Minted! Check out your TX here:\nhttps://explorer.solana.com/tx/${signature}?cluster=devnet`)

    console.log("Mint Address: ", mint.publicKey);
};

(async () => {
    try {
        const imageURI = await uploadImage({
            path: "/home/hnariman/Pictures/turbin3-rug-day/duck/2.png",
            fileName: "duck2.png",
            contentType: "image/png"
        });

        const metadataURI = await createMetadata({
            imageURI,
            name: "Turbin3 Duck Two",
            symbol: "TRBND2",
            description: "Duck series continues!"
        });

        await mintNFT({ name: "Turbin3 Duck Two", symbol: "TRBND2", metadataURI });
        return null
    } catch (e) { console.error(e) }
})()

