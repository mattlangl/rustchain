use std::any::Any;
use std::collections::HashMap;
use std::error::Error;
use std::sync::{Arc, Mutex, RwLock};
use std::sync::mpsc::{Receiver, Sender};

use super::server::Channel;
use super::transport::{NetAddr, RPC, Transport};

#[derive(Clone)]
pub struct LocalTransport {
    addr: NetAddr,
    chan: Channel<RPC>,
    peers: Arc<RwLock<HashMap<NetAddr, LocalTransport>>>,
}

impl LocalTransport {
    pub fn new(addr: NetAddr) -> Self {
        Self {
            addr: addr,
            chan: Channel::new(),
            peers: Arc::new(RwLock::new(HashMap::new()))
        }
    }
}

impl Transport for LocalTransport {
    fn consume(&self) -> Arc<Mutex<Receiver<RPC>>> {
        return self.chan.receiver();
    }

    fn connect(&mut self, transport: &dyn Transport) -> Result<(), String> {
        let local_transport: &LocalTransport = transport.as_any().downcast_ref().expect("not local transport");
        let addr = local_transport.addr();
        self.peers.write().unwrap().insert(addr, local_transport.clone().to_owned());
        Ok(())
    }

    fn send_message(&self, to: NetAddr, payload: Vec<u8>) -> Result<(), String> {
        let peers = self.peers.write().unwrap();

        let peer = peers.get(&to).expect("not found");

        let s = peer.chan.sender();

        match s.send(RPC {
            from: self.addr(),
            payload,
        }) {
            Ok(x) => Ok(x),
            Err(x) => Err("error sending".to_owned()),
        }
    }

    fn addr(&self) -> NetAddr {
        return self.addr.clone();
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

}
