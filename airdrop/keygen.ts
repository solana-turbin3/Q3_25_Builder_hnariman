import {Keypair} from "@solana/web3.js"

const kp = Keypair.generate();
console.log(`You've generated a new Solana wallt: \n\n ${kp.publicKey.toBase58()}`);
console.log(`\n\n [${kp.secretKey}]`);
