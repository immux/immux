use std::convert::TryFrom;
use std::path::PathBuf;

use crate::storage::chain_height::ChainHeight;
use crate::storage::executor::command::{Command, CommandError, SelectCondition};
use crate::storage::executor::errors::ExecutorResult;
use crate::storage::executor::filter::content_satisfied_filter;
use crate::storage::executor::grouping_label::GroupingLabel;
use crate::storage::executor::unit_content::UnitContent;
use crate::storage::executor::unit_key::UnitKey;
use crate::storage::kv::LogKeyValueStore;
use crate::storage::kvkey::KVKey;
use crate::storage::kvvalue::KVValue;
use crate::storage::transaction_manager::TransactionId;

pub struct Executor {
    store_engine: LogKeyValueStore,
}

impl Executor {
    pub fn open(path: &PathBuf) -> ExecutorResult<Executor> {
        let store_engine = LogKeyValueStore::open(path)?;
        let executor = Executor { store_engine };
        return Ok(executor);
    }

    pub fn set(
        &mut self,
        grouping: &GroupingLabel,
        key: &UnitKey,
        value: &UnitContent,
        transaction_id: Option<TransactionId>,
    ) -> ExecutorResult<()> {
        let kv_key = KVKey::from_grouping_and_unit_key(&grouping, &key);
        let kv_value = KVValue::from(value);
        self.store_engine.set(&kv_key, &kv_value, transaction_id)?;
        return Ok(());
    }

    pub fn get(
        &mut self,
        target_grouping: &GroupingLabel,
        condition: &SelectCondition,
    ) -> ExecutorResult<Vec<UnitContent>> {
        match condition {
            SelectCondition::UnconditionalMatch => {
                let kvs = self.store_engine.get_all_current()?;
                let result: Vec<UnitContent> = kvs
                    .iter()
                    .filter_map(|(key, value)| {
                        if let Ok((grouping, _)) = KVKey::parse(&key.as_bytes()) {
                            if &grouping == target_grouping {
                                match UnitContent::parse(value.as_bytes()) {
                                    Err(_error) => return None,
                                    Ok((content, _)) => return Some(content),
                                }
                            }
                        }
                        return None;
                    })
                    .collect();
                return Ok(result);
            }
            SelectCondition::Filter(filter) => {
                let kvs = self.store_engine.get_all_current()?;
                let result: Vec<UnitContent> = kvs
                    .iter()
                    .filter_map(|(key, value)| {
                        if let Ok((grouping, _unit_key)) = KVKey::parse(&key.as_bytes()) {
                            if &grouping == target_grouping {
                                match UnitContent::parse(value.as_bytes()) {
                                    Err(_error) => return None,
                                    Ok((content, _)) => {
                                        if content_satisfied_filter(&content, &filter) {
                                            return Some(content);
                                        }
                                        return None;
                                    }
                                }
                            }
                        }
                        return None;
                    })
                    .collect();
                return Ok(result);
            }
            SelectCondition::Key(key, transaction_id) => {
                let kv_key = KVKey::from_grouping_and_unit_key(&target_grouping, &key);
                match self.store_engine.get(&kv_key, *transaction_id)? {
                    None => Ok(vec![]),
                    Some(kv_value) => {
                        let (content, _) = UnitContent::parse(kv_value.as_bytes())?;
                        return Ok(vec![content]);
                    }
                }
            }
        }
    }

    pub fn revert_one(
        &mut self,
        grouping: &GroupingLabel,
        key: &UnitKey,
        height: &ChainHeight,
        transaction_id: Option<TransactionId>,
    ) -> ExecutorResult<()> {
        let kv_key = KVKey::from_grouping_and_unit_key(&grouping, &key);
        self.store_engine
            .revert_one(&kv_key, height, transaction_id)?;
        return Ok(());
    }

    pub fn revert_all(&mut self, height: &ChainHeight) -> ExecutorResult<()> {
        self.store_engine.revert_all(height)?;
        return Ok(());
    }

    pub fn remove_one(
        &mut self,
        grouping: &GroupingLabel,
        key: &UnitKey,
        transaction_id: Option<TransactionId>,
    ) -> ExecutorResult<()> {
        let kv_key = KVKey::from_grouping_and_unit_key(&grouping, &key);
        self.store_engine.remove_one(&kv_key, transaction_id)?;
        return Ok(());
    }

    pub fn remove_all(&mut self) -> ExecutorResult<()> {
        self.store_engine.remove_all()?;
        return Ok(());
    }

    pub fn inspect_all(&mut self) -> ExecutorResult<Vec<(Command, ChainHeight)>> {
        let result: Result<Vec<_>, CommandError> = self
            .store_engine
            .inspect_all()?
            .iter()
            .map(|(command, height)| {
                let instruction = Command::try_from(command)?;
                Ok((instruction, height.to_owned()))
            })
            .collect();

        return Ok(result?);
    }

    pub fn inspect_one(
        &mut self,
        grouping: &GroupingLabel,
        target_key: &UnitKey,
    ) -> ExecutorResult<Vec<(Command, ChainHeight)>> {
        let kv_key = KVKey::from_grouping_and_unit_key(&grouping, &target_key);
        let result: Result<Vec<_>, CommandError> = self
            .store_engine
            .inspect_one(&kv_key)?
            .iter()
            .map(|(command, height)| {
                let instruction = Command::try_from(command)?;
                Ok((instruction, height.to_owned()))
            })
            .collect();

        return Ok(result?);
    }

    pub fn start_transaction(&mut self) -> ExecutorResult<TransactionId> {
        let transaction_id = self.store_engine.start_transaction()?;
        return Ok(transaction_id);
    }

    pub fn commit_transaction(&mut self, transaction_id: TransactionId) -> ExecutorResult<()> {
        self.store_engine.commit_transaction(transaction_id)?;
        return Ok(());
    }

    pub fn abort_transaction(&mut self, transaction_id: TransactionId) -> ExecutorResult<()> {
        self.store_engine.abort_transaction(transaction_id)?;
        return Ok(());
    }
}
