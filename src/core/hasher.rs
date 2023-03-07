use sha2::{Sha256, Digest};

use crate::types::hash::Hash;

use super::{block::{Block, Header}, encoding::{Encode, HeaderEncoder}, transaction::Transaction};

pub trait Hasher<T> {
    fn hash(&self, obj: &T) -> Result<Hash, String>;
}

pub struct BlockHasher {
    // encoder: dyn Encoder<Header>
}

impl BlockHasher {
    pub fn new() -> BlockHasher {
        BlockHasher {}
    }
}

impl Hasher<Header> for BlockHasher {
    fn hash(&self, obj: &Header) -> Result<Hash, String> {
        let mut hasher = Sha256::new();
        hasher.update(obj.as_bytes());
        let h = hasher.finalize();
        Hash::from_bytes(&h)
    }
}

pub struct TxHasher {}

impl TxHasher {
    pub fn new() -> TxHasher {
        TxHasher {}
    }
}

impl Hasher<Transaction> for TxHasher {
    fn hash(&self, obj: &Transaction) -> Result<Hash, String> {
        let mut hasher = Sha256::new();
        hasher.update(obj.data.clone());
        let h = hasher.finalize();
        Hash::from_bytes(&h)
    }
}