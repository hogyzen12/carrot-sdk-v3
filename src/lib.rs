use borsh::{BorshDeserialize, BorshSerialize};
use solana_sdk::pubkey::Pubkey;

pub mod accounts;
pub mod error;
pub mod instructions;
pub mod client;

pub use error::CarrotError;
pub use client::{deposit_usdc, withdraw_crt, CarrotClient};

/// Carrot Protocol Program ID
pub const CARROT_PROGRAM_ID: Pubkey = solana_sdk::pubkey!("CarrotwivhMpDnm27EHmRLeQ683Z1PufuqEmBZvD282s");

/// CRT Token Mint
pub const CRT_MINT: Pubkey = solana_sdk::pubkey!("CRTx1JouZhzSU6XytsE42UQraoGqiHgxabocVfARTy2s");

/// USDC Token Mint
pub const USDC_MINT: Pubkey = solana_sdk::pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");

/// USDT Token Mint
pub const USDT_MINT: Pubkey = solana_sdk::pubkey!("Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB");

/// pyUSD Token Mint
pub const PYUSD_MINT: Pubkey = solana_sdk::pubkey!("2b1kV6DkPAnxd5ixfnxCpjxmKwqjjaYmCZfHsFu24GXo");

/// Log Program ID (used by Carrot for logging)
pub const LOG_PROGRAM_ID: Pubkey = solana_sdk::pubkey!("7Mc3vSdRWoThArpni6t5W4XjvQf4BuMny1uC8b6VBn48");

/// Main vault address on mainnet
pub const VAULT_ADDRESS: Pubkey = solana_sdk::pubkey!("FfCRL34rkJiMiX5emNDrYp3MdWH2mES3FvDQyFppqgpJ");

/// Fee structure within vault
#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
pub struct Fee {
    pub redemption_fee_bps: u16,
    pub redemption_fee_accumulated: u64,
    pub management_fee_bps: u16,
    pub management_fee_last_update: i64,
    pub management_fee_accumulated: u64,
    pub performance_fee_bps: u16,
}

/// Asset data structure within vault
#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
pub struct Asset {
    /// Asset ID
    pub asset_id: u16,
    /// Token mint address
    pub mint: Pubkey,
    /// Decimals for the token
    pub decimals: u8,
    /// Associated token account for vault
    pub ata: Pubkey,
    /// Price oracle address
    pub oracle: Pubkey,
}

/// Strategy record within vault
#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
pub struct StrategyRecord {
    pub strategy_id: u16,
    pub asset_id: u16,
    pub balance: u64,
    pub net_earnings: i64,
}

/// Vault account structure (matches on-chain IDL)
#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
pub struct Vault {
    /// Vault authority
    pub authority: Pubkey,
    /// Share token mint (CRT)
    pub shares: Pubkey,
    /// Fee configuration
    pub fee: Fee,
    /// Whether vault is paused
    pub paused: bool,
    /// Current asset index
    pub asset_index: u16,
    /// Current strategy index
    pub strategy_index: u16,
    /// List of assets (USDC, USDT, pyUSD)
    pub assets: Vec<Asset>,
    /// List of strategy records
    pub strategies: Vec<StrategyRecord>,
}

impl Vault {
    /// Get all asset ATAs and oracles as remaining accounts
    pub fn get_remaining_accounts(&self) -> Vec<Pubkey> {
        self.assets
            .iter()
            .flat_map(|asset| vec![asset.ata, asset.oracle])
            .collect()
    }
}

/// Arguments for issue (deposit) instruction
#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
pub struct IssueArgs {
    pub amount: u64,
}

/// Arguments for redeem (withdrawal) instruction
#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
pub struct RedeemArgs {
    pub amount: u64,
}