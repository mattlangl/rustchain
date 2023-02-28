use std::{io::{Write, Read}};

use encode_decode_derive::{Encode, Decode};
use p256::ecdsa::Signature;
use crate::{core::encoding::{Encode, Decode, Encoder, Decoder}, crypto::keypair::{PublicKey, PrivateKey}};

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct Transaction {
    data: Vec<u8>,
    key: PublicKey,
    signature: Option<Signature>,
}

impl Transaction {
    pub fn sign(&mut self, private_key: &PrivateKey) -> Result<(), String> {
        self.signature = Some(private_key.sign(&self.data).expect("could not sign"));
        self.key = private_key.generate_public();
        Ok(())
    }
    

    pub fn verify(&self) -> Result<(), p256::ecdsa::Error> {
        assert_eq!(self.signature.is_none(), false);
        self.key.verify(&self.data, &self.signature.unwrap())
    }
}
