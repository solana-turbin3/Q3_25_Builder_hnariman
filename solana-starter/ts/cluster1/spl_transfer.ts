import { Commitment, Connection, Keypair, LAMPORTS_PER_SOL, PublicKey, sendAndConfirmTransaction, SystemProgram, Transaction } from "@solana/web3.js"
import wallet from "/home/hnariman/turbin3-wallet.json";
import { getOrCreateAssociatedTokenAccount, transfer } from "@solana/spl-token";

// We're going to import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

// Mint address
const mint = new PublicKey("BoR8jeiykAx5xFVCJbwJMhDPuywZx6yrUeCpjUc18EsR");

// Recipient address
const to = new PublicKey("E3G8GD4Gt3MNf8sPaWprJUpsY32ciXDjipLKUX3PnMq7");

const sendSOL = async () => {
    try {
        // Get the token account of the fromWallet address, and if it does not exist, create it
        const from = await getOrCreateAssociatedTokenAccount(
            connection,
            keypair,
            mint,
            keypair.publicKey
        );

        // Get the token account of the toWallet address, and if it does not exist, create it
        const toWallet = await getOrCreateAssociatedTokenAccount(
            connection,
            keypair,
            mint,
            to
        );
        // Transfer the new token to the "toTokenAccount" we just created
        const transaction = new Transaction();

        transaction.add(
            SystemProgram.transfer({
                fromPubkey: keypair.publicKey,
                toPubkey: to,
                lamports: LAMPORTS_PER_SOL / 100
            })
        )

        const signature = sendAndConfirmTransaction(
            connection,
            transaction,
            [keypair],
        );

        console.log(`Transaction success: ${JSON.stringify(signature)}`)
    } catch (e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
};


const sendCustom = async () => {
    try {
        const fromATA = await getOrCreateAssociatedTokenAccount(
            connection,
            keypair,
            mint,
            keypair.publicKey
        )

        const toAta = await getOrCreateAssociatedTokenAccount(
            connection,
            to,
            mint,
            keypair.publicKey
        );

        const tx = await transfer(
            connection,
            keypair,
            fromATA.address,
            toAta.address,
            keypair,
            1
        );

        console.log(`we've got your transaction: ${JSON.stringify(tx)}`);
    } catch (e) { console.error(e) }
}
sendCustom();

// sendSOL();
