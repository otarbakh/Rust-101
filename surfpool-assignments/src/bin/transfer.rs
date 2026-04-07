use solana_client::rpc_client::RpcClient;
use solana_commitment_config::CommitmentConfig;
use solana_native_token::LAMPORTS_PER_SOL;

// ვიყენებთ იმავე იმპორტებს, რაც balance_checker-ში ამუშავდა
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_instruction,
    transaction::Transaction,
};

fn main() {
    let rpc_url = "http://127.0.0.1:8899".to_string();
    let client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

    // 1. შევქმნათ ორი Keypair
    let sender = Keypair::new();
    let receiver = Keypair::new();

    println!("Sender:   {}", sender.pubkey());
    println!("Receiver: {}", receiver.pubkey());

    // 2. Airdrop 2 SOL გამგზავნზე
    println!("\nRequesting 2 SOL airdrop...");
    let airdrop_sig = client.request_airdrop(
        &sender.pubkey().to_bytes().into(), 
        2 * LAMPORTS_PER_SOL
    ).expect("Airdrop failed");
    
    // დაველოდოთ დადასტურებას
    while !client.confirm_transaction(&airdrop_sig).expect("Confirmation error") {}
    println!("Airdrop confirmed!");

    // 3. გადარიცხვა 0.5 SOL
    let transfer_amount = LAMPORTS_PER_SOL / 2;
    println!("\nTransferring 0.5 SOL from sender to receiver...");

    let recent_blockhash = client.get_latest_blockhash().expect("Failed to get blockhash");

    // ვქმნით ინსტრუქციას ბაიტების მასივის გამოყენებით
    let transfer_instruction = system_instruction::transfer(
        &sender.pubkey().to_bytes().into(),
        &receiver.pubkey().to_bytes().into(),
        transfer_amount,
    );

    // ვქმნით ტრანზაქციას
    // აქ Signer-ის პრობლემა რომ არ იყოს, პირდაპირ გადავცემთ რეფერენსს
    let mut transaction = Transaction::new_with_payer(
        &[transfer_instruction],
        Some(&sender.pubkey().to_bytes().into()),
    );

    // ხელმოწერა ხდება ცალკე, რომ ვერსიების კონფლიქტი ავიცილოთ
    transaction.sign(&[&sender], recent_blockhash);

    // ვაგზავნით ტრანზაქციას
    let transaction_sig = client.send_and_confirm_transaction(&transaction).expect("Transfer failed");
    println!("Transfer completed! Signature: {}", transaction_sig);

    // 4. ბალანსების შემოწმება
    let sender_balance = client.get_balance(&sender.pubkey().to_bytes().into()).unwrap();
    let receiver_balance = client.get_balance(&receiver.pubkey().to_bytes().into()).unwrap();

    println!("\nFinal Balances:");
    println!("Sender:   {:.4} SOL", sender_balance as f64 / LAMPORTS_PER_SOL as f64);
    println!("Receiver: {:.4} SOL", receiver_balance as f64 / LAMPORTS_PER_SOL as f64);
}
