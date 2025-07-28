use solana_program::system_instruction::transfer;
use solana_sdk::{
    message::Message,
    pubkey::Pubkey,
    signature::{Keypair, Signer, read_keypair_file},
    transaction::Transaction,
};

use solana_client::rpc_client::RpcClient;

use bs58;
use std::io::{self, BufRead};
use std::str::FromStr;

const DEVNET_RPC_URL: &str = "https://api.devnet.solana.com";
const LOCAL_RPC_URL: &str = "http://localhost:8899";

fn keygen() {
    println!("Hello, world!");

    let kp = Keypair::new();
}

#[cfg(test)]
mod tests {
    use std::f64::consts::E;

    use super::*;
    use solana_sdk::{
        self,
        instruction::{AccountMeta, Instruction},
        system_program,
    };

    // #[test]
    // fn keygen() {
    //     let kp = Keypair::new();
    //     println!("A new Solana wallet: {}", kp.pubkey().to_string());
    //     println!("");
    //     println!("To save your wallet, copy and paste the following into a JSON file:");
    //     println!("{:?}", &kp.to_bytes());
    //     println!("{:?}", &kp.to_base58_string());
    // }

    // test cases TODO: cover later with proper tests?
    // [144, 106, 226, 18, 185, 212, 161, 108, 49, 233, 61, 191, 163, 224, 101, 14, 151, 150, 89, 218, 85, 19, 225, 52, 192, 240, 218, 2, 177, 234, 110, 46, 15, 53, 133, 74, 56, 89, 30, 89, 160, 107, 3, 165, 29, 210, 65, 237, 129, 162, 85, 74, 128, 255, 1, 173, 12, 159, 88, 24, 33, 71, 30, 181]
    // "3tU7GZ3gGCPpEJ5dFos6etQJYWHcgu9wqmg6u3Yp3ECGxMS64SrorALUFo2Xpyn9q859bmuTQZiyw3KNAYWayRYY"
    //
    // 35zdT92NzrQ9wjTwohqQMz9apUMzRtVRkWUd4GbPyFMqgJupJheuBhYok6FFoWRukeuARRqbQsrjD58XWEx37hV1
    // [104, 87, 28, 197, 247, 9, 8, 131, 232, 29, 72, 175, 199, 165, 62, 212, 201, 88, 120, 179, 14, 233, 224, 68, 193, 214, 129, 202, 95, 110, 98, 214, 192, 139, 60, 197, 100, 10, 27, 6, 204, 192, 133, 185, 18, 1, 111, 229, 201, 90, 120, 18, 110, 64, 36, 162, 45, 226, 224, 64, 188, 215, 246, 40]
    #[test]
    fn base58_to_wallet() {
        println!("Input your private key as base58 string:");
        let stdin = io::stdin();
        let base58 = stdin.lock().lines().next().unwrap().unwrap();
        println!("Your wallet format is:");
        let wallet = bs58::decode(base58).into_vec().unwrap();
        println!("{:?}", wallet);
    }

    #[test]
    fn wallet_to_base58() {
        println!("Input your private key as a JSON byte array:");
        let stdin = io::stdin();
        let wallet = stdin
            .lock()
            .lines()
            .next()
            .unwrap()
            .unwrap()
            .trim_start_matches('[')
            .trim_end_matches(']')
            .split(',')
            .map(|s| s.trim().parse::<u8>().unwrap())
            .collect::<Vec<u8>>();
        println!("Your Base58 string is:");
        println!("{}", bs58::encode(wallet).into_string());
    }

    // #[test]
    // fn claim_airdrop() {
    //     let kp = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
    //     println!("{:?}", kp.pubkey().to_string());
    //     let client = RpcClient::new(DEVNET_RPC_URL);
    //     match client.request_airdrop(&kp.pubkey(), 2_000_000_000u64) {
    //         Ok(sig) => {
    //             println!("Success! TX:");
    //             println!("https://explorer.solana.com/tx/{}?cluster=devnet", sig);
    //             // println!("https://explorer.solana.com/tx/{}?cluster=localhost", sig);
    //         }
    //         Err(err) => {
    //             println!("Error: {}", err);
    //         }
    //     }
    // }

