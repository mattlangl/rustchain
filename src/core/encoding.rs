use std::io::{Write, Read};
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

pub trait Decode<T> {
    fn decode(&mut self) -> T;
}

impl <'a, R: Read>Decode<Header> for Decoder<'a, R> {
    fn decode(&mut self) -> Header {
        let header: Header = ciborium::de::from_reader(&mut self.reader).unwrap();
        header
    }
}