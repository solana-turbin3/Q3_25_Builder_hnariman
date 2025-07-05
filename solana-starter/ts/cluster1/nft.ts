import wallet from "/home/hnariman/turbin3-wallet.json"; //with {type: "json"};
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createGenericFile, createSignerFromKeypair, signerIdentity, generateSigner, percentAmount } from "@metaplex-foundation/umi"
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys"
import { readFile } from "fs/promises"
import { createNft, mplTokenMetadata } from "@metaplex-foundation/mpl-token-metadata";
import base58 from "bs58";
import { throws } from "assert";
import { programSupportsExtensions } from "@solana/spl-token";

// Create a devnet connection
const RPC_URL = 'https://devnet.helius-rpc.com/?api-key=71d05d9f-5d94-4548-9137-c6c3d9f69b3e';
// const umi = createUmi('https://api.devnet.solana.com');
const umi = createUmi(RPC_URL);

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

// umi.use(irysUploader({ address: "https://devnet.irys.xyz/" }));
umi.use(irysUploader());
umi.use(signerIdentity(signer));
umi.use(mplTokenMetadata());
const mint = generateSigner(umi);

interface IUploadImage {
    path: string,
    fileName: string,
    contentType: string
}

const uploadImage = async ({ path, fileName, contentType }: IUploadImage): Promise<string> => {
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
        throw Error("Oops.. Something went wrong");
    }
};

interface IMetadata {
    imageURI: string,
    name: string,
    symbol: string,
    description: string,
};

const createMetadata = async ({ imageURI, name, symbol, description }: IMetadata): Promise<string> => {
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
        const uri = await umi.uploader.uploadJson(metadata);
        console.log("Your metadata URI: ", uri);
        return uri
    }
    catch (error) {
        console.log("Oops.. Something went wrong", error);
        throw Error("Oops.. Something went wrong");
    }
};

interface IMintNFT {
    name: string,
    symbol: string,
    metadataURI: string,
}

const mintNFT = async ({ name, symbol, metadataURI }: IMintNFT): Promise<unknown> => {
    try {
        console.log('MINTINnnnnnNOW!');
        let tx = createNft(umi, {
            mint,
            name,
            symbol,
            uri: metadataURI,
            sellerFeeBasisPoints: percentAmount(1),
            isMutable: true,
            collectionDetails: null
        });
        console.log('TX_________', { tx });
        console.log(`tx ready, uri:${metadataURI}`);
        let result = await tx.sendAndConfirm(umi);
        console.log({ result });
        const signature = base58.encode(result.signature);

        console.log(`Succesfully Minted! Check out your TX here:\nhttps://explorer.solana.com/tx/${signature}?cluster=devnet`)

        return console.log("Mint Address: ", mint.publicKey);
    } catch (e) {
        console.error(e);
        throw Error("Oops.. Something went wrong");
    }
};

(async () => {
    try {
        const data = {
            path: "/home/hnariman/Pictures/rug/astro/3.png",
            fileName: "punk3.png",
            contentType: "image/png",
            name: "Turbin3 Astro 3",
            symbol: "TRBNA3",
            description: "Astro series continues!"
        }

        const imageURI = await uploadImage({
            path: data.path,
            fileName: data.fileName,
            contentType: data.contentType
        });

        const metadataURI = await createMetadata({
            imageURI,
            name: data.name,
            symbol: data.symbol,
            description: data.description
        });

        await mintNFT({ name: data.name, symbol: data.symbol, metadataURI });
        return null
    } catch (e) {
        console.error(e);
        throw Error("Houston we have a problem!");
    } finally {
        console.log(process.getActiveResourcesInfo());
    }
})()

