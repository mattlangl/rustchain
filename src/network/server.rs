
use std::sync::mpsc::{Receiver, SyncSender};
use std::sync::{mpsc, Arc};
use std::time::{Duration, Instant};

use log::info;

use crate::core::hasher::{Hasher};
use crate::core::transaction::Transaction;
use crate::crypto::keypair::PrivateKey;

use super::channel::Channel;
use super::rpc::RPCDecodeFunc;
use super::transport::{Transport, RPC};
use super::txpool::TxPool;

const default_time: std::time::Duration = Duration::new(5, 0);

pub struct ServerOpts {
    pub transports: Vec<Box<dyn Transport>>,
    pub block_time: Option<Duration>,
    pub key: Option<PrivateKey>,
    pub rpc_decode_func: RPCDecodeFunc,
}

pub struct Server {
    opts: ServerOpts,
    block_time: Duration,
    pool: TxPool,
    validator: bool,
    rpc_ch: Channel<RPC>,
    quit_ch: Channel<()>,
    hasher: Hasher,
}

impl Server {
    pub fn new(opts: ServerOpts) -> Server {
        let duration = match opts.block_time
         {
            Some(s) => s,
            None => default_time,
        };
        Server {
            rpc_ch: Channel::new(),
            quit_ch: Channel::new(),
            block_time: duration,
            pool: TxPool::new(),
            validator: opts.key.is_some(),
            opts,
            hasher: Hasher::new(),
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

        let mut ticker = Instant::now() + self.block_time;

        loop {
            let msg = self.rpc_ch.receiver().lock().unwrap().try_recv();
            match msg {
                Ok(rpc) => println!("{:?}", rpc),
                Err(mpsc::TryRecvError::Empty) => (),
                Err(mpsc::TryRecvError::Disconnected) => break,
            };

            if ticker <= Instant::now() {
                println!("do stuff every x seconds");
                ticker = Instant::now() + self.block_time;
            }

            //std::thread::yield_now();
        }

        println!("Server shutdown");
    }

    fn handle_transaction(&mut self, tx: &Transaction) -> Result<(), Box<dyn std::error::Error>> {
        if let Err(e) = tx.verify() {
            return Err(Box::new(e));
        }

        let hash = self.hasher.hash(tx).unwrap();

        if self.pool.has(&hash) {
            info!("transaction already in mempool: hash={}", hash);
            return Ok(());
        }

        info!("adding new tx to the mempool: hash={}", hash);

        self.pool.add(tx.clone());

        Ok(())
    }

    fn create_new_block(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("creating a new block");
        Ok(())
    }

}


