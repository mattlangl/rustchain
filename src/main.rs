use std::{time, thread};
use crypto::keypair::PrivateKey;
use simple_logger::SimpleLogger;
use network::{local_transport::LocalTransport, transport::{Transport, TransportWrapper}, server::{ServerOpts, Server}, rpc::default_rpc_decode_func};

mod network;
mod core;
mod types;
mod crypto;

fn main() {
    SimpleLogger::new().with_threads(true).init().unwrap();

    let mut tr_local = LocalTransport::new("LOCAL".to_owned());
    let mut tr_remote = LocalTransport::new("REMOTE".to_owned());

    tr_local.connect(TransportWrapper::Local(&tr_remote));

    tr_remote.connect(TransportWrapper::Local(&tr_local));

    let sec = time::Duration::from_secs(1);

    let local_addr = tr_local.addr();



    thread::spawn(move || {
        loop {
        tr_remote.send_message(local_addr.clone() ,"Hello World".as_bytes().to_vec());
        thread::sleep(sec);
        }
    });

    let mut opts = ServerOpts {
        transports: Vec::new(),
        block_time: Some(time::Duration::new(300, 0)),
        key: Some(PrivateKey::generate_key()),
        rpc_decode_func: default_rpc_decode_func,
    };

    opts.transports.push(Box::new(tr_local.clone()));

    let server = Server::new(opts);

    server.start();

}
