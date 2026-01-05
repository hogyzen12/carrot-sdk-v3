use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};
use solana_sdk_ids::system_program;
use crate::{
    accounts::{get_user_asset_ata, get_user_crt_ata, get_vault_asset_ata, get_token_program_id},
    error::Result,
    IssueArgs, RedeemArgs, CARROT_PROGRAM_ID, CRT_MINT, LOG_PROGRAM_ID, VAULT_ADDRESS,
};

/// Instruction discriminators for Carrot Protocol
/// These are derived from the instruction name using anchor's discriminator algorithm
/// Calculated as: sha256("global:issue")[..8] and sha256("global:redeem")[..8]
const ISSUE_DISCRIMINATOR: [u8; 8] = [190, 1, 98, 214, 81, 99, 222, 247];
const REDEEM_DISCRIMINATOR: [u8; 8] = [184, 12, 86, 149, 70, 196, 97, 225];

/// Build the issue (deposit) instruction
/// This deposits asset tokens (USDC, USDT, pyUSD) and mints CRT shares
pub fn build_issue_instruction(
    user: &Pubkey,
    asset_mint: &Pubkey,
    amount: u64,
    remaining_accounts: Vec<Pubkey>,
) -> Result<Instruction> {
    let user_shares_ata = get_user_crt_ata(user);
    let user_asset_ata = get_user_asset_ata(user, asset_mint);
    let vault_asset_ata = get_vault_asset_ata(asset_mint);

    let args = IssueArgs { amount };
    
    // Serialize instruction data: discriminator + args
    let mut data = ISSUE_DISCRIMINATOR.to_vec();
    data.extend_from_slice(&borsh::to_vec(&args)?);

    // Get correct token program for asset (Token-2022 for pyUSD, Token for USDC/USDT)
    let asset_token_program = get_token_program_id(asset_mint);
    
    let mut accounts = vec![
        AccountMeta::new(VAULT_ADDRESS, false),
        AccountMeta::new(CRT_MINT, false),
        AccountMeta::new(user_shares_ata, false),
        AccountMeta::new_readonly(*asset_mint, false),
        AccountMeta::new(vault_asset_ata, false),
        AccountMeta::new(user_asset_ata, false),
        AccountMeta::new(*user, true), // signer
        AccountMeta::new_readonly(system_program::id(), false),
        AccountMeta::new_readonly(asset_token_program, false), // asset token program (Token or Token-2022)
        AccountMeta::new_readonly(spl_token_2022_interface::id(), false), // shares token program (Token-2022)
        AccountMeta::new_readonly(LOG_PROGRAM_ID, false),
    ];

    // Add remaining accounts (vault asset ATAs and oracles for each asset)
    for account in remaining_accounts {
        accounts.push(AccountMeta::new(account, false));
    }

    Ok(Instruction {
        program_id: CARROT_PROGRAM_ID,
        accounts,
        data,
    })
}

/// Build the redeem (withdrawal) instruction
/// This burns CRT shares and returns asset tokens
pub fn build_redeem_instruction(
    user: &Pubkey,
    asset_mint: &Pubkey,
    amount: u64,
    remaining_accounts: Vec<Pubkey>,
) -> Result<Instruction> {
    let user_shares_ata = get_user_crt_ata(user);
    let user_asset_ata = get_user_asset_ata(user, asset_mint);
    let vault_asset_ata = get_vault_asset_ata(asset_mint);

    let args = RedeemArgs { amount };
    
    // Serialize instruction data: discriminator + args
    let mut data = REDEEM_DISCRIMINATOR.to_vec();
    data.extend_from_slice(&borsh::to_vec(&args)?);

    // Get correct token program for asset (Token-2022 for pyUSD, Token for USDC/USDT)
    let asset_token_program = get_token_program_id(asset_mint);
    
    let mut accounts = vec![
        AccountMeta::new(VAULT_ADDRESS, false),
        AccountMeta::new(CRT_MINT, false),
        AccountMeta::new(user_shares_ata, false),
        AccountMeta::new_readonly(*asset_mint, false),
        AccountMeta::new(vault_asset_ata, false),
        AccountMeta::new(user_asset_ata, false),
        AccountMeta::new(*user, true), // signer
        AccountMeta::new_readonly(system_program::id(), false),
        AccountMeta::new_readonly(asset_token_program, false), // asset token program (Token or Token-2022)
        AccountMeta::new_readonly(spl_token_2022_interface::id(), false), // shares token program (Token-2022)
        AccountMeta::new_readonly(LOG_PROGRAM_ID, false),
    ];

    // Add remaining accounts (vault asset ATAs and oracles for each asset)
    for account in remaining_accounts {
        accounts.push(AccountMeta::new(account, false));
    }

    Ok(Instruction {
        program_id: CARROT_PROGRAM_ID,
        accounts,
        data,
    })
}
