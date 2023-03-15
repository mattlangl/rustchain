use std::{sync::{mpsc::Receiver, Mutex, Arc}, any::Any};

use super::local_transport::LocalTransport;

pub type NetAddr = String;

#[derive(Debug, Clone)]
pub struct RPC {
    pub from: NetAddr,
    pub payload: Vec<u8>,
}

pub trait Transport: Send + Sync {
    fn consume(&self) -> Arc<Mutex<Receiver<RPC>>>;
    fn connect(&mut self, transport: TransportWrapper) -> Result<(), String>;
    fn send_message(&self, addr: NetAddr, payload: Vec<u8>) -> Result<(), String>;
    fn broadcast(&self, payload: Vec<u8>) -> Result<(), String>;
    fn addr(&self) -> NetAddr;
    // fn as_any(&self) -> &dyn Any;
    
}

pub enum TransportWrapper<'a> {
    Local(&'a LocalTransport),
}