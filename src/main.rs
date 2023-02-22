use std::{time, thread};

use network::{local_transport::LocalTransport, transport::Transport, server::{ServerOpts, Server}};

mod network;
mod core;
mod types;

fn main() {
    let mut trLocal = LocalTransport::new("LOCAL".to_owned());
    let mut trRemote = LocalTransport::new("REMOTE".to_owned());

    trLocal.connect(&trRemote);

    trRemote.connect(&trLocal);

    let sec = time::Duration::from_secs(1);

    let localAddr = trLocal.addr();



    thread::spawn(move || {
        loop {
        trRemote.send_message(localAddr.clone() ,"Hello World".as_bytes().to_vec());
        thread::sleep(sec);
        }
    });

    let mut opts = ServerOpts {
        transports: Vec::new(),
    };

    opts.transports.push(Box::new(trLocal.clone()));

    let server = Server::new(opts);

    server.start();

}
