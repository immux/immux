use crate::storage::kvkey::KVKey;
use std::collections::HashMap;

pub type TransactionId = u64;

#[derive(Debug, PartialEq)]
pub enum TransactionManagerError {
    TransactionIdOutOfRange,
    TransactionNotAlive,
}

pub struct TransactionManager {
    current_transaction_id: TransactionId,
    transaction_id_to_keys: HashMap<TransactionId, Vec<KVKey>>,
}

impl TransactionManager {
    pub fn new() -> TransactionManager {
        TransactionManager {
            current_transaction_id: 0,
            transaction_id_to_keys: HashMap::new(),
        }
    }

    pub fn generate_new_transaction_id(
        &mut self,
    ) -> Result<TransactionId, TransactionManagerError> {
        if self.current_transaction_id == u64::MAX {
            return Err(TransactionManagerError::TransactionIdOutOfRange);
        }

        self.current_transaction_id += 1;
        return Ok(self.current_transaction_id);
    }

    pub fn update_transaction_id(&mut self, transaction_id: TransactionId) {
        self.current_transaction_id = transaction_id;
    }

    pub fn update_transaction_id_to_keys(
        &mut self,
        transaction_id: TransactionId,
        key: Option<KVKey>,
    ) {
        if let Some(key) = key {
            if let Some(kvs) = self.transaction_id_to_keys.get_mut(&transaction_id) {
                kvs.push(key);
            } else {
                self.transaction_id_to_keys
                    .insert(transaction_id, vec![key]);
            }
        } else {
            self.transaction_id_to_keys.insert(transaction_id, vec![]);
        }
    }

    pub fn validate_transaction_id(
        &self,
        transaction_id: &TransactionId,
    ) -> Result<(), TransactionManagerError> {
        return if self.transaction_id_to_keys.contains_key(&transaction_id) {
            Ok(())
        } else {
            Err(TransactionManagerError::TransactionNotAlive)
        };
    }

    pub fn get_affected_keys(&self, transaction_id: &TransactionId) -> Option<&Vec<KVKey>> {
        self.transaction_id_to_keys.get(&transaction_id)
    }

    pub fn remove_transaction(&mut self, transaction_id: &TransactionId) {
        self.transaction_id_to_keys.remove(&transaction_id);
    }
}
