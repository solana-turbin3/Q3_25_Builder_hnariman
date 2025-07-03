// Irys 
import { Uploader } from "@irys/upload";
import { EclipseEth } from "@irys/upload-solana";

import { loadKeypairFromFile } from "gill/node";

const f = "/home/hnariman/Pictures/turbin3-rug-day/duck/1.png";

const getIrysUploader = async (keys:CryptoKeyPair) => {
    // RPC URLs change often. Use a current one from https://chainlist.org/
    const rpcURL = "https://ethereum-rpc.publicnode.com";
    const irysUploader = await Uploader(EclipseEth)
        .withWallet(keys.publicKey)
        .withRpc(rpcURL)
        .devnet();

    return irysUploader;
};

(async () => {
    try {
        const keys = await loadKeypairFromFile("/home/hnariman/turbin3-wallet.json");
        console.log({ keys });
        const uploader = await getIrysUploader(keys);
        const tags = [{ name: "Content-Type", value: "image/png" }];

        const res = await uploader.uploadFile(f,{tags});
        console.log(`File uploaded ==> https://gateway.irys.xyz/${res.id}`);

    }
    catch (e) { console.error(e) }


})()
