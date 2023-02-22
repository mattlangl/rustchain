use std::{sync::{mpsc::Receiver, Mutex, Arc}, any::Any};

pub type NetAddr = String;

#[derive(Debug, Clone)]
pub struct RPC {
    pub from: NetAddr,
    pub payload: Vec<u8>,
}

pub trait Transport: Send + Sync {
    fn consume(&self) -> Arc<Mutex<Receiver<RPC>>>;
    fn connect(&mut self, transport: &dyn Transport) -> Result<(), String>;
    fn send_message(&self, addr: NetAddr, payload: Vec<u8>) -> Result<(), String>;
    fn addr(&self) -> NetAddr;
    fn as_any(&self) -> &dyn Any;
    
}