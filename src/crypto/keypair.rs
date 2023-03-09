use p256::{
    ecdsa::{
        signature::{Signer, Verifier},
        SigningKey, VerifyingKey, Signature,
    },
    pkcs8::EncodePrivateKey,
    PublicKey as P256PublicKey, SecretKey,
    elliptic_curve::rand_core::OsRng
};
use serde::{Serialize, Deserialize};
use sha2::{Digest, Sha256};

use crate::types::address::Address;

#[derive(Debug, PartialEq, Clone)]
pub struct PrivateKey {
    key: String
}

impl PrivateKey {
    pub fn sign(&self, message: &[u8]) -> Result<Signature, String> {
        let signing_key: SigningKey = self.key.parse::<SecretKey>().expect("error").into();
        Ok(signing_key.sign(message))
    }

    pub fn generate_key() -> Self {
        let secret_key = SecretKey::random(&mut OsRng);
        let secret_key_serialized = secret_key
        .to_pkcs8_pem(Default::default())
        .unwrap()
        .to_string();
        PrivateKey{key: secret_key_serialized}
    }

    pub fn generate_public(&self) -> PublicKey {
        let parsed = self.key.parse::<SecretKey>().unwrap().public_key();
        PublicKey{ key: parsed }
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub struct PublicKey {
    key: P256PublicKey
}

impl PublicKey {
    pub fn to_slice(&self) -> Vec<u8> {
        self.key.to_string().into()
    }

    pub fn address(&self) -> Result<Address, String> {
        let mut hasher = Sha256::new();
        hasher.update(&self.to_slice());
        let result = hasher.finalize();
        let address_bytes = &result[result.len() - 20..];
        Address::from_bytes(address_bytes.try_into().unwrap())
    }

    pub fn verify(&self, message: &[u8], signature: &Signature) -> Result<(), p256::ecdsa::Error>
    {
        let verifying_key: VerifyingKey = self.key.into();
        verifying_key.verify(message, signature)
    }

}


#[cfg(test)]
mod test {
    
    use super::*;
    

    #[test]
    fn test_keypair_sign_verify_success() {
        let private = PrivateKey::generate_key();
        let public = private.generate_public();

        let message = "Hello World".as_bytes();

        let signature = private.sign(message);

        assert!(public.verify(message, &signature.unwrap()).is_ok());
    }

    #[test]
    fn test_keypair_sign_verify_failure() {
        let private = PrivateKey::generate_key();
        let public = private.generate_public();

        let message = "Hello World".as_bytes();

        let signature = private.sign(message);

        let other_private = PrivateKey::generate_key();
        let other_public = other_private.generate_public();

        assert!(public.verify("hello".as_bytes(), signature.as_ref().unwrap()).is_err());
        assert!(other_public.verify(message, &signature.unwrap()).is_err());
    }
}