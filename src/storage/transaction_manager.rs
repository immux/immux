use std::collections::HashMap;

use crate::constants as Constants;
use crate::storage::kvkey::KVKey;
use crate::utils::varint::varint_encode;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Hash, Eq)]
pub struct TransactionId(u64);

impl TransactionId {
    pub fn new(data: u64) -> Self {
        Self(data)
    }

    pub fn increment(&mut self) -> Result<Self, TransactionManagerError> {
        if self.0 >= Constants::MAX_TRANSACTION_ID {
            return Err(TransactionManagerError::TransactionIdOutOfRange);
        }
        self.0 += 1;
        return Ok(Self(self.0));
    }

    pub fn as_u64(&self) -> u64 {
        self.0
    }

    pub fn serialize(&self) -> Vec<u8> {
        varint_encode(self.as_u64())
    }
}

#[derive(Debug, PartialEq)]
pub enum TransactionManagerError {
    TransactionIdOutOfRange,
    TransactionNotAlive,
}

pub struct TransactionManager {
    current_transaction_id: TransactionId,
    affected_keys_in_transactions: HashMap<TransactionId, Vec<KVKey>>,
}

impl TransactionManager {
    pub fn new() -> TransactionManager {
        TransactionManager {
            current_transaction_id: TransactionId::new(0),
            affected_keys_in_transactions: HashMap::new(),
        }
    }

    pub fn generate_new_transaction_id(&mut self) -> Result<TransactionId, TransactionManagerError> {
        let next_transaction_id = self.current_transaction_id.increment()?;
        return Ok(next_transaction_id);
    }

    pub fn update_transaction_id(&mut self, transaction_id: &TransactionId) {
        self.current_transaction_id = transaction_id.to_owned();
    }

    pub fn add_affected_keys(&mut self, transaction_id: &TransactionId, key: &KVKey) {
        if let Some(keys) = self.affected_keys_in_transactions.get_mut(&transaction_id) {
            keys.push(key.to_owned());
        } else {
            self.affected_keys_in_transactions.insert(transaction_id.clone(), vec![key.to_owned()]);
        }
    }

    pub fn initialize_affected_keys(&mut self, transaction_id: &TransactionId) {
        self.affected_keys_in_transactions.insert(transaction_id.clone(), vec![]);
    }

    pub fn validate_transaction_id(&self, transaction_id: &TransactionId) -> Result<(), TransactionManagerError> {
        return if self.affected_keys_in_transactions.contains_key(&transaction_id) {
            Ok(())
        } else {
            Err(TransactionManagerError::TransactionNotAlive)
        };
    }

    pub fn get_affected_keys(&self, transaction_id: &TransactionId) -> Vec<KVKey> {
        if let Some(keys) = self.affected_keys_in_transactions.get(&transaction_id) {
            return keys.to_owned();
        } else {
            return vec![];
        }
    }

    pub fn remove_transaction(&mut self, transaction_id: &TransactionId) {
        self.affected_keys_in_transactions.remove(&transaction_id);
    }
}
