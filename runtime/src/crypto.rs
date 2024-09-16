use sp_core::sr25519::{Public, Signature};
let mini_key: MiniSecretKey = mini_secret_from_entropy(entropy, password);
use pqcrypto_dilithium::dilithium3::*;
use heavyhash::HeavyHash;
use frame_support::{decl_module, decl_storage, dispatch::DispatchResult};
use frame_system::ensure_signed;
use sp_runtime::traits::{Verify, IdentifyAccount};

#[derive(Clone, Eq, PartialEq, Encode, Decode, Default)]
pub struct Transaction {
    pub sender: Public,
    pub receiver: Public,
    pub amount: u64,
    pub signature: Signature,
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
        pub fn submit_transaction(origin, receiver: Public, amount: u64, signature: Signature, dag_block_hash: [u8; 32]) -> DispatchResult {
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
        tx.signature.verify(&tx.sender, &tx.dag_block_hash).is_ok()
    }
}
