# Carrot SDK

A Rust SDK for interacting with the Carrot Protocol on Solana. This SDK provides a simple interface for depositing stablecoins (USDC, USDT, pyUSD) and withdrawing CRT tokens.

## Features

- Deposit stablecoins (USDC, USDT, pyUSD) to receive CRT tokens
- Withdraw CRT tokens to receive stablecoins
- Automatic ATA (Associated Token Account) creation
- Simple, Anchor-free implementation using core Solana libraries

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
carrot-sdk = "0.1.0"
```

## Usage

### Basic Example - Deposit USDC

```rust
use carrot_sdk::{CarrotClient, USDC_MINT};
use solana_sdk::signature::Keypair;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client with RPC URL
    let rpc_url = "https://api.mainnet-beta.solana.com".to_string();
    let client = CarrotClient::new(rpc_url);
    
    // Load your keypair
    let keypair = Keypair::new(); // or read_keypair_file()
    
    // Deposit 1 USDC (6 decimals)
    let amount = 1_000_000u64;
    let signature = client.deposit(&keypair, &USDC_MINT, amount)?;
    
    println!("Deposit successful! Signature: {}", signature);
    Ok(())
}
```

### Withdraw CRT

```rust
use carrot_sdk::{CarrotClient, USDC_MINT};
use solana_sdk::signature::Keypair;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rpc_url = "https://api.mainnet-beta.solana.com".to_string();
    let client = CarrotClient::new(rpc_url);
    let keypair = Keypair::new();
    
    // Withdraw 0.5 CRT (9 decimals)
    let amount = 500_000_000u64;
    let signature = client.withdraw(&keypair, &USDC_MINT, amount)?;
    
    println!("Withdrawal successful! Signature: {}", signature);
    Ok(())
}
```

### Check Balances

```rust
use carrot_sdk::{CarrotClient, USDC_MINT};
use solana_sdk::pubkey::Pubkey;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rpc_url = "https://api.mainnet-beta.solana.com".to_string();
    let client = CarrotClient::new(rpc_url);
    let user = Pubkey::new_unique();
    
    // Check USDC balance
    let usdc_balance = client.get_asset_balance(&user, &USDC_MINT)?;
    println!("USDC: {} USDC", usdc_balance as f64 / 1_000_000.0);
    
    // Check CRT balance
    let crt_balance = client.get_crt_balance(&user)?;
    println!("CRT: {} CRT", crt_balance as f64 / 1_000_000_000.0);
    
    Ok(())
}
```

### Convenience Functions

The SDK provides convenience functions for common operations:

```rust
use carrot_sdk::{deposit_usdc, withdraw_crt};
use solana_sdk::signature::Keypair;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rpc_url = "https://api.mainnet-beta.solana.com".to_string();
    let keypair = Keypair::new();
    
    // Deposit 1 USDC
    let sig = deposit_usdc(rpc_url.clone(), &keypair, 1_000_000)?;
    println!("Deposited: {}", sig);
    
    // Withdraw 0.5 CRT
    let sig = withdraw_crt(rpc_url, &keypair, 500_000_000)?;
    println!("Withdrew: {}", sig);
    
    Ok(())
}
```

## Running Examples

The SDK includes example programs that demonstrate real-world usage:

### Deposit Example

Deposits 1 USDC to the Carrot Protocol:

```bash
cargo run --example deposit
```

### Withdraw Example

Withdraws half of your CRT balance:

```bash
cargo run --example withdraw
```

## Important Constants

The SDK provides pre-configured constants for mainnet:

```rust
use carrot_sdk::*;

// Program and addresses
CARROT_PROGRAM_ID  // CarrotwivhMpDnm27EHmRLeQ683Z1PufuqEmBZvD282s
VAULT_ADDRESS      // FfCRL34rkJiMiX5emNDrYp3MdWH2mES3FvDQyFppqgpJ

// Token mints
CRT_MINT          // CRTx1JouZhzSU6XytsE42UQraoGqiHgxabocVfARTy2s
USDC_MINT         // EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v
USDT_MINT         // Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB
PYUSD_MINT        // 2b1kV6DkPAnxd5ixfnxCpjxmKwqjjaYmCZfHsFu24GXo
```

## Token Decimals

- USDC: 6 decimals (1 USDC = 1,000,000)
- USDT: 6 decimals (1 USDT = 1,000,000)
- pyUSD: 6 decimals (1 pyUSD = 1,000,000)
- CRT: 9 decimals (1 CRT = 1,000,000,000)

## API Reference

### CarrotClient

Main client for interacting with Carrot Protocol.

#### Methods

- `new(rpc_url: String) -> Self` - Create a new client
- `deposit(user: &Keypair, asset_mint: &Pubkey, amount: u64) -> Result<Signature>` - Deposit assets
- `withdraw(user: &Keypair, asset_mint: &Pubkey, amount: u64) -> Result<Signature>` - Withdraw assets
- `get_asset_balance(user: &Pubkey, asset_mint: &Pubkey) -> Result<u64>` - Check asset balance
- `get_crt_balance(user: &Pubkey) -> Result<u64>` - Check CRT balance
- `fetch_vault() -> Result<Vault>` - Fetch vault data from blockchain

## Error Handling

The SDK uses a custom error type that wraps common Solana and token errors:

```rust
use carrot_sdk::CarrotError;

match client.deposit(&keypair, &USDC_MINT, amount) {
    Ok(sig) => println!("Success: {}", sig),
    Err(CarrotError::InsufficientBalance { required, available }) => {
        println!("Need {} but only have {}", required, available);
    }
    Err(e) => println!("Error: {}", e),
}
```

## Testing

Run the test suite:

```bash
cargo test
```

The tests verify:
- Vault PDA derivation
- ATA address derivation
- Program ID and constant correctness

## Dependencies

This SDK uses the following Solana crates:

- `solana-sdk = "2.3.1"`
- `solana-client = "2.3.2"`
- `spl-token = "8.0.0"`
- `spl-associated-token-account = "7.0.0"`
- `borsh = "1.5.7"`

## License

MIT

## Links

- [Carrot Protocol App](https://deficarrot.com/)
- [Documentation](https://docs.deficarrot.com/)
- [Program on Solscan](https://solscan.io/token/CarrotwivhMpDnm27EHmRLeQ683Z1PufuqEmBZvD282s)
- [CRT Token](https://solscan.io/token/CRTx1JouZhzSU6XytsE42UQraoGqiHgxabocVfARTy2s)

## Support

For issues and questions, please open an issue on GitHub.