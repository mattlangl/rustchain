use std::{io::{self, Write, Read, ErrorKind, Error}};

#[derive(Debug, PartialEq)]
pub struct Transaction {
    data: Vec<u8>
}

impl Transaction {
    pub fn encode_binary<W: Write>(&self, mut writer: W) -> io::Result<()> {
        Ok(())
    }

    pub fn decode_binary<R: Read>(mut reader: R) -> io::Result<Transaction> {

        Err(Error::new(ErrorKind::Other, "oh no!"))
    }
}
