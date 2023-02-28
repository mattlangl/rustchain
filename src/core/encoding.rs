use std::io::{Write, Read, Result};

use super::block::Header;

pub trait Encoder<T: ?Sized> {
    fn encode<W: Write>(&self, writer: W, t: &T) -> Result<()>;
}

pub trait Decoder<T: ?Sized> {
    fn decode<R: Read>(&self, reader: R) -> Result<Box<T>>;
}

pub trait Encode {
    fn encode_binary<W: Write, E: Encoder<Self>>(&self, writer: &mut W, encoder: E) -> Result<()>;
}

pub trait Decode {
    fn decode_binary<R: Read, D: Decoder<Self>>(writer: &mut R, decoder: D) -> Result<Box<Self>>;
}

pub struct HeaderEncoder {}

impl HeaderEncoder {
    pub fn new() -> Self {
        HeaderEncoder {}
    }
}

impl Encoder<Header> for HeaderEncoder {


    fn encode<W: Write>(&self, _writer: W, _h: &Header) -> Result<()> {
        Ok(())
    }
}

pub struct HeaderDecoder {}

impl HeaderDecoder {
    pub fn new() -> Self {
        HeaderDecoder {}
    }
}

impl Decoder<Header> for HeaderDecoder {
    fn decode<R: Read>(&self, _reader: R) -> Result<Box<Header>> {
        todo!()
    }
}