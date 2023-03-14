use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use crate::core::hasher::Hasher;
use crate::core::{transaction::Transaction};
use crate::types::hash::Hash;
use rand::Rng;

type TxMap = HashMap<Hash, Transaction>;
pub struct TxMapSorter {
    transactions: Vec<Transaction>
}

impl TxMapSorter {
    fn new(map: &TxMap) -> Self {
        let mut txs = Vec::with_capacity(map.len());
        for (_, tx) in map {
            txs.push(tx.clone());
        }
        txs.sort_by(|a, b| a.seen.cmp(&b.seen));
        TxMapSorter { transactions: txs }
    }
}

pub struct TxPool {
    transactions: Arc<RwLock<TxMap>>,
}

impl TxPool {
    pub fn new() -> TxPool {
        TxPool {
            transactions: Arc::new(RwLock::new(TxMap::new())),
        }
    }

    pub fn add(&mut self, mut tx: Transaction) -> Result<(), ()> {
        let mut transactions = self.transactions.write().unwrap();
        let hash = tx.hash(Hasher::new());
        transactions.insert(hash, tx);
        Ok(())
    }

    pub fn has(&self, hash: &Hash) -> bool {
        let transactions = self.transactions.read().unwrap();
        transactions.contains_key(hash)
    }

    pub fn len(&self) -> usize {
        let transactions = self.transactions.read().unwrap();
        transactions.len()
    }

    pub fn flush(&mut self) -> Result<(), ()> {
        let mut transactions = self.transactions.write().unwrap();

        transactions.clear();
        Ok(())
    }

    fn get_transactions(&self) -> Vec<Transaction> {
        let transactions = self.transactions.read().unwrap();
        let sorter = TxMapSorter::new(&transactions);
        return sorter.transactions;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tx_pool() {
        let p = TxPool::new();
        assert_eq!(p.len(), 0);
    }

    #[test]
    fn test_tx_pool_add_tx() {
        let mut p = TxPool::new();
        let tx = Transaction::new(b"fooo".to_vec()).unwrap();
        assert!(p.add(tx).is_ok());
        assert_eq!(p.len(), 1);

        let _ = Transaction::new(b"fooo".to_vec());
        assert_eq!(p.len(), 1);

        let tx = Transaction::new(b"sway".to_vec()).unwrap();
        assert!(p.add(tx).is_ok());
        assert_eq!(p.len(), 2);

        p.flush();
        assert_eq!(p.len(), 0);
    }

    #[test]
fn test_sort_transactions() {
    let mut p = TxPool::new();
    let tx_len = 1000;
    let mut rng = rand::thread_rng();


    for i in 0..tx_len {
        let mut tx = Transaction::new(i.to_string().as_bytes().to_vec());
        assert!(tx.is_ok());
        let mut tx = tx.unwrap();
        tx.set_seen(rng.gen::<i64>());
        assert!(p.add(tx).is_ok());
    }

    assert_eq!(tx_len, p.len());

    let txx = p.get_transactions();
    println!("{:?}", txx);
    for i in 0..txx.len()-1 {
        assert!(txx[i].seen() < txx[i+1].seen());
    }
}
}

