use solana_client::rpc_client::RpcClient;
use solana_commitment_config::CommitmentConfig;
use solana_keypair::Keypair;
use solana_signer::Signer;
use solana_native_token::LAMPORTS_PER_SOL;

fn main() {
    let client = RpcClient::new_with_commitment("http://127.0.0.1:8899".to_string(), CommitmentConfig::confirmed());
    let wallet = Keypair::new();
    println!("Wallet: {}", wallet.pubkey());
    println!("Before: 0.0000 SOL");

    let sig = client.request_airdrop(&wallet.pubkey(), 2 * LAMPORTS_PER_SOL).unwrap();
    loop {
        if client.confirm_transaction(&sig).unwrap() { break; }
    }
    let balance = client.get_balance(&wallet.pubkey()).unwrap();
    println!("After:  {:.4} SOL", balance as f64 / LAMPORTS_PER_SOL as f64);
}
