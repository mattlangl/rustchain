use byteorder::{WriteBytesExt, LittleEndian, ReadBytesExt};
use rand::{RngCore, thread_rng, Rng};
use std::{fmt, io};
use std::io::{Write, Read};
use std::iter::repeat;

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Hash([u8; 32]);

impl Hash {
    pub fn default() -> Hash {
        let mut hash = [0u8; 32];
        for byte in &mut hash {
            *byte = 0;
        }
        Hash(hash)
    }

    pub fn encode_binary<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        for byte in self.0 {
            writer.write_u8(byte)?
        }
        Ok(())
    }

    pub fn decode_binary<R: Read>(reader: &mut R) -> io::Result<Hash> {
        let mut hash = [0u8; 32];
        for byte in &mut hash {
            *byte = reader.read_u8()?;
        }
        Ok(Hash(hash))
    }

    pub fn is_zero(&self) -> bool {
        self.0.iter().all(|&byte| byte == 0)
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.0.to_vec()
    }

    pub fn from_bytes(b: &[u8]) -> Result<Self, String> {
        println!("conv part");
        if b.len() != 32 {
            println!("wrong len");
            return Err(format!("given bytes with length {} should be 32", b.len()));
        }

        let mut value = [0u8; 32];
        value.copy_from_slice(b);

        Ok(Hash(value))
    }

    pub fn to_string(&self) -> String {
        hex::encode(&self.0)
    }

    pub fn random() -> Self {
        let mut rng = thread_rng();
        let bytes = repeat(())
            .map(|()| rng.gen::<u8>())
            .take(32)
            .collect::<Vec<u8>>();
        Hash::from_bytes(&bytes).unwrap()
    }
}

impl fmt::Debug for Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl fmt::Display for Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
