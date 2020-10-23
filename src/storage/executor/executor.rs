use std::convert::TryFrom;

use crate::storage::chain_height::ChainHeight;
use crate::storage::executor::command::{Command, CommandError, SelectCondition};
use crate::storage::executor::errors::{ExecutorError, ExecutorResult};
use crate::storage::executor::filter::content_satisfied_filter;
use crate::storage::executor::grouping_label::GroupingLabel;
use crate::storage::executor::outcome::Outcome;
use crate::storage::executor::unit_content::UnitContent;
use crate::storage::executor::unit_key::UnitKey;
use crate::storage::kv::LogKeyValueStore;
use crate::storage::kvkey::KVKey;
use crate::storage::kvvalue::KVValue;
use crate::storage::preferences::DBPreferences;
use crate::storage::transaction_manager::TransactionId;

pub struct Executor {
    store_engine: LogKeyValueStore,
}

impl Executor {
    pub fn open(preferences: &DBPreferences) -> ExecutorResult<Executor> {
        let store_engine = LogKeyValueStore::open(preferences)?;
        let executor = Executor { store_engine };
        return Ok(executor);
    }

    pub fn set(
        &mut self,
        grouping: &GroupingLabel,
        key: &UnitKey,
        value: &UnitContent,
        transaction_id: Option<TransactionId>,
    ) -> ExecutorResult<Outcome> {
        let kv_key = KVKey::from_grouping_and_unit_key(&grouping, &key);
        let kv_value = KVValue::from(value);
        self.store_engine.set(&kv_key, &kv_value, transaction_id)?;

        if let Some(_) = transaction_id {
            return Ok(Outcome::TransactionalInsertSuccess);
        } else {
            return Ok(Outcome::InsertSuccess);
        }
    }

    pub fn get(&mut self, condition: &SelectCondition) -> ExecutorResult<Outcome> {
        match condition {
            SelectCondition::UnconditionalMatch(target_grouping) => {
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
                return Ok(Outcome::Select(result));
            }
            SelectCondition::Filter(target_grouping, filter) => {
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
                return Ok(Outcome::Select(result));
            }
            SelectCondition::Key(target_grouping, key, transaction_id) => {
                let kv_key = KVKey::from_grouping_and_unit_key(&target_grouping, &key);
                match self.store_engine.get(&kv_key, *transaction_id)? {
                    None => Ok(Outcome::Select(vec![])),
                    Some(kv_value) => {
                        let (content, _) = UnitContent::parse(kv_value.as_bytes())?;
                        return Ok(Outcome::Select(vec![content]));
                    }
                }
            }
            SelectCondition::AllGrouping => {
                let kvs = self.store_engine.get_all_current()?;
                let mut result = vec![];

                for (key, _value) in kvs.iter() {
                    let (grouping, _unit_key) = KVKey::parse(&key.as_bytes())?;
                    if !result.contains(&grouping) {
                        result.push(grouping);
                    }
                }
                return Ok(Outcome::GetAllGroupingsSuccess(result));
            }
        }
    }

    pub fn remove_groupings(&mut self, groupings: &[GroupingLabel]) -> ExecutorResult<Outcome> {
        let outcome = self.start_transaction()?;

        match outcome {
            Outcome::CreateTransaction(transaction_id) => {
                let current_keys = self.store_engine.get_current_keys()?;
                for key in current_keys {
                    let (grouping, _unit_key) = KVKey::parse(&key.as_bytes())?;
                    if groupings.contains(&grouping) {
                        self.store_engine.remove_one(&key, Some(transaction_id))?;
                    }
                }
                self.commit_transaction(transaction_id)?;
                return Ok(Outcome::DeleteGroupingSuccess);
            }
            _ => Err(ExecutorError::UnexpectedOutcome),
        }
    }

    pub fn revert_one(
        &mut self,
        grouping: &GroupingLabel,
        key: &UnitKey,
        height: &ChainHeight,
        transaction_id: Option<TransactionId>,
    ) -> ExecutorResult<Outcome> {
        let kv_key = KVKey::from_grouping_and_unit_key(&grouping, &key);
        self.store_engine
            .revert_one(&kv_key, height, transaction_id)?;
        if let Some(_) = transaction_id {
            return Ok(Outcome::TransactionalRevertOneSuccess);
        } else {
            return Ok(Outcome::RevertOneSuccess);
        }
    }

    pub fn revert_all(&mut self, height: &ChainHeight) -> ExecutorResult<Outcome> {
        self.store_engine.revert_all(height)?;
        return Ok(Outcome::RevertAllSuccess);
    }

    pub fn remove_one(
        &mut self,
        grouping: &GroupingLabel,
        key: &UnitKey,
        transaction_id: Option<TransactionId>,
    ) -> ExecutorResult<Outcome> {
        let kv_key = KVKey::from_grouping_and_unit_key(&grouping, &key);
        self.store_engine.remove_one(&kv_key, transaction_id)?;
        if let Some(_) = transaction_id {
            return Ok(Outcome::TransactionalRemoveOneSuccess);
        } else {
            return Ok(Outcome::RemoveOneSuccess);
        }
    }

    pub fn remove_all(&mut self) -> ExecutorResult<Outcome> {
        self.store_engine.remove_all()?;
        return Ok(Outcome::RemoveAllSuccess);
    }

    pub fn inspect_all(&mut self) -> ExecutorResult<Outcome> {
        let result: Result<Vec<_>, CommandError> = self
            .store_engine
            .inspect_all()?
            .iter()
            .map(|(command, height)| {
                let instruction = Command::try_from(command)?;
                Ok((instruction, height.to_owned()))
            })
            .collect();
        let outcome = Outcome::InspectAll(result?);
        return Ok(outcome);
    }

    pub fn inspect_one(
        &mut self,
        grouping: &GroupingLabel,
        target_key: &UnitKey,
    ) -> ExecutorResult<Outcome> {
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

        let outcome = Outcome::InspectOne(result?);
        return Ok(outcome);
    }

    pub fn start_transaction(&mut self) -> ExecutorResult<Outcome> {
        let transaction_id = self.store_engine.start_transaction()?;
        return Ok(Outcome::CreateTransaction(transaction_id));
    }

    pub fn commit_transaction(&mut self, transaction_id: TransactionId) -> ExecutorResult<Outcome> {
        self.store_engine.commit_transaction(transaction_id)?;
        return Ok(Outcome::TransactionCommitSuccess);
    }

    pub fn abort_transaction(&mut self, transaction_id: TransactionId) -> ExecutorResult<Outcome> {
        self.store_engine.abort_transaction(transaction_id)?;
        return Ok(Outcome::TransactionAbortSuccess);
    }
}
