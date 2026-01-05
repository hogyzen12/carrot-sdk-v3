use solana_sdk::pubkey::Pubkey;
use spl_associated_token_account::{get_associated_token_address, get_associated_token_address_with_program_id};
use crate::{CARROT_PROGRAM_ID, CRT_MINT, VAULT_ADDRESS, PYUSD_MINT};

/// Derive the vault PDA address
/// Seeds: ["vault", shares_mint]
pub fn derive_vault_address(shares_mint: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[b"vault", shares_mint.as_ref()],
        &CARROT_PROGRAM_ID,
    )
}

/// Determine if a mint uses Token-2022 program
/// Currently only pyUSD uses Token-2022, USDC and USDT use standard Token program
pub fn is_token_2022_mint(mint: &Pubkey) -> bool {
    mint == &PYUSD_MINT
}

/// Get the token program ID for a given mint
/// Returns Token-2022 for pyUSD and CRT, standard Token for USDC/USDT
pub fn get_token_program_id(mint: &Pubkey) -> Pubkey {
    if is_token_2022_mint(mint) || mint == &CRT_MINT {
        spl_token_2022_interface::id()
    } else {
        spl_token::id()
    }
}

/// Get the associated token account address for a wallet and mint
pub fn get_ata_address(wallet: &Pubkey, mint: &Pubkey) -> Pubkey {
    get_associated_token_address(wallet, mint)
}

/// Get user's CRT token account address (uses Token-2022 program)
pub fn get_user_crt_ata(user: &Pubkey) -> Pubkey {
    get_associated_token_address_with_program_id(user, &CRT_MINT, &spl_token_2022_interface::id())
}

/// Get user's asset token account address (USDC, USDT, pyUSD)
/// Uses Token-2022 program for pyUSD, standard Token program for USDC/USDT
pub fn get_user_asset_ata(user: &Pubkey, asset_mint: &Pubkey) -> Pubkey {
    let token_program = get_token_program_id(asset_mint);
    get_associated_token_address_with_program_id(user, asset_mint, &token_program)
}

/// Get vault's asset token account address
/// Uses Token-2022 program for pyUSD, standard Token program for USDC/USDT
pub fn get_vault_asset_ata(asset_mint: &Pubkey) -> Pubkey {
    let token_program = get_token_program_id(asset_mint);
    get_associated_token_address_with_program_id(&VAULT_ADDRESS, asset_mint, &token_program)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_derive_vault_address() {
        let (vault, _bump) = derive_vault_address(&CRT_MINT);
        // Should derive to the known mainnet vault address
        assert_eq!(vault, VAULT_ADDRESS);
    }

    #[test]
    fn test_ata_derivation() {
        let user = Pubkey::from_str("RnGrVx38FRDJUyH6pS6QHFHikbTrs9m1csNiJPWHaZA").unwrap();
        let crt_ata = get_user_crt_ata(&user);
        
        // This should match the ATA from the transaction logs
        let expected = Pubkey::from_str("DQQ7otzQpMZmZE4RAdicJnAQbacFAwoxz4XzhUtPK7u9").unwrap();
        assert_eq!(crt_ata, expected);
    }
}
