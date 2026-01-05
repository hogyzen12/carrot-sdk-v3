use thiserror::Error;

#[derive(Error, Debug)]
pub enum CarrotError {
    #[error("Solana client error: {0}")]
    SolanaClient(#[from] solana_client::client_error::ClientError),
    
    #[error("Solana SDK error: {0}")]
    SolanaSdk(#[from] solana_sdk::program_error::ProgramError),
    
    #[error("Token error: {0}")]
    Token(#[from] spl_token::error::TokenError),
    
    #[error("Borsh deserialization error: {0}")]
    BorshDeserialize(#[from] std::io::Error),
    
    #[error("Invalid vault data")]
    InvalidVaultData,
    
    #[error("Invalid asset: {0}")]
    InvalidAsset(String),
    
    #[error("Insufficient balance: required {required}, available {available}")]
    InsufficientBalance { required: u64, available: u64 },
    
    #[error("Transaction failed: {0}")]
    TransactionFailed(String),
    
    #[error("Account not found: {0}")]
    AccountNotFound(String),
}

pub type Result<T> = std::result::Result<T, CarrotError>;