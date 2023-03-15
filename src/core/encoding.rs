use std::io::{Write, Read};
use serde::Deserialize;

use crate::network::rpc::Message;

use super::{block::{Block, Header}, transaction::Transaction};

pub struct Encoder<'a, W: Write> {
    writer: &'a mut W
}

impl <'a, W: Write>Encoder<'a, W> {
    pub fn new(writer: &'a mut W) -> Self {
        Encoder {
            writer
        }
    }
}

pub trait Encode<T> {
    fn encode(&mut self, obj: &T);
}

impl <'a, W: Write>Encode<Block> for Encoder<'a, W> {
    fn encode(&mut self, obj: &Block) {
        ciborium::ser::into_writer( obj, &mut self.writer);
    }
}

impl <'a, W: Write>Encode<Header> for Encoder<'a, W> {
    fn encode(&mut self, obj: &Header) {
        ciborium::ser::into_writer( obj, &mut self.writer);
    }
}

impl <'a, W: Write>Encode<Transaction> for Encoder<'a, W> {
    fn encode(&mut self, obj: &Transaction) {
        ciborium::ser::into_writer( obj, &mut self.writer);
    }
}

impl <'a, W: Write>Encode<Message> for Encoder<'a, W> {
    fn encode(&mut self, obj: &Message) {
        ciborium::ser::into_writer( obj, &mut self.writer);
    }
}

pub struct Decoder<'a, R: Read> {
    reader: &'a mut R,
}

impl <'a, R: Read>Decoder<'a, R> {
    pub fn new(reader: &'a mut R) -> Self {
        Decoder {
            reader
        }
    }
}

pub trait Decode<'de, T: Deserialize<'de>> {
    fn decode(&mut self) -> T;
}


impl <'de, R: Read, T: Deserialize<'de>>Decode<'de, T> for Decoder<'de, R> {
    fn decode(&mut self) -> T {
        let obj: T = ciborium::de::from_reader(&mut self.reader).unwrap();
        obj
    }
}


// impl <'a, R: Read>Decode<Transaction> for Decoder<'a, R> {
//     fn decode(&mut self) -> Transaction {
//         let transaction = ciborium::de::from_reader(&mut self.reader).unwrap();
//         transaction
//     }
// }