use std::collections::HashMap;

use std::sync::{Arc, Mutex, RwLock};
use std::sync::mpsc::{Receiver};

use crate::network::channel::Channel;

use super::transport::{NetAddr, RPC, Transport, TransportWrapper};

#[derive(Clone)]
pub struct LocalTransport {
    addr: NetAddr,
    chan: Channel<RPC>,
    peers: Arc<RwLock<HashMap<NetAddr, LocalTransport>>>,
}



impl LocalTransport {
    pub fn new(addr: NetAddr) -> Self {
        Self {
            addr,
            chan: Channel::new(),
            peers: Arc::new(RwLock::new(HashMap::new()))
        }
    }

    fn send(&self, to: &LocalTransport, payload: Vec<u8>) -> Result<(), String> {
        let s = to.chan.sender();

        match s.send(RPC {
            from: self.addr(),
            payload: payload,
        }) {
            Ok(x) => Ok(x),
            Err(_x) => Err("error sending".to_owned()),
        }
    }

    pub fn has_peer(&self, addr: &NetAddr) -> bool {
        let peers = self.peers.read().unwrap();
        peers.contains_key(addr)
    }
}

impl Transport for LocalTransport {
    fn consume(&self) -> Arc<Mutex<Receiver<RPC>>> {
        self.chan.receiver()
    }

    fn connect(&mut self, transport: TransportWrapper) -> Result<(), String> {
        let local_transport = match transport{
            TransportWrapper::Local(t) => t,
        };
        let addr = local_transport.addr();
        self.peers.write().unwrap().insert(addr, local_transport.clone());
        Ok(())
    }

    fn send_message(&self, to: NetAddr, payload: Vec<u8>) -> Result<(), String> {
        let peers = self.peers.read().unwrap();

        let peer = peers.get(&to).expect("not found");

        self.send(peer, payload)
    }

    fn addr(&self) -> NetAddr {
        self.addr.clone()
    }

    fn broadcast(&self, payload: Vec<u8>) -> Result<(), String> {
        let peers = self.peers.read().unwrap();
        for (_, transport) in peers.iter() {
            self.send(transport, payload.clone());
        }
        Ok(())
    }


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connect() {
        let mut tra = LocalTransport::new("A".to_string());
        let mut trb = LocalTransport::new("B".to_string());
        let mut trc = LocalTransport::new("C".to_string());

        assert!(tra.connect(TransportWrapper::Local(&trb)).is_ok());
        assert!(trb.connect(TransportWrapper::Local(&tra)).is_ok());

        assert_eq!(tra.has_peer(&trb.addr), true);
        assert_eq!(trb.has_peer(&tra.addr), true);

        assert_eq!(trc.has_peer(&trb.addr), false);
        assert_eq!(trc.has_peer(&trb.addr), false);

    }

    #[test]
    fn test_send_message() {
        let mut tra = LocalTransport::new("A".to_string());
        let mut trb = LocalTransport::new("B".to_string());

        assert!(tra.connect(TransportWrapper::Local(&trb)).is_ok());
        assert!(trb.connect(TransportWrapper::Local(&tra)).is_ok());

        let msg = b"hello world".to_vec();
        let receiver = trb.consume();
        std::thread::spawn(move || {
            while let Ok(rpc) = receiver.lock().unwrap().recv() {
                assert_eq!(rpc.payload, msg);
                break;
            }
        });
        

        let msg = b"hello world".to_vec();
        assert!(tra.send_message(trb.addr.clone(), msg).is_ok());

        println!("done");


    }

    #[test]
    fn test_broadcast() {
        let mut tra = LocalTransport::new("A".to_string());
        let mut trb = LocalTransport::new("B".to_string());
        let mut trc = LocalTransport::new("C".to_string());

        assert!(tra.connect(TransportWrapper::Local(&trb)).is_ok());
        assert!(tra.connect(TransportWrapper::Local(&trc)).is_ok());

        let trs = vec![trb, trc];
        let msg = b"foo".to_vec();

        for transport in trs {
            let receiver = transport.consume();
            let msg = msg.clone();
            std::thread::spawn(move || {
                while let Ok(rpc) = receiver.lock().unwrap().recv() {
                    assert_eq!(rpc.payload, msg);
                    break;
                }
            });
        }

        assert!(tra.broadcast(msg).is_ok());
    }
}