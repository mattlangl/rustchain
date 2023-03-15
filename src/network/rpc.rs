use std::io::{self, Read};
use std::fmt;

use serde::{Deserialize, Serialize};
use log::debug;

use crate::core::encoding::{Encoder, Decoder, Decode};
use crate::core::hasher::Bytes;
use crate::core::encoding::Encode;
use crate::core::transaction::Transaction;
use super::transport::NetAddr;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum MessageType {
    Tx = 0x1,
    Block,
}
pub struct RPC  {
    pub from: NetAddr,
    pub payload: Box<dyn Read>,
}



#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Message {
    pub header: MessageType,
    pub data: Vec<u8>,
}

impl Bytes for Message {
    fn as_bytes(&self) -> Vec<u8> {
        let mut writer = vec![];
        let mut encoder = Encoder::new(&mut writer);

        encoder.encode(self);
        writer
    }
}

impl Message {
    pub fn new(t: MessageType, data: Vec<u8>) -> Self {
        Self {
            header: t,
            data: data,
        }
    }
}

#[derive(Debug)]
pub enum Decoded {
    Tx(Transaction),
}

#[derive(Debug)]
pub struct DecodedMessage {
    pub from: NetAddr,
    pub data: Decoded,
}

impl DecodedMessage {
    pub fn new(from: NetAddr, data: Decoded) -> Self {
        Self {
            from: from,
            data: data,
        }
    }
}

#[derive(Debug)]
pub struct MessageDecodeError {
    from: NetAddr,
    error: String,
}

impl fmt::Display for MessageDecodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "failed to decode message from {}: {}", self.from, self.error)
    }
}

impl std::error::Error for MessageDecodeError {}

pub type RPCDecodeFunc = fn(RPC) -> Result<DecodedMessage, MessageDecodeError>;

pub fn default_rpc_decode_func(mut rpc: RPC) -> Result<DecodedMessage, MessageDecodeError> {
    let mut decoder = Decoder::new(&mut rpc.payload);


    let msg: Message = decoder.decode();

    debug!(
        "new incoming message from {}: type={:?}",
        rpc.from, msg.header
    );

    match msg.header {
        MessageType::Tx => {
            let tx : Transaction = decoder.decode();
            Ok(DecodedMessage::new(rpc.from, Decoded::Tx(tx)))
        }
        _ => Err(MessageDecodeError {
            from: rpc.from,
            error: format!("invalid message header {:?}", msg.header),
        }),
    }
}

pub trait RPCProcessor {
    fn process_message(&mut self, dm: &DecodedMessage) -> io::Result<()>;
}
