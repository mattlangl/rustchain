

use std::sync::{RwLock, Arc};

use crate::core::hasher::BlockHasher;

use super::{storage::{Storage, MemoryStore}, block::{Header, Block}, validator::{Validator, BlockValidator}};

pub struct Blockchain {
    data: Arc<RwLock<BlockchainData>>
}

pub struct BlockchainData {
    store: Box<dyn Storage>,
    headers: Vec<Header>,
    validator:Box<dyn Validator>,
}

impl Blockchain {
    pub fn new(genesis: &mut Block) -> Result<Blockchain, ()> {
            let mut blockchain = Blockchain{
                data: Arc::new(RwLock::new(BlockchainData { 
                store: Box::new(MemoryStore::new()), 
                headers: vec![], 
                validator: Box::new(BlockValidator::new_validator()),
                }))
            };
            // blockchain.set_validator(validator);
            assert!(blockchain.add_block_without_validation(genesis).is_ok());
            Ok(blockchain)
        
    }

    pub fn set_validator(&mut self, v: Box<dyn Validator>) {
        let mut bc = self.data.write().unwrap();
        bc.validator = v
    }

    pub fn add_block(&mut self, b: &mut Block) -> Result<(), ()> {
        let bc = self.data.read().unwrap();
        assert!(bc.validator.as_ref().validate_block(self, b).is_ok());
        std::mem::drop(bc);
        self.add_block_without_validation(b)
    }

    pub fn get_header(&self, h: u32) -> Header {
        assert!(self.height() <= h);
        let bc = self.data.read().unwrap();
        let header = bc.headers.get(h as usize).cloned();
        header.unwrap()
    }

    pub fn has_block(&self, h: u32) -> Result<(), ()> {
        let bc = self.data.read().unwrap();
        if h <= self.height() {
            return Ok(());
        }
        Err(())
    }

    pub fn height(&self) -> u32 {
        let bc = self.data.read().unwrap();
        bc.headers.len() as u32 - 1
    }

    pub fn add_block_without_validation(&mut self, b: &mut Block) -> Result<(), ()> {
        let mut bc = self.data.write().unwrap();
        let height = b.header.height;
        log::info!("Adding block - height: {}, hash: {}", height, b.hash(Box::new(BlockHasher::new())));

        bc.headers.push(b.header);
        bc.store.put(b)
    }
 }


#[cfg(test)]
mod test {
    use crate::core::block::Block;

    use super::Blockchain;

    fn new_blockchain_with_genesis() -> Blockchain {
        let bc = Blockchain::new(&mut Block::random_block(0));
        assert!(bc.is_ok());
        bc.unwrap()
    }

    #[test]
    fn test_add_block() {
        let mut bc = new_blockchain_with_genesis();

        let len = 1000;
        for i in 1..len+1 {
            let mut rand = Block::random_block_with_signature(i);
            assert!(bc.add_block(&mut rand).is_ok());
        }
    }
 }