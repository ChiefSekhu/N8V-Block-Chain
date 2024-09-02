use sp_core::sr25519::{Public, Signature};
use schnorrkel::{MiniSecretKey, ExpansionMode};
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
use heavyhash::HeavyHash;

pub struct N8IVMining;

impl N8IVMining {
    pub fn hash_data(data: &[u8]) -> [u8; 32] {
        let mut hasher = HeavyHash::new();
        hasher.update(data);
        hasher.finalize()
    }
    
    pub fn mine_block(data: &[u8]) -> [u8; 32] {
        loop {
            let hash = Self::hash_data(data);
            if Self::is_valid_block_hash(&hash) {
                return hash;
            }
        }
    }

    fn is_valid_block_hash(hash: &[u8; 32]) -> bool {
        hash[0] == 0 && hash[1] == 0 && hash[2] == 0
    }
}
use decentralized_storage::Ipfs;

pub struct N8IVStorage;

impl N8IVStorage {
    pub fn store_data(data: &[u8]) -> Result<String, &'static str> {
        let ipfs = Ipfs::new();
        let cid = ipfs.add(data)?;
        Ok(cid.to_string())
    }

    pub fn retrieve_data(cid: &str) -> Result<Vec<u8>, &'static str> {
        let ipfs = Ipfs::new();
        let data = ipfs.cat(cid)?;
        Ok(data)
    }
}
