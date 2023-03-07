use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use crate::core::{transaction::Transaction, hasher::TxHasher};
use crate::types::hash::Hash;

pub struct TxPool {
    transactions: Arc<RwLock<HashMap<Hash, Transaction>>>,
}

impl TxPool {
    pub fn new() -> TxPool {
        TxPool {
            transactions: Arc::new(HashMap::new().into()),
        }
    }

    pub fn add(&mut self, mut tx: Transaction) -> Result<(), ()> {
        let mut transactions = self.transactions.write().unwrap();
        let hash = tx.hash(Box::new(TxHasher::new()));
        transactions.insert(hash, tx);
        Ok(())
    }

    pub fn has(&self, hash: Hash) -> bool {
        let transactions = self.transactions.read().unwrap();
        transactions.contains_key(&hash)
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
}

