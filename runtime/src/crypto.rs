use sp_core::sr25519::{Public};
use pqcrypto_dilithium::dilithium3::*;
use frame_support::{decl_module, decl_storage, dispatch::DispatchResult};
use frame_system::ensure_signed;
use sp_runtime::traits::{IdentifyAccount};

#[derive(Clone, Eq, PartialEq, Encode, Decode, Default)]
pub struct Transaction {
    pub sender: Public, // Public key of the sender
    pub receiver: Public, // Public key of the receiver
    pub amount: u64,
    pub signature: Vec<u8>, // This will now be a vector of bytes
    pub dag_block_hash: [u8; 32],
}

decl_storage! {
    trait Store for Module<T: Config> as N8IVModule {
        Transactions: Vec<Transaction>;
        DAGBlocks: map hasher(blake2_128_concat) [u8; 32] => Option<Transaction>;
    }
}

decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {
        #[weight = 10_000]
        pub fn submit_transaction(origin, receiver: Public, amount: u64, signature: Vec<u8>, dag_block_hash: [u8; 32]) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            let transaction = Transaction {
                sender,
                receiver,
                amount,
                signature,
                dag_block_hash,
            };

            ensure!(Self::verify_signature(&transaction), "Invalid signature");
            DAGBlocks::insert(transaction.dag_block_hash, transaction.clone());
            Transactions::append(transaction);

            Ok(())
        }
    }
}

impl<T: Config> Module<T> {
    fn verify_signature(tx: &Transaction) -> bool {
        let public_key = DilithiumPublicKey::from_bytes(&tx.sender.0).expect("Invalid public key format");
        public_key.verify(&[tx.amount.to_le_bytes(), tx.dag_block_hash].concat(), &tx.signature).is_ok()
    }
}