    // #[test]
    // fn transfer_token_to_turbin3_wallet() {
    //     let kp = read_keypair_file("dev-wallet.json").expect("unable to read wallet file");

    //     let pubkey = kp.pubkey();
    //     let message_bytes = b"I verify my Solana Keypair";

    //     let sig = kp.sign_message(message_bytes);
    //     // TODO: not sure which hash to import
    //     let sig_hashed = solana_sdk::hash::hash(sig.as_ref());

    //     match sig.verify(&pubkey.to_bytes(), &sig_hashed.to_bytes()) {
    //         true => println!("Signature verified successfully!"),
    //         false => println!("Signature verification failed!"),
    //     }

    //     let to = Pubkey::from_str("DxcRNdAVn9aarXvsTTCnJ4RboayLHXqGtGzf7g9NLt5D").unwrap();
    //     let rpc = RpcClient::new(DEVNET_RPC_URL);
    //     let balance = rpc
    //         .get_balance(&kp.pubkey())
    //         .expect("Failed to get balance for the account");

    //     let recent_hash = rpc
    //         .get_latest_blockhash()
    //         .expect("Failed to get recent hash");

    //     let message = Message::new_with_blockhash(
    //         &[transfer(&kp.pubkey(), &to, balance)],
    //         Some(&kp.pubkey()),
    //         &recent_hash,
    //     );

    //     let fee = rpc
    //         .get_fee_for_message(&message)
    //         .expect("Failed to get fee for message");

    //     let transaction = Transaction::new_signed_with_payer(
    //         &[transfer(&kp.pubkey(), &to, balance - fee)],
    //         Some(&kp.pubkey()),
    //         &vec![&kp],
    //         recent_hash,
    //     );

    //     let signature = rpc
    //         .send_and_confirm_transaction(&transaction)
    //         .expect("Failed to send transaction");
    //     // println!("Transaction signature: {}", signature);
    //     println!(
    //         "Transferring entire balance: https://explorer.solana.com/tx/{}?cluster=devnet",
    //         signature
    //     );
    // }
    #[test]
    fn submit_rust() {
        let rpc = RpcClient::new(DEVNET_RPC_URL);
        let signer = read_keypair_file("turbin3-wallet.json").expect("unable to read dev wallet");

        let mint = Keypair::new();
        let turbine3_prereq_program =
            Pubkey::from_str("TRBZyQHB3m68FGeVsqTK39Wm4xejadjVhP5MAZaKWDM").unwrap();
        let collection = Pubkey::from_str("5ebsp5RChCGK7ssRZMVMufgVZhd2kFbNaotcZ5UvytN2").unwrap();

        let mpl_core_program =
            Pubkey::from_str("CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d").unwrap();
        let authority = Pubkey::from_str("5xstXUdRJKxRrqbJuo5SAfKf68y7afoYwTeH1FXbsA3k").unwrap();
        let system_program_id = system_program::id();

        // PDA
        let signer_pub = signer.pubkey();
        let seeds = &[b"prereqs", signer_pub.as_ref()];

        let (prereq_pda, _bump) = Pubkey::find_program_address(seeds, &turbine3_prereq_program);

        // From the IDL, the submit_rs instruction discriminator is:
        let data = vec![77, 124, 82, 163, 21, 133, 181, 206];

        let accounts = vec![
            AccountMeta::new(signer_pub, true),
            AccountMeta::new(prereq_pda, false),
            AccountMeta::new(mint.pubkey(), true),
            AccountMeta::new(collection, false),
            AccountMeta::new_readonly(authority, false),
            AccountMeta::new_readonly(mpl_core_program, false),
            AccountMeta::new_readonly(system_program_id, false),
        ];

        let block_hash = rpc.get_latest_blockhash().expect("unable to get hash");

        let instruction = Instruction {
            program_id: turbine3_prereq_program,
            accounts,
            data,
        };

        let transaction = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&signer_pub),
            &[&signer, &mint],
            block_hash,
        );

        let signature = rpc
            .send_and_confirm_transaction(&transaction)
            .expect("unable to send transaction");

        println!(
            "Success! TX@: https://explorer.solana.com/tx/{}/?cluster=devnet",
            signature
        );
    }
}
