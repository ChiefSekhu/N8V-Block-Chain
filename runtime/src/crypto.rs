use pqcrypto_dilithium::dilithium2::{keypair, sign, verify};

pub struct Wallet {
    pub public_key: Vec<u8>,
    pub secret_key: Vec<u8>,
    pub account_id: AccountId, // Assuming AccountId is defined somewhere in your runtime
}

impl Wallet {
    pub fn create() -> Self {
        let (pk, sk) = keypair(); // Generate quantum-resistant keypair
        Wallet {
            public_key: pk.to_vec(),
            secret_key: sk.to_vec(),
            account_id: generate_account_id(pk.to_vec()), // Assuming a function to generate AccountId
        }
    }

    pub fn sign_transaction(&self, data: &[u8]) -> Vec<u8> {
        sign(data, &self.secret_key)
    }

    pub fn verify_transaction(data: &[u8], signature: &[u8], public_key: &[u8]) -> bool {
        verify(signature, data, public_key).is_ok()
    }

    pub fn transfer(&self, to: &AccountId, amount: u64) -> dispatch::DispatchResult {
        N8VTokenModule::transfer(self.account_id.clone(), to.clone(), amount)
    }

    pub fn balance(&self) -> u64 {
        N8VTokenModule::balance_of(self.account_id.clone())
    }
}
