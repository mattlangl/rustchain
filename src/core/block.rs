use std::{io::{self, Write, Read, Cursor}};

use chrono::Utc;
use encode_decode_derive::{Encode, Decode};
use p256::ecdsa::Signature;
use sha2::{Sha256, Digest};
use crate::{types::hash::Hash, crypto::keypair::{PublicKey, PrivateKey}};

use super::{transaction::{Transaction}, encoding::{Encoder, Decoder, Encode, Decode, HeaderEncoder}, hasher::{BlockHasher, Hasher}};

#[derive(Debug, PartialEq, Eq, Encode, Decode, Clone, Copy)]
pub struct Header {
    pub version: u32,
    pub data: Hash,
    pub prev_block: Hash,
    pub timestamp: i64,
    pub height: u32,
}

impl Header {
    pub fn as_bytes(&self) -> Vec<u8> {
        let encoder = HeaderEncoder::new();
        let mut writer = Cursor::new(vec![]);

        assert!(encoder.encode(&mut writer, self).is_ok());
        writer.set_position(0);
        writer.into_inner()
    }
}


#[derive(Debug, PartialEq, Decode, Encode, Clone)]
pub struct Block {
    pub header: Header,
    pub transactions: Vec<Transaction>,
    pub signature: Option<Signature>,
    pub validator: Option<PublicKey>,
    pub hash: Option<Hash>, // Cached version of the header hash
    pub prev_hash: Option<Hash>,
}


impl Block {
    pub fn new(header: Header, transactions: Vec<Transaction>) -> Block {
        Block {
            header,
            transactions,
            hash: None,
            signature: None,
            validator: None,
            prev_hash: None,
        }
    }

    pub fn add_transaction(&mut self, t: &Transaction) -> Result<(), ()> {
        self.transactions.push(t.clone());
        Ok(())
    }


    pub fn random_block(h: u32) -> Self {
        let header = Header {
            version: 1,
            data: Hash::random(),
            prev_block: Hash::random(),
            timestamp: Utc::now().timestamp(),
            height: h,
        };

        Block::new(header, vec![])
    }

    pub fn random_block_with_signature(h: u32) -> Self {
        let key = PrivateKey::generate_key();
        let mut b = Self::random_block(h);
        assert!(b.sign(key).is_ok());
    
        b
    }
    

    pub fn hash(&mut self, hasher: Box<dyn Hasher<Header>>) -> Hash {
        if self.hash.is_none() {
            self.hash = Some(hasher.hash(&self.header).expect("could not hash"));
        }
        self.hash.unwrap()
    }

    // pub fn header_data(&self) -> Result<Vec<u8>, io::Error> {
    //     let encoder = HeaderEncoder::new();
    //     let mut vec = Cursor::new(vec![]);
    //     self.header.encode_binary(&mut vec, encoder).expect("couldn't encode header");
    //     Ok(vec.get_ref().to_owned())
    // }

    pub fn sign(&mut self, key: PrivateKey) -> Result<(), String> {
        let header = self.header.as_bytes();
        self.signature = Some(key.sign(&header).expect("could not sign"));
        self.validator = Some(key.generate_public());
        Ok(())
    }

    pub fn verify(&self) -> Result<(), String> {
        if self.signature.is_none() {
            return Err("no signature".to_string());
        }

        let validator = self.validator.as_ref().unwrap();
        let signature = self.signature.unwrap();
        let res = validator.verify(&self.header.as_bytes(), &signature);
        if res.is_err() {
            return Err("Could not verify".to_owned());
        }

        for t in &self.transactions {
            assert!(t.verify().is_ok());
        }
        Ok(())
    }


}


#[cfg(test)]
mod test {
    // use crate::types::hash::Hash;

    // use super::*;
    // use std::io::Cursor;

    // #[test]
    // fn test_header_encode_decode() {
    //     let h = Header {
    //         version: 1,
    //         prev_block: Hash::random(),
    //         timestamp: chrono::Utc::now().timestamp_nanos(),
    //         height: 10,
    //         nonce: 989394,
    //     };

    //     let mut buf = Cursor::new(vec![]);
    //     assert!(h.encode_binary(&mut buf).is_ok());

    //     buf.set_position(0);
    //     let h_decode = Header::decode_binary(&mut buf).unwrap();
    //     assert_eq!(h, h_decode);
    // }

    // #[test]
    // fn test_block_encode_decode() {
    //     let header = Header {
    //         version: 1,
    //         prev_block: Hash::random(),
    //         timestamp: chrono::Utc::now().timestamp_nanos(),
    //         height: 10,
    //         nonce: 989394,
    //     };
    //     let b = Block::new(header, vec![]);

    //     let mut buf = Cursor::new(vec![]);
    //     assert!(b.encode_binary(&mut buf).is_ok());

    //     buf.set_position(0);
    //     let b_decode = Block::decode_binary(&mut buf).unwrap();
    //     println!("third part");
    //     assert_eq!(b, b_decode);
    // }

    // #[test]
    // fn test_block_hash() {
    //     let mut b = Block {
    //         header: Header {
    //             version: 1,
    //             prev_block: Hash::random(),
    //             timestamp: chrono::Utc::now().timestamp_nanos(),
    //             height: 10,
    //             nonce: 0,
    //         },
    //         transactions: vec![],
    //         hash: Hash::default(),
    //     };

    //     let h = b.hash();
    //     assert!(!h.is_zero());
    // }

    

    use crate::{crypto::{keypair::PrivateKey}};

    use super::{Block};

    


    #[test]
    fn test_sign_block() {
        let key = PrivateKey::generate_key();
        let mut b = Block::random_block(0);
        assert!(b.sign(key).is_ok());
        assert!(b.signature.is_some());
    }

    #[test]
    fn test_verify_block() {
        let key = PrivateKey::generate_key();
        let mut b = Block::random_block(0);
        assert!(b.sign(key).is_ok());
        println!("{:?}", b);
        assert!(b.verify().is_ok());

        let other_key = PrivateKey::generate_key();
        let old_validator = b.validator;
        b.validator = Some(other_key.generate_public());
        assert!(b.verify().is_err());
        b.validator = old_validator;
        b.header.height = 100;

        assert!(b.verify().is_err());

    }

}