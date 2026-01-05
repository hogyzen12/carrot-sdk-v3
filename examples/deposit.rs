use carrot_sdk_v3::{CarrotClient, USDC_MINT};
use solana_sdk::signature::{read_keypair_file, Signer};
use std::path::Path;

fn main() -> anyhow::Result<()> {
    println!("🥕 Carrot SDK - Deposit Example\n");

    // Configuration
    let rpc_url = "https://mainnet.helius-rpc.com/?api-key=93812d12-f56f-4624-97c9-9a4d242db974".to_string();
    let keypair_path = "/Users/hogyzen12/.config/solana/RnGrVx38FRDJUyH6pS6QHFHikbTrs9m1csNiJPWHaZA.json";
    let deposit_amount = 1_000_000u64; // 1 USDC (6 decimals)

    // Load keypair
    println!("Loading wallet from: {}", keypair_path);
    let keypair = read_keypair_file(Path::new(keypair_path))
        .map_err(|e| anyhow::anyhow!("Failed to read keypair: {}", e))?;
    
    println!("Wallet address: {}", keypair.pubkey());

    // Create client
    let client = CarrotClient::new(rpc_url);

    // Check current balances
    println!("\nChecking balances...");
    let usdc_balance = client.get_asset_balance(&keypair.pubkey(), &USDC_MINT)?;
    let crt_balance = client.get_crt_balance(&keypair.pubkey())?;
    
    println!("USDC Balance: {} USDC", usdc_balance as f64 / 1_000_000.0);
    println!("CRT Balance: {} CRT", crt_balance as f64 / 1_000_000_000.0);

    // Check if sufficient balance
    if usdc_balance < deposit_amount {
        return Err(anyhow::anyhow!(
            "Insufficient USDC balance. Required: {} USDC, Available: {} USDC",
            deposit_amount as f64 / 1_000_000.0,
            usdc_balance as f64 / 1_000_000.0
        ));
    }

    // Deposit USDC
    println!("\nDepositing {} USDC to Carrot Protocol...", deposit_amount as f64 / 1_000_000.0);
    let signature = client.deposit(&keypair, &USDC_MINT, deposit_amount)?;
    
    println!("✅ Deposit successful!");
    println!("Transaction signature: {}", signature);
    println!("View on Solscan: https://solscan.io/tx/{}", signature);

    // Check new balances
    println!("\nChecking new balances...");
    let new_usdc_balance = client.get_asset_balance(&keypair.pubkey(), &USDC_MINT)?;
    let new_crt_balance = client.get_crt_balance(&keypair.pubkey())?;
    
    println!("New USDC Balance: {} USDC (change: {})", 
        new_usdc_balance as f64 / 1_000_000.0,
        (new_usdc_balance as i64 - usdc_balance as i64) as f64 / 1_000_000.0
    );
    println!("New CRT Balance: {} CRT (change: +{})", 
        new_crt_balance as f64 / 1_000_000_000.0,
        (new_crt_balance - crt_balance) as f64 / 1_000_000_000.0
    );

    Ok(())
}
