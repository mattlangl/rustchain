
use std::sync::mpsc::{Receiver, SyncSender};
use std::sync::{mpsc, Mutex, Arc};
use std::time::{Duration, Instant};

use super::transport::{Transport, RPC};

#[derive(Clone)]
pub struct Channel<T> {
    sender: SyncSender<T>,
    receiver: Arc<Mutex<Receiver<T>>>,
}

impl<T> Channel<T>  {
    pub fn new() -> Channel<T> {
        let (send, recv) = mpsc::sync_channel(0);
        Channel {
            sender: send,
            receiver: Arc::new(Mutex::new(recv)),
        }
    }

    pub fn sender(&self) -> SyncSender<T> {
        self.sender.clone()
    }

    pub fn receiver(&self) -> Arc<Mutex<Receiver<T>>> {
        self.receiver.clone()
    }

}



pub struct ServerOpts {
    pub transports: Vec<Box<dyn Transport>>,
}

pub struct Server<> {
    opts: ServerOpts,
    rpc_ch: Channel<RPC>,
    quit_ch: Channel<()>,
}

impl Server {
    pub fn new(opts: ServerOpts) -> Server {
        Server {
            opts,
            rpc_ch: Channel::new(),
            quit_ch: Channel::new(),
        }
    }

    pub fn start(self) {
        
        for transport in self.opts.transports {
            let sender = self.rpc_ch.sender().clone();
            std::thread::spawn(move || {
                while let Ok(msg) = transport.consume().lock().unwrap().recv() {
                    let _ = sender.send(msg);
                }
            });
        }

        let mut ticker = Instant::now();

        loop {
            let msg = self.rpc_ch.receiver().lock().unwrap().try_recv();
            match msg {
                Ok(rpc) => println!("{:?}", rpc),
                Err(mpsc::TryRecvError::Empty) => (),
                Err(mpsc::TryRecvError::Disconnected) => break,
            };

            if ticker.elapsed() >= Duration::from_secs(5) {
                println!("do stuff every x seconds");
                ticker = Instant::now();
            }

            //std::thread::yield_now();
        }

        println!("Server shutdown");
    }

    pub fn shutdown(&self) {
        let _ = self.quit_ch.sender().send(());
    }

}


