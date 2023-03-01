use std::{io::{Write, Read}};

use encode_decode_derive::{Encode, Decode};
use p256::ecdsa::Signature;
use crate::{core::encoding::{Encode, Decode, Encoder, Decoder}, crypto::keypair::{PublicKey, PrivateKey}};

#[derive(Debug, PartialEq, Encode, Decode, Clone)]
pub struct Transaction {
    pub data: Vec<u8>,
    pub key: Option<PublicKey>,
    pub signature: Option<Signature>,
}

impl Transaction {
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
        };

        assert!(tx.sign(&key).is_ok());
        assert!(tx.key.is_some());
        assert!(tx.signature.is_some());
    }

    #[test]
    fn test_verify_transaction() {
        let key = PrivateKey::generate_key();
        let mut tx = Transaction {
            data: br#"foo"#.to_vec(),
            key: None,
            signature: None,
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