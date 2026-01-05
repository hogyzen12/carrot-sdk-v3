#[cfg(test)]
mod tests {
    use carrot_sdk_v3::accounts::*;
    use carrot_sdk_v3::*;
    use solana_sdk::pubkey::Pubkey;
    use std::str::FromStr;

    #[test]
    fn test_vault_address_derivation() {
        let (vault, _bump) = derive_vault_address(&CRT_MINT);
        assert_eq!(vault, VAULT_ADDRESS);
    }

    #[test]
    fn test_user_ata_derivation() {
        let user = Pubkey::from_str("RnGrVx38FRDJUyH6pS6QHFHikbTrs9m1csNiJPWHaZA").unwrap();
        
        // Test CRT ATA derivation (from transaction logs)
        let crt_ata = get_user_crt_ata(&user);
        let expected_crt_ata = Pubkey::from_str("DQQ7otzQpMZmZE4RAdicJnAQbacFAwoxz4XzhUtPK7u9").unwrap();
        assert_eq!(crt_ata, expected_crt_ata);
        
        // Test USDC ATA derivation (from transaction logs)
        let usdc_ata = get_user_asset_ata(&user, &USDC_MINT);
        let expected_usdc_ata = Pubkey::from_str("EWuWVR2hBWiNwStMNpBLDimQJXb9wJTtNjbo3mHWuw9U").unwrap();
        assert_eq!(usdc_ata, expected_usdc_ata);
    }

    #[test]
    fn test_vault_ata_derivation() {
        // Test vault's USDC ATA derivation (from transaction logs)
        let vault_usdc_ata = get_vault_asset_ata(&USDC_MINT);
        let expected = Pubkey::from_str("Gfedc4JEmMahEMBJXcXfLHWgNs9d7UzLPq1tkba5S11U").unwrap();
        assert_eq!(vault_usdc_ata, expected);
    }

    #[test]
    fn test_constants() {
        // Verify program IDs and addresses
        assert_eq!(
            CARROT_PROGRAM_ID,
            Pubkey::from_str("CarrotwivhMpDnm27EHmRLeQ683Z1PufuqEmBZvD282s").unwrap()
        );
        
        assert_eq!(
            CRT_MINT,
            Pubkey::from_str("CRTx1JouZhzSU6XytsE42UQraoGqiHgxabocVfARTy2s").unwrap()
        );
        
        assert_eq!(
            USDC_MINT,
            Pubkey::from_str("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v").unwrap()
        );
        
        assert_eq!(
            VAULT_ADDRESS,
            Pubkey::from_str("FfCRL34rkJiMiX5emNDrYp3MdWH2mES3FvDQyFppqgpJ").unwrap()
        );
    }
}
