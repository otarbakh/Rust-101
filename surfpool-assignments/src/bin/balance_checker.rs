use solana_client::rpc_client::RpcClient;
use solana_commitment_config::CommitmentConfig;
use solana_native_token::LAMPORTS_PER_SOL;
use std::env;
use std::str::FromStr;

// ვიყენებთ solana_sdk-ს Pubkey-სთვის
use solana_sdk::pubkey::Pubkey;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("გამოყენება: cargo run --bin balance_checker <ADDRESS>");
        return;
    }

    // 1. ვპარსავთ სტრინგს Pubkey-ში
    let pubkey = Pubkey::from_str(&args[1]).expect("არასწორი მისამართის ფორმატი");
    
    // 2. გადაგვყავს ბაიტების მასივში (ეს არის "უნივერსალური ენა")
    let pubkey_bytes: [u8; 32] = pubkey.to_bytes();
    
    let client = RpcClient::new_with_commitment(
        "http://127.0.0.1:8899".to_string(), 
        CommitmentConfig::confirmed()
    );

    // 3. ვიყენებთ .into()-ს ბაიტების მასივზე. 
    // კომპილატორმა გვითხრა, რომ Address-ს შეუძლია [u8; 32]-ის მიღება.
    match client.get_balance(&pubkey_bytes.into()) {
        Ok(lamports) => {
            println!("Public key : {}", args[1]);
            println!("Balance    : {:.4} SOL ({} lamports)", lamports as f64 / LAMPORTS_PER_SOL as f64, lamports);
        }
        Err(e) => println!("შეცდომა ბალანსის მიღებისას: {}", e),
    }
}
