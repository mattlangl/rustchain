use sha2::{Sha256, Digest};

use crate::types::hash::Hash;


pub trait Bytes {
    fn as_bytes(&self) -> Vec<u8>;
}

pub struct Hasher {

}

impl Hasher {
    pub fn new() -> Hasher {
        Hasher {}
    }
    pub fn hash<B>(&self, obj: &B) -> Result<Hash, String>
    where B: Bytes
     {
        let mut hasher = Sha256::new();
        hasher.update(obj.as_bytes());
        let h = hasher.finalize();
        Hash::from_bytes(&h)
    }
}