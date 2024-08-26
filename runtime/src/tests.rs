#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keypair_generation() {
        let wallet = Wallet::create();
        assert!(wallet
    fn test_keypair_generation() {
        let wallet = Wallet::create();
        assert!(wallet.public_key.len() > 0);
    }

    #[test]
    fn test_sign_and_verify_transaction() {
        let wallet = Wallet::create();
        let data = b"test transaction";
        let signature = wallet.sign_transaction(data);
        assert!(Wallet::verify_transaction(data, &signature, &wallet.public_key));
    }

    #[test]
    fn test_transfer() {
        let wallet_1 = Wallet::create();
        let wallet_2 = Wallet::create();

        // Mint some initial tokens to wallet_1
        N8VTokenModule::mint(wallet_1.account_id.clone(), 1000).expect("Minting failed");

        // Transfer tokens from wallet_1 to wallet_2
        wallet_1.transfer(&wallet_2.account_id, 500).expect("Transfer failed");

        // Check balances
        assert_eq!(wallet_1.balance(), 500);
        assert_eq!(wallet_2.balance(), 500);
    }

    #[test]
    fn test_create_and_transfer_custom_token() {
        let creator_wallet = Wallet::create();
        let receiver_wallet = Wallet::create();
        let token_name = b"MyMemeCoin".to_vec();

        // Create a custom token
        CustomTokensModule::create_token(creator_wallet.account_id.clone(), token_name.clone(), 1000).expect("Token creation failed");

        // Transfer some custom tokens to receiver_wallet
        CustomTokensModule::transfer_token(creator_wallet.account_id.clone(), receiver_wallet.account_id.clone(), token_name.clone(), 500).expect("Token transfer failed");

        // Verify the transfer - Note: You'll need to implement a method to check the custom token balance
        let creator_tokens = CustomTokensModule::tokens(&creator_wallet.account_id);
        let receiver_tokens = CustomTokensModule::tokens(&receiver_wallet.account_id);

        assert_eq!(creator_tokens.iter().find(|&&(ref name, _)| name == &token_name).map(|(_, balance)| *balance).unwrap_or(0), 500);
        assert_eq!(receiver_tokens.iter().find(|&&(ref name, _)| name == &token_name).map(|(_, balance)| *balance).unwrap_or(0), 500);
    }
}
