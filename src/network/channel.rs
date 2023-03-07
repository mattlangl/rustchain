use std::sync::{mpsc::{SyncSender, Receiver, self}, Mutex, Arc};


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

