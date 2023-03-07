use super::block::Block;

pub trait Storage {
    fn put(&mut self, b: &Block) -> Result<(), ()>;
}

pub struct MemoryStore{}

impl MemoryStore {
    pub fn new() -> Self {
        MemoryStore {  }
    }
}

impl Storage for MemoryStore {
    fn put(&mut self, _b: &Block) -> Result<(), ()> {
        Ok(())
    }
}