import { Connection, Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import { Program, Wallet, AnchorProvider } from "@coral-xyz/anchor";
import { IDL } from "./programs/Turbin3_prereq";
import wallet from "./Turbin3-wallet.json";
import idl from "./TRBZyQHB3m68FGeVsqTK39Wm4xejadjVhP5MAZaKWDM-idl.json";

const MPL_CORE_PROGRAM_ID = new PublicKey(
  "CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d",
);

const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));
const connection = new Connection("https://api.devnet.solana.com", "confirmed");
const provider = new AnchorProvider(connection, new Wallet(keypair), {
  commitment: "confirmed",
});

const PROGRAM_ID = new PublicKey(idl.address);

const program: Program<IDL> = new Program(idl as IDL, provider);

const account_seeds = [Buffer.from("prereqs"), keypair.publicKey.toBuffer()];

const [account_key, bump] = PublicKey.findProgramAddressSync(
  account_seeds,
  PROGRAM_ID,
);

// 5.4. Putting things together

const mintCollection = new PublicKey(
  "5ebsp5RChCGK7ssRZMVMufgVZhd2kFbNaotcZ5UvytN2",
);
//FIXME: use mint account for the new asset? check IDL probably?

const mintTs = Keypair.generate();
// (async () => {
//   try {
//     const txhash = await program.methods
//       .initialize("hnariman")
//       .accountsPartial({
//         user: keypair.publicKey,
//         account: account_key,
//         system_program: SystemProgram.programId,
//       })
//       .signers([keypair])
//       .rpc();

//     console.log(
//       `Success! Check out your TX here: https://explorer.solana.com/tx/${txhash}?cluster=devnet`,
//     );
//   } catch (e) {
//     console.error(e);
//   }
// })();

//execute submit transaction
(async () => {
  try {
    const txhash = await program.methods
      .submitTs()
      .accountsPartial({
        user: keypair.publicKey,
        account: account_key,
        mint: mintTs.publicKey,
        collection: mintCollection,
        mpl_core_program: MPL_CORE_PROGRAM_ID,
        system_program: SystemProgram.programId,
      })
      .signers([mintTs])
      .rpc();

    console.log(
      `Success! Check out your TX here: https://explorer.solana.com/tx/${txhash}?cluster=devnet`,
    );
  } catch (e) {
    console.error(e);
  }
})();
