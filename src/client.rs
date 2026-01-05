use borsh::BorshDeserialize;
use solana_client::rpc_client::RpcClient;
use solana_commitment_config::CommitmentConfig;
use solana_sdk::{
    instruction::Instruction,
    pubkey::Pubkey,
    signature::{Keypair, Signature},
    signer::Signer,
    transaction::Transaction,
};
use spl_associated_token_account::instruction::create_associated_token_account_idempotent;
use spl_token_2022_interface;

use crate::{
    accounts::{get_token_program_id, get_user_asset_ata, get_user_crt_ata},
    error::{CarrotError, Result},
    instructions::{build_issue_instruction, build_redeem_instruction},
    Vault, VAULT_ADDRESS,
};

pub struct CarrotClient {
    rpc_client: RpcClient,
}

impl CarrotClient {
    /// Create a new Carrot client with the given RPC URL
    pub fn new(rpc_url: String) -> Self {
        let rpc_client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());
        Self { rpc_client }
    }

    /// Fetch and deserialize vault data from the blockchain
    pub fn fetch_vault(&self) -> Result<Vault> {
        let account = self
            .rpc_client
            .get_account(&VAULT_ADDRESS)
            .map_err(|_| CarrotError::AccountNotFound("Vault account not found".to_string()))?;

        // Account data starts with 8-byte discriminator (Anchor), skip it
        let data = &account.data[8..];
        
        Vault::try_from_slice(data).map_err(|e| {
            eprintln!("Failed to deserialize vault data: {:?}", e);
            CarrotError::InvalidVaultData
        })
    }

    /// Get remaining accounts (asset ATAs and oracles) from vault data
    pub fn get_remaining_accounts(&self) -> Result<Vec<Pubkey>> {
        let vault = self.fetch_vault()?;
        Ok(vault.get_remaining_accounts())
    }

    /// Deposit asset tokens (USDC, USDT, pyUSD) and receive CRT shares
    pub fn deposit(
        &self,
        user: &Keypair,
        asset_mint: &Pubkey,
        amount: u64,
    ) -> Result<Signature> {
        let user_pubkey = user.pubkey();
        
        // Get remaining accounts from vault
        let remaining_accounts = self.get_remaining_accounts()?;

        // Build instructions
        let mut instructions = Vec::new();

        // Create ATA for CRT if needed (idempotent)
        let create_crt_ata_ix = create_associated_token_account_idempotent(
            &user_pubkey,
            &user_pubkey,
            &crate::CRT_MINT,
            &spl_token_2022_interface::id(),
        );
        instructions.push(create_crt_ata_ix);

        // Build issue instruction
        let issue_ix = build_issue_instruction(&user_pubkey, asset_mint, amount, remaining_accounts)?;
        instructions.push(issue_ix);

        // Create and send transaction
        self.send_transaction(&instructions, user)
    }

    /// Withdraw CRT shares and receive asset tokens
    pub fn withdraw(
        &self,
        user: &Keypair,
        asset_mint: &Pubkey,
        amount: u64,
    ) -> Result<Signature> {
        let user_pubkey = user.pubkey();
        
        // Get remaining accounts from vault
        let remaining_accounts = self.get_remaining_accounts()?;

        // Build instructions
        let mut instructions = Vec::new();

        // Create ATA for asset if needed (idempotent)
        let asset_token_program = get_token_program_id(asset_mint);
        let create_asset_ata_ix = create_associated_token_account_idempotent(
            &user_pubkey,
            &user_pubkey,
            asset_mint,
            &asset_token_program,
        );
        instructions.push(create_asset_ata_ix);

        // Build redeem instruction
        let redeem_ix = build_redeem_instruction(&user_pubkey, asset_mint, amount, remaining_accounts)?;
        instructions.push(redeem_ix);

        // Create and send transaction
        self.send_transaction(&instructions, user)
    }

    /// Send a transaction with the given instructions
    fn send_transaction(&self, instructions: &[Instruction], signer: &Keypair) -> Result<Signature> {
        let recent_blockhash = self.rpc_client.get_latest_blockhash()?;
        
        let transaction = Transaction::new_signed_with_payer(
            instructions,
            Some(&signer.pubkey()),
            &[signer],
            recent_blockhash,
        );

        let signature = self
            .rpc_client
            .send_and_confirm_transaction(&transaction)
            .map_err(|e| CarrotError::TransactionFailed(e.to_string()))?;

        Ok(signature)
    }

    /// Check user's asset token balance
    pub fn get_asset_balance(&self, user: &Pubkey, asset_mint: &Pubkey) -> Result<u64> {
        let ata = get_user_asset_ata(user, asset_mint);
        
        match self.rpc_client.get_token_account_balance(&ata) {
            Ok(balance) => Ok(balance.amount.parse().unwrap_or(0)),
            Err(_) => Ok(0), // Account doesn't exist yet
        }
    }

    /// Check user's CRT token balance
    pub fn get_crt_balance(&self, user: &Pubkey) -> Result<u64> {
        let ata = get_user_crt_ata(user);
        
        match self.rpc_client.get_token_account_balance(&ata) {
            Ok(balance) => Ok(balance.amount.parse().unwrap_or(0)),
            Err(_) => Ok(0), // Account doesn't exist yet
        }
    }
}

/// Convenience function to deposit USDC
pub fn deposit_usdc(rpc_url: String, user: &Keypair, amount_usdc: u64) -> Result<Signature> {
    let client = CarrotClient::new(rpc_url);
    client.deposit(user, &crate::USDC_MINT, amount_usdc)
}

/// Convenience function to withdraw CRT for USDC
pub fn withdraw_crt(rpc_url: String, user: &Keypair, amount_crt: u64) -> Result<Signature> {
    let client = CarrotClient::new(rpc_url);
    client.withdraw(user, &crate::USDC_MINT, amount_crt)
}
