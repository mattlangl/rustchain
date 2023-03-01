use std::{pin::Pin, sync::Arc, cell::RefCell};

use super::{storage::{Storage, MemoryStore}, block::{Header, Block}, validator::{Validator, self, BlockValidator}};

pub struct Blockchain {
    store: Box<dyn Storage>,
    headers: Vec<Header>,
    validator:Box<dyn Validator>,
}

impl Blockchain {
    pub fn new(genesis: &Block) -> Result<Blockchain, ()> {
            let mut blockchain = Blockchain { 
                store: Box::new(MemoryStore::new()), 
                headers: vec![], 
                validator: Box::new(BlockValidator::new_validator()),
            };
            // blockchain.set_validator(validator);
            assert!(blockchain.add_block_without_validation(genesis).is_ok());
            Ok(blockchain)
        
    }

    pub fn set_validator(&mut self, v: Box<dyn Validator>) {
        self.validator = v
    }

    pub fn add_block(&mut self, b: &Block) -> Result<(), ()> {
        assert!(self.validator.as_ref().validate_block(self, b).is_ok());
        self.add_block_without_validation(b)
    }

    pub fn has_block(&self, h: u32) -> Result<(), ()> {
        if h <= self.height() {
            return Ok(());
        }
        Err(())
    }

    pub fn height(&self) -> u32 {
        self.headers.len() as u32 - 1
    }

    pub fn add_block_without_validation(&mut self, b: &Block) -> Result<(), ()> {
        self.headers.push(b.header.clone());
        self.store.put(b)
    }
 }


#[cfg(test)]
mod test {
    use crate::core::block::Block;

    use super::Blockchain;

    fn newBlockchainWithGenesis() -> Blockchain {
        let bc = Blockchain::new(&Block::random_block(0));
        assert!(bc.is_ok());
        bc.unwrap()
    }

    #[test]
    fn test_add_block() {
        let mut bc = newBlockchainWithGenesis();

        let len = 1000;
        for i in 1..len+1 {
            let rand = Block::random_block_with_signature(i);
            assert!(bc.add_block(&rand).is_ok());
        }
    }
 }