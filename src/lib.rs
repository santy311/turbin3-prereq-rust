mod programs;

#[cfg(test)]
mod tests {
    use {
        crate::programs::turbin3_prereq::{CompleteArgs, Turbin3PrereqProgram},
        bs58,
        solana_client::rpc_client::RpcClient,
        solana_sdk::{
            message::Message,
            native_token::LAMPORTS_PER_SOL,
            pubkey::Pubkey,
            signature::{read_keypair_file, Keypair, Signer},
            system_instruction::transfer,
            system_program,
            transaction::Transaction,
        },
        std::{
            io::{self, BufRead},
            str::FromStr,
        },
    };

    const RPC_URL: &str = "https://api.devnet.solana.com";

    #[test]
    fn keygen() {
        let kp = Keypair::new();
        println!(
            "You've generated a new Solana wallet: {}\n",
            kp.pubkey().to_string()
        );
        println!("To save your wallet, copy and paste the following into a JSON file:");
        println!("{:?}", kp.to_bytes());
    }

    #[test]
    fn base58_to_wallet() {
        println!("Input your private key as base58:");
        let stdin = io::stdin();
        let base58 = stdin.lock().lines().next().unwrap().unwrap();
        println!("Your wallet file is:");
        let wallet = bs58::decode(base58).into_vec().unwrap();
        println!("{:?}", wallet);
    }

    #[test]
    fn wallet_to_base58() {
        println!("Input your private key as a wallet file byte array:");
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

        let base58 = bs58::encode(wallet).into_string();
        println!("Your private key is:\n{:?}", base58);
    }

    #[test]
    fn airdrop() {
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
        let client = RpcClient::new(RPC_URL);

        match client.request_airdrop(&keypair.pubkey(), LAMPORTS_PER_SOL * 2) {
            Ok(sig) => {
                println!("Success! Check out your TX here:");
                println!(
                    "https://explorer.solana.com/tx/{}?cluster=devnet",
                    sig.to_string()
                );
            }
            Err(e) => println!("Oops, something went wrong: {}", e.to_string()),
        };
    }

    #[test]
    fn transfer_sol() {
        let from_keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
        let to_pubkey = Pubkey::from_str("GN94ThZm9mghXtJxEvDaydK4h8CNkE5JanfDM3Lm9Gdh").unwrap();

        let client = RpcClient::new(RPC_URL);

        // Transfer 0.1 SOL to the Turbin3 wallet
        let instruction = transfer(&from_keypair.pubkey(), &to_pubkey, LAMPORTS_PER_SOL / 10);

        let recent_blockhash = client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");

        let transaction = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&from_keypair.pubkey()),
            &[&from_keypair],
            recent_blockhash,
        );

        match client.send_and_confirm_transaction(&transaction) {
            Ok(sig) => {
                println!("Success! Check out your TX here:");
                println!(
                    "https://explorer.solana.com/tx/{}?cluster=devnet",
                    sig.to_string()
                );
            }
            Err(e) => println!("Error: {}", e.to_string()),
        };
    }

    #[test]
    fn empty_wallet() {
        let from_keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
        let to_pubkey = Pubkey::from_str("GN94ThZm9mghXtJxEvDaydK4h8CNkE5JanfDM3Lm9Gdh").unwrap();

        let rpc_client = RpcClient::new(RPC_URL);

        let balance = rpc_client
            .get_balance(&from_keypair.pubkey())
            .expect("Failed to get balance");

        let recent_blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");

        let mut instruction = transfer(&from_keypair.pubkey(), &to_pubkey, balance);

        let message = Message::new_with_blockhash(
            &[instruction],
            Some(&from_keypair.pubkey()),
            &recent_blockhash,
        );

        let fee = rpc_client
            .get_fee_for_message(&message)
            .expect("Failed to get fee calculator");

        println!("Fee: {}", fee);

        instruction = transfer(&from_keypair.pubkey(), &to_pubkey, balance - fee);

        let transaction = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&from_keypair.pubkey()),
            &[&from_keypair],
            recent_blockhash,
        );

        match rpc_client.send_and_confirm_transaction(&transaction) {
            Ok(sig) => {
                println!("Success! Check out your TX here:");
                println!(
                    "https://explorer.solana.com/tx/{}?cluster=devnet",
                    sig.to_string()
                );
            }
            Err(e) => println!("Error: {}", e.to_string()),
        };
    }

    #[test]
    fn complete_prereq() {
        let rpc_client = RpcClient::new(RPC_URL);
        let signer = read_keypair_file("Turbin3-wallet.json").expect("Couldn't find wallet file");

        let prereq = Turbin3PrereqProgram::derive_program_address(&[
            b"prereq",
            signer.pubkey().to_bytes().as_ref(),
        ]);

        let args = CompleteArgs {
            github: b"santy311".to_vec(),
        };

        let blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");

        let transaction = Turbin3PrereqProgram::complete(
            &[&signer.pubkey(), &prereq, &system_program::id()],
            &args,
            Some(&signer.pubkey()),
            &[&signer],
            blockhash,
        );

        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");

        println!("Success! Check out your TX here:");
        println!(
            "https://explorer.solana.com/tx/{}?cluster=devnet",
            signature.to_string()
        );
    }
}
