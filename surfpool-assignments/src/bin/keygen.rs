use solana_keypair::Keypair;
use solana_signer::Signer;
use std::fs::File;
use std::io::{Write, Read};

fn main() {
    let original_keypair = Keypair::new();
    println!("Step 1: Generated a new keypair\n   Public key: {}", original_keypair.pubkey());

    let secret_bytes = original_keypair.to_bytes();
    let json_data = serde_json::to_string(&secret_bytes.to_vec()).unwrap();
    let mut file = File::create("my-keypair.json").unwrap();
    file.write_all(json_data.as_bytes()).unwrap();
    println!("\nStep 2: Saved keypair to 'my-keypair.json'");

    let mut file = File::open("my-keypair.json").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let decoded_bytes: Vec<u8> = serde_json::from_str(&contents).unwrap();
    
    let loaded_keypair = <Keypair as TryFrom<&[u8]>>::try_from(&decoded_bytes[..]).unwrap();
    println!("\nStep 3: Loaded keypair from file\n   Public key: {}", loaded_keypair.pubkey());

    println!("\nStep 4: Comparing original and loaded keypairs");
    println!("   Public keys match: {}", original_keypair.pubkey() == loaded_keypair.pubkey());
    println!("\n✅ Success!");
}
