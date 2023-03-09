
use std::sync::mpsc::{Receiver, SyncSender};
use std::sync::{mpsc, Arc};
use std::time::{Duration, Instant};

use log::info;

use crate::core::hasher::TxHasher;
use crate::core::transaction::Transaction;
use crate::crypto::keypair::PrivateKey;

use super::channel::Channel;
use super::transport::{Transport, RPC};
use super::txpool::TxPool;


pub struct ServerOpts {
    pub transports: Vec<Box<dyn Transport>>,
    block_time: Duration,
    key: Option<PrivateKey>,
}

pub struct Server<> {
    opts: ServerOpts,
    block_time: Duration,
    pool: TxPool,
    validator: bool,
    rpc_ch: Channel<RPC>,
    quit_ch: Channel<()>,
}

impl Server {
    pub fn new(opts: ServerOpts) -> Server {
        Server {
            rpc_ch: Channel::new(),
            quit_ch: Channel::new(),
            block_time: opts.block_time.clone(),
            pool: TxPool::new(),
            validator: opts.key.is_some(),
            opts,

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

    fn handle_transaction(&self, tx: &Transaction) -> Result<(), Box<dyn std::error::Error>> {
        if let Err(e) = tx.verify() {
            return Err(Box::new(e));
        }

        let hash = tx.hash();

        if self.pool.has(hash.clone()) {
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


