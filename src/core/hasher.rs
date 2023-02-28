use sha2::{Sha256, Digest};

use crate::types::hash::Hash;

use super::{block::{Block, Header}, encoding::{Encode, Encoder, HeaderEncoder}};

pub trait Hasher<T> {
    fn hash(obj: T) -> Result<Hash, String>;
}

struct BlockHasher {
    // encoder: dyn Encoder<Header>
}

impl Hasher<Block> for BlockHasher {
    fn hash(mut obj: Block) -> Result<Hash, String> {
        if obj.hash.is_zero() {
            let mut buf = Vec::new();
            let encoder = HeaderEncoder::new();
            obj.header.encode_binary(&mut buf, encoder).unwrap();    
            let mut hasher = Sha256::new();
            hasher.update(buf);
            let fin = hasher.finalize().to_vec();
            obj.hash = Hash::from_bytes(&fin).expect("failed");
        }

        Ok(obj.hash)
    }
}