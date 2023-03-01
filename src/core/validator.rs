use super::{block::Block, blockchain::Blockchain};



pub trait Validator {
    fn validate_block(&self, bc: &Blockchain, b: &Block) -> Result<(), ()>;
}

pub struct BlockValidator {}

impl <'a>BlockValidator {
    pub fn new_validator() -> Self {
        BlockValidator {}
    }
}

impl <'a>Validator for BlockValidator {
    fn validate_block(&self, bc: &Blockchain, b: &Block) -> Result<(), ()> {
        assert!(bc.has_block(b.header.height).is_err());
        assert!(b.verify().is_ok());

        Ok(())
    }
}