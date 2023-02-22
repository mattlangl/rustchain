use std::io::{self, Write, Read};
use byteorder::{LittleEndian, WriteBytesExt, ReadBytesExt};
use sha2::{Sha256, Digest};
use crate::types::hash::Hash;

use super::transaction::Transaction;

#[derive(Debug, PartialEq, Eq)]
pub struct Header {
    pub version: u32,
    pub prev_block: Hash,
    pub timestamp: i64,
    pub height: u32,
    pub nonce: u64,
}

impl Header {
    pub fn encode_binary<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_u32::<LittleEndian>(self.version)?;
        self.prev_block.encode_binary(writer)?;
        writer.write_i64::<LittleEndian>(self.timestamp)?;
        writer.write_u32::<LittleEndian>(self.height)?;
        writer.write_u64::<LittleEndian>(self.nonce)?;
        Ok(())
    }

    pub fn decode_binary<R: Read>(reader: &mut R) -> io::Result<Header> {
        let version = reader.read_u32::<LittleEndian>()?;
        let prev_block = Hash::decode_binary(reader)?;
        let timestamp = reader.read_i64::<LittleEndian>()?;
        let height = reader.read_u32::<LittleEndian>()?;

        let nonce = reader.read_u64::<LittleEndian>()?;
        Ok(Header {
            version,
            prev_block,
            timestamp,
            height,
            nonce,
        })
    }
    
}

#[derive(Debug, PartialEq)]
pub struct Block {
    pub header: Header,
    pub transactions: Vec<Transaction>,
    pub hash: Hash, // Cached version of the header hash
}


impl Block {
    pub fn hash(&mut self) -> Hash {
        let mut buf = Vec::new();
        self.header.encode_binary(&mut buf).unwrap();

        if self.hash.is_zero() {
            let mut hasher = Sha256::new();
            hasher.update(buf);
            let fin = hasher.finalize().to_vec();
            self.hash = Hash::from_bytes(&fin).expect("failed");
        }

        self.hash
    }

    pub fn encode_binary<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        println!("encode part");
        self.header.encode_binary(writer)?;

        for tx in self.transactions.iter() {
            println!("encode t");
            tx.encode_binary(&mut *writer)?;
        }

        Ok(())
    }

    pub fn decode_binary<R: Read>(r: &mut R) -> io::Result<Block> {
        let header = Header::decode_binary(r)?;

        let mut transactions = Vec::new();

        loop {
            match Transaction::decode_binary(&mut *r) {
                Ok(tx) => {transactions.push(tx) }
                Err(_) => { break }

            }
        }

        let mut buf = Vec::new();
        header.encode_binary(&mut buf).unwrap();

        let mut hasher = Sha256::new();
        hasher.update(buf);
        let fin = hasher.finalize().to_vec();
        let hash = Hash::from_bytes(&fin).expect("failed");

        Ok(Block {
            header,
            transactions,
            hash
        })
    }
}


#[cfg(test)]
mod test {
    use crate::types::hash::Hash;

    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_header_encode_decode() {
        let h = Header {
            version: 1,
            prev_block: Hash::random(),
            timestamp: chrono::Utc::now().timestamp_nanos(),
            height: 10,
            nonce: 989394,
        };

        let mut buf = Cursor::new(vec![]);
        assert!(h.encode_binary(&mut buf).is_ok());

        buf.set_position(0);
        let h_decode = Header::decode_binary(&mut buf).unwrap();
        assert_eq!(h, h_decode);
    }

    #[test]
    fn test_block_encode_decode() {
        let b = Block {
            header: Header {
                version: 1,
                prev_block: Hash::random(),
                timestamp: chrono::Utc::now().timestamp_nanos(),
                height: 10,
                nonce: 989394,
            },
            transactions: vec![],
            hash: Hash::random(),
        };

        let mut buf = Cursor::new(vec![]);
        assert!(b.encode_binary(&mut buf).is_ok());

        buf.set_position(0);
        let b_decode = Block::decode_binary(&mut buf).unwrap();
        println!("third part");
        assert_eq!(b, b_decode);
    }

    #[test]
    fn test_block_hash() {
        let mut b = Block {
            header: Header {
                version: 1,
                prev_block: Hash::random(),
                timestamp: chrono::Utc::now().timestamp_nanos(),
                height: 10,
                nonce: 0,
            },
            transactions: vec![],
            hash: Hash::default(),
        };

        let h = b.hash();
        assert!(!h.is_zero());
    }

}