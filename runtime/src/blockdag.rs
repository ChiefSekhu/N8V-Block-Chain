use std::collections::HashMap;

#[derive(Clone, Eq, PartialEq, Encode, Decode, Default)]
pub struct Block {
    pub parent_hashes: Vec<[u8; 32]>,
    pub block_hash: [u8; 32],
    pub data: Vec<u8>,
}

decl_storage! {
    trait Store for Module<T: Config> as BlockDAGModule {
        Blocks: map hasher(blake2_128_concat) [u8; 32] => Option<Block>;
    }
}

decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {
        #[weight = 10_000]
        pub fn submit_block(origin, parent_hashes: Vec<[u8; 32]>, data: Vec<u8>, block_hash: [u8; 32]) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            let new_block = Block {
                parent_hashes,
                block_hash,
                data,
            };

            Blocks::insert(new_block.block_hash, new_block);
            Ok(())
        }
    }
}
