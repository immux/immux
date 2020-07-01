use std::path::PathBuf;

use crate::database::errors::DatabaseResult;
use crate::database::unit_content::UnitContent;
use crate::database::unit_key::UnitKey;
use crate::storage::chain_height::ChainHeight;
use crate::storage::command::Command;
use crate::storage::kv::LogKeyValueStore;
use crate::storage::kvkey::KVKey;
use crate::storage::kvvalue::KVValue;
use crate::storage::transaction_manager::TransactionId;

pub struct Database {
    store_engine: LogKeyValueStore,
}

impl Database {
    pub fn open(path: &PathBuf) -> DatabaseResult<Database> {
        let store_engine = LogKeyValueStore::open(path)?;
        let database = Database { store_engine };
        return Ok(database);
    }

    pub fn set(
        &mut self,
        key: UnitKey,
        value: UnitContent,
        transaction_id: Option<TransactionId>,
    ) -> DatabaseResult<()> {
        let kv_key = KVKey::from(&key);
        let kv_value = KVValue::from(value);
        self.store_engine.set(kv_key, kv_value, transaction_id)?;
        return Ok(());
    }

    pub fn get(
        &mut self,
        key: &UnitKey,
        transaction_id: &Option<TransactionId>,
    ) -> DatabaseResult<Option<UnitContent>> {
        let kv_key = KVKey::from(key);
        match self.store_engine.get(&kv_key, transaction_id)? {
            None => Ok(None),
            Some(kv_value) => {
                let (content, _) = UnitContent::parse(kv_value.as_bytes())?;
                return Ok(Some(content));
            }
        }
    }

    pub fn revert_one(
        &mut self,
        key: UnitKey,
        height: &ChainHeight,
        transaction_id: Option<TransactionId>,
    ) -> DatabaseResult<()> {
        let kv_key = KVKey::from(&key);
        self.store_engine
            .revert_one(kv_key, height, transaction_id)?;
        return Ok(());
    }

    pub fn revert_all(&mut self, height: &ChainHeight) -> DatabaseResult<()> {
        self.store_engine.revert_all(height)?;
        return Ok(());
    }

    pub fn remove_one(
        &mut self,
        key: UnitKey,
        transaction_id: Option<TransactionId>,
    ) -> DatabaseResult<()> {
        let kv_key = KVKey::from(&key);
        self.store_engine.remove_one(kv_key, transaction_id)?;
        return Ok(());
    }

    pub fn remove_all(&mut self) -> DatabaseResult<()> {
        self.store_engine.remove_all()?;
        return Ok(());
    }
    
    pub fn inspect(
        &mut self,
        key: Option<&UnitKey>,
    ) -> DatabaseResult<Vec<(Command, ChainHeight)>> {
        let kv_key = key.map(|unit_key| KVKey::from(unit_key));
        let result = self.store_engine.inspect(kv_key.as_ref())?;

        return Ok(result);
    }

    pub fn start_transaction(&mut self) -> DatabaseResult<TransactionId> {
        let transaction_id = self.store_engine.start_transaction()?;
        return Ok(transaction_id);
    }

    pub fn commit_transaction(&mut self, transaction_id: TransactionId) -> DatabaseResult<()> {
        self.store_engine.commit_transaction(transaction_id)?;
        return Ok(());
    }

    pub fn abort_transaction(&mut self, transaction_id: TransactionId) -> DatabaseResult<()> {
        self.store_engine.abort_transaction(transaction_id)?;
        return Ok(());
    }
}
