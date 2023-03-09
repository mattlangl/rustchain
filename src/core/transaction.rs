use std::{io::{Write, Read, Cursor}};
use p256::ecdsa::Signature;
use serde::{Serialize, Deserialize};
use crate::{types::hash::Hash, core::encoding::{Encode, Decode, Encoder, Decoder}, crypto::keypair::{PublicKey, PrivateKey}};

use super::hasher::{Hasher, Bytes};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub data: Vec<u8>,
    pub key: Option<PublicKey>,
    pub signature: Option<Signature>,
    pub hash: Option<Hash>,
    pub seen: Option<i64>,
}

impl Bytes for Transaction {
    fn as_bytes(&self) -> Vec<u8> {
        let writer = Cursor::new(vec![]);
        let encoder = Encoder::new(&mut writer);

        encoder.encode(self);
        writer.set_position(0);
        writer.into_inner()
    }
}

impl Transaction {
    pub fn new(data: Vec<u8>) -> Result<Transaction, ()> {
        let mut tx = Transaction {
            data: data,
            key: None,
            signature: None,
            hash: None,
            seen: None,
        };
        Ok(tx)
    }

    pub fn sign(&mut self, private_key: &PrivateKey) -> Result<(), String> {
        self.signature = Some(private_key.sign(&self.data).expect("could not sign"));
        self.key = Some(private_key.generate_public());
        Ok(())
    }
    

    pub fn verify(&self) -> Result<(), p256::ecdsa::Error> {
        assert_eq!(self.signature.is_none(), false);
        assert_eq!(self.key.is_none(), false);
        self.key.as_ref().unwrap().verify(&self.data, self.signature.as_ref().unwrap())
    }

    pub fn hash(&mut self, hasher: Hasher) -> Hash 
    {
        if self.hash.is_none() {
            self.hash = Some(hasher.hash(*self).expect("could not hash"));
        }
        self.hash.unwrap()
    }

    pub fn set_seen(&mut self, seen: i64) {
        self.seen = Some(seen);
    }

    pub fn seen(&self) -> Option<i64> {
        self.seen
    }

}

#[cfg(test)]
mod test {
    use crate::crypto::keypair::PrivateKey;

    use super::Transaction;

    #[test]
    fn test_sign_transaction() {
        let key = PrivateKey::generate_key();
        let mut tx = Transaction {
            data: br#"foo"#.to_vec(),
            key: None,
            signature: None,
            hash: None,
            seen: None,
        };

        assert!(tx.sign(&key).is_ok());
        assert!(tx.key.is_some());
        assert!(tx.signature.is_some());
        // assert!(tx.hash().is_ok());
    }

    #[test]
    fn test_verify_transaction() {
        let key = PrivateKey::generate_key();
        let mut tx = Transaction {
            data: br#"foo"#.to_vec(),
            key: None,
            signature: None,
            hash: None,
            seen: None,
        };

        assert!(tx.sign(&key).is_ok());
        assert!(tx.verify().is_ok());

        let old_key = tx.key;
        tx.key = Some(PrivateKey::generate_key().generate_public());

        assert!(tx.verify().is_err());

        tx.key = old_key;
        tx.data = br#"Hello World!"#.to_vec();

        assert!(tx.verify().is_err());

    }
}