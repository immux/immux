use std::collections::{HashMap, HashSet};
use std::fs::create_dir_all;
use std::path::{Path, PathBuf};

use crate::constants as Constants;
use crate::storage::chain_height::ChainHeight;
use crate::storage::errors::{KVError, KVResult};
use crate::storage::instruction::{Instruction, InstructionError};
use crate::storage::kvkey::KVKey;
use crate::storage::kvvalue::KVValue;
use crate::storage::log_pointer::LogPointer;
use crate::storage::log_reader::LogReader;
use crate::storage::log_version::LogVersion;
use crate::storage::log_writer::LogWriter;
use crate::storage::preferences::DBPreferences;
use crate::storage::transaction_manager::{Snapshot, TransactionId, TransactionManager};

/// The core of ImmuxDB storage engine.
/// Log file sequentially writes down all the instructions, we maintain a hashmap index in our
/// memory. The key is the KVKey type, value is the LogPointer type.

pub struct LogKeyValueStore {
    reader: LogReader,
    writer: LogWriter,
    snapshot: Snapshot,
    current_height: ChainHeight,
    transaction_manager: TransactionManager,
}

impl LogKeyValueStore {
    /// Load the entire log file, initialize ImmuxDB storage engine.
    pub fn open(preferences: &DBPreferences) -> KVResult<LogKeyValueStore> {
        create_dir_all(&preferences.log_dir)?;

        let log_file_path = get_main_log_full_path(&preferences.log_dir);

        let db_version_major_str = env!("CARGO_PKG_VERSION_MAJOR");
        let db_version_minor_str = env!("CARGO_PKG_VERSION_MINOR");
        let db_version_revise_str = env!("CARGO_PKG_VERSION_PATCH");

        let db_version = vec![
            db_version_major_str,
            db_version_minor_str,
            db_version_revise_str,
        ];

        let db_version = LogVersion::try_from(&db_version)?;

        let writer = LogWriter::new(&log_file_path, preferences.ecc_mode, db_version)?;
        let mut reader = LogReader::new(&log_file_path, db_version)?;

        let (snapshot, current_height, transaction_manager) = load_snapshot(&mut reader, None)?;

        let incomplete_transaction_ids: Vec<TransactionId> = transaction_manager
            .transactions
            .keys()
            .map(|transaction_id| transaction_id.clone())
            .collect();

        let mut engine = LogKeyValueStore {
            reader,
            writer,
            snapshot,
            current_height,
            transaction_manager,
        };

        for transaction_id in incomplete_transaction_ids {
            engine.abort_transaction(&transaction_id)?;
        }

        return Ok(engine);
    }

    /// Set a key value pair within/without a transaction.
    pub fn set(
        &mut self,
        key: &KVKey,
        value: &KVValue,
        transaction_id: Option<TransactionId>,
    ) -> KVResult<()> {
        if key.as_bytes().len() > Constants::MAX_KEY_LENGTH {
            return Err(InstructionError::KeyExceedsMaxLength.into());
        }

        let instruction = {
            if let Some(transaction_id) = transaction_id {
                self.transaction_manager
                    .validate_transaction_id(&transaction_id)?;
                self.transaction_manager
                    .add_affected_keys(&transaction_id, &key)?;

                Instruction::TransactionalSet {
                    key: key.clone(),
                    value: value.clone(),
                    transaction_id,
                }
            } else {
                Instruction::Set {
                    key: key.clone(),
                    value: value.clone(),
                }
            }
        };

        let log_pointer = self.writer.append_instruction(&instruction)?;

        self.current_height.increment()?;
        if let Some(transaction_id) = transaction_id {
            self.transaction_manager.update_transaction_meta_data(
                &key,
                &log_pointer,
                &transaction_id,
            )?;
        }

        update_snapshot(key, &log_pointer, &mut self.snapshot, transaction_id);

        return Ok(());
    }

    /// Get value of a specific key within/without a transaction.
    pub fn get(
        &mut self,
        key: &KVKey,
        transaction_id: Option<TransactionId>,
    ) -> KVResult<Option<KVValue>> {
        let snapshot = if let Some(transaction_id) = transaction_id {
            let transaction_meta_data = self
                .transaction_manager
                .get_transaction_meta_data(&transaction_id)?;
            transaction_meta_data.snapshot
        } else {
            self.snapshot.clone()
        };

        self.read_from_snapshot(&snapshot, key, transaction_id)
    }

    fn read_from_snapshot(
        &mut self,
        snapshot: &Snapshot,
        key: &KVKey,
        transaction_id: Option<TransactionId>,
    ) -> KVResult<Option<KVValue>> {
        match snapshot.get(&key) {
            None => Ok(None),
            Some(log_pointers) => {
                if let Some(log_pointer) = log_pointers.get(&transaction_id) {
                    let instruction = self.reader.read_pointer(log_pointer)?;
                    match instruction {
                        Instruction::Set { key: _, value } => Ok(Some(value)),
                        Instruction::RevertOne { key, height } => {
                            get_revert_value(&mut self.reader, &key, &height)
                        }
                        Instruction::RemoveOne { key: _ } => Ok(None),
                        Instruction::TransactionalSet {
                            key: _,
                            value,
                            transaction_id: _,
                        } => Ok(Some(value)),
                        Instruction::TransactionalRevertOne {
                            key,
                            height,
                            transaction_id: _,
                        } => get_revert_value(&mut self.reader, &key, &height),
                        Instruction::TransactionalRemoveOne {
                            key: _,
                            transaction_id: _,
                        } => Ok(None),
                        _ => Err(KVError::PointToUnexpectedInstruction),
                    }
                } else {
                    return if transaction_id.is_some() {
                        self.read_from_snapshot(snapshot, &key, None)
                    } else {
                        Ok(None)
                    };
                }
            }
        }
    }

    /// Get all the keys in the current storage engine.
    pub fn get_current_keys(&mut self) -> KVResult<Vec<KVKey>> {
        let keys: Vec<KVKey> = self.snapshot.keys().map(|key| key.clone()).collect();
        return Ok(keys);
    }

    /// Get all the key value pairs in the current storage engine.
    pub fn get_all_current(&mut self) -> KVResult<Vec<(KVKey, KVValue)>> {
        let mut result = vec![];

        for (_key, log_pointer) in &self.snapshot {
            match log_pointer.get(&None) {
                None => {}
                Some(log_pointer) => {
                    let instruction = self.reader.read_pointer(log_pointer)?;

                    match instruction {
                        Instruction::Set { key, value } => {
                            result.push((key, value));
                        }
                        Instruction::RevertOne { key, height } => {
                            if let Some(value) = get_revert_value(&mut self.reader, &key, &height)?
                            {
                                result.push((key, value));
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        return Ok(result);
    }

    /// Revert a specific key to certain height within/without a transaction.
    pub fn revert_one(
        &mut self,
        key: &KVKey,
        height: &ChainHeight,
        transaction_id: Option<TransactionId>,
    ) -> KVResult<()> {
        if key.as_bytes().len() > Constants::MAX_KEY_LENGTH {
            return Err(InstructionError::KeyExceedsMaxLength.into());
        }

        if height > &self.current_height {
            return Err(KVError::RevertOutOfRange);
        }

        let instruction = {
            if let Some(transaction_id) = transaction_id {
                Instruction::TransactionalRevertOne {
                    key: key.clone(),
                    height: height.clone(),
                    transaction_id,
                }
            } else {
                Instruction::RevertOne {
                    key: key.clone(),
                    height: height.clone(),
                }
            }
        };

        if let Some(transaction_id) = transaction_id {
            self.transaction_manager
                .validate_transaction_id(&transaction_id)?;
            self.transaction_manager
                .add_affected_keys(&transaction_id, &key)?;
        }

        let log_pointer = self.writer.append_instruction(&instruction)?;

        self.current_height.increment()?;

        if let Some(transaction_id) = transaction_id {
            self.transaction_manager.update_transaction_meta_data(
                &key,
                &log_pointer,
                &transaction_id,
            )?;
        }

        update_snapshot(key, &log_pointer, &mut self.snapshot, transaction_id);

        return Ok(());
    }

    pub fn revert_all(&mut self, height: &ChainHeight) -> KVResult<()> {
        if height > &self.current_height {
            return Err(KVError::RevertOutOfRange);
        }

        let instruction = Instruction::RevertAll {
            height: height.clone(),
        };

        self.writer.append_instruction(&instruction)?;

        self.current_height.increment()?;
        let (new_snapshot, _, _) = load_snapshot(&mut self.reader, Some(height))?;

        self.snapshot = new_snapshot;
        self.transaction_manager = TransactionManager::new();

        return Ok(());
    }

    /// Remove a specific key within/without a transaciton.
    pub fn remove_one(
        &mut self,
        key: &KVKey,
        transaction_id: Option<TransactionId>,
    ) -> KVResult<()> {
        if key.as_bytes().len() > Constants::MAX_KEY_LENGTH {
            return Err(InstructionError::KeyExceedsMaxLength.into());
        }

        let instruction = {
            if let Some(transaction_id) = transaction_id {
                Instruction::TransactionalRemoveOne {
                    key: key.clone(),
                    transaction_id,
                }
            } else {
                Instruction::RemoveOne { key: key.clone() }
            }
        };

        let log_pointer = self.writer.append_instruction(&instruction)?;

        if let Some(transaction_id) = transaction_id {
            self.transaction_manager
                .validate_transaction_id(&transaction_id)?;
            self.transaction_manager
                .add_affected_keys(&transaction_id, &key)?;
        }

        self.current_height.increment()?;

        update_snapshot(key, &log_pointer, &mut self.snapshot, transaction_id);

        if let Some(transaction_id) = transaction_id {
            self.transaction_manager.update_transaction_meta_data(
                &key,
                &log_pointer,
                &transaction_id,
            )?;
        }

        return Ok(());
    }

    /// Clear the entire storage engine.
    pub fn remove_all(&mut self) -> KVResult<()> {
        let instruction = Instruction::RemoveAll;

        self.writer.append_instruction(&instruction)?;

        for log_pointers in self.snapshot.values_mut() {
            log_pointers.remove(&None);
        }

        self.current_height.increment()?;

        self.snapshot = HashMap::new();
        self.transaction_manager = TransactionManager::new();

        return Ok(());
    }

    /// Get all the historical instructions with their corresponding heights.
    pub fn inspect_all(&mut self) -> KVResult<Vec<(Instruction, ChainHeight)>> {
        let (instruction_iterator, _) = self.reader.read_all()?;

        let instructions_with_height = instruction_iterator
            .enumerate()
            .map(|(index, (instruction, _))| (instruction, ChainHeight::new((index) as u64)))
            .collect();
        return Ok(instructions_with_height);
    }

    /// Get all the historical instructions with their corresponding heights regarding to a specific
    /// key.
    pub fn inspect_one(&mut self, target_key: &KVKey) -> KVResult<Vec<(Instruction, ChainHeight)>> {
        let (instruction_iterator, _) = self.reader.read_all()?;
        let mut appeared_key = HashSet::new();
        let result = instruction_iterator
            .enumerate()
            .filter_map(|(index, (instruction, _))| match &instruction {
                Instruction::Set { key, value: _ } => {
                    appeared_key.insert(key.clone());

                    if target_key == key {
                        Some((instruction, ChainHeight::new((index) as u64)))
                    } else {
                        None
                    }
                }
                Instruction::RevertOne { key, height: _ } => {
                    appeared_key.insert(key.clone());

                    if target_key == key {
                        Some((instruction, ChainHeight::new((index) as u64)))
                    } else {
                        None
                    }
                }
                Instruction::RevertAll { height: _ } => {
                    if appeared_key.contains(&target_key) {
                        Some((instruction, ChainHeight::new((index) as u64)))
                    } else {
                        None
                    }
                }
                Instruction::RemoveOne { key } => {
                    appeared_key.insert(key.clone());

                    if target_key == key {
                        Some((instruction, ChainHeight::new((index) as u64)))
                    } else {
                        None
                    }
                }
                Instruction::RemoveAll => {
                    if appeared_key.contains(&target_key) {
                        Some((instruction, ChainHeight::new((index) as u64)))
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .collect();
        return Ok(result);
    }

    /// Start a transaction.
    pub fn start_transaction(&mut self) -> KVResult<TransactionId> {
        let transaction_id = self.transaction_manager.generate_new_transaction_id()?;
        let instruction = Instruction::TransactionStart { transaction_id };
        self.writer.append_instruction(&instruction)?;

        let snapshot = self.snapshot.clone();
        self.transaction_manager
            .initialize_transaction(&transaction_id, snapshot);

        self.current_height.increment()?;

        return Ok(transaction_id);
    }

    /// Commit a transaction.
    pub fn commit_transaction(&mut self, transaction_id: TransactionId) -> KVResult<()> {
        self.transaction_manager
            .validate_transaction_id(&transaction_id)?;

        let instruction = Instruction::TransactionCommit { transaction_id };
        self.writer.append_instruction(&instruction)?;

        update_committed_log_pointers(
            &mut self.transaction_manager,
            &mut self.snapshot,
            transaction_id.clone(),
        );

        self.current_height.increment()?;

        return Ok(());
    }

    /// Abort a transaction.
    pub fn abort_transaction(&mut self, transaction_id: &TransactionId) -> KVResult<()> {
        self.transaction_manager
            .validate_transaction_id(&transaction_id)?;

        let instruction = Instruction::TransactionAbort {
            transaction_id: transaction_id.clone(),
        };
        self.writer.append_instruction(&instruction)?;

        update_aborted_log_pointers(
            &mut self.transaction_manager,
            &mut self.snapshot,
            *transaction_id,
        );

        self.current_height.increment()?;

        return Ok(());
    }
}

fn get_revert_value(
    reader: &mut LogReader,
    key: &KVKey,
    height: &ChainHeight,
) -> KVResult<Option<KVValue>> {
    let (instruction_iterator, _) = reader.read_all()?;

    let instructions: Vec<Instruction> = instruction_iterator
        .map(|(instruction, _)| instruction)
        .collect();
    let value = recursive_find(&key, &instructions, &height)?;

    return Ok(value);
}

fn recursive_find(
    target_key: &KVKey,
    instructions: &Vec<Instruction>,
    target_height: &ChainHeight,
) -> KVResult<Option<KVValue>> {
    let target_instruction = &instructions.as_slice()[target_height.as_u64() as usize];

    match target_instruction {
        Instruction::Set { key, value } => {
            if target_key == key {
                Ok(Some(value.to_owned()))
            } else {
                let next_target_height = &target_height.clone().decrement()?;
                recursive_find(&target_key, &instructions, &next_target_height)
            }
        }
        Instruction::RevertOne { key, height } => {
            if target_key == key {
                recursive_find(&target_key, &instructions, height)
            } else {
                let next_target_height = &target_height.clone().decrement()?;
                recursive_find(&target_key, &instructions, &next_target_height)
            }
        }
        Instruction::RevertAll { height } => {
            return recursive_find(&target_key, &instructions, height);
        }
        Instruction::RemoveOne { key } => {
            if target_key == key {
                Ok(None)
            } else {
                let next_target_height = &target_height.clone().decrement()?;
                recursive_find(&target_key, &instructions, next_target_height)
            }
        }
        Instruction::RemoveAll => Ok(None),
        _ => {
            let next_target_height = &target_height.clone().decrement()?;
            recursive_find(&target_key, &instructions, &next_target_height)
        }
    }
}

fn load_snapshot(
    mut reader: &mut LogReader,
    target_height: Option<&ChainHeight>,
) -> KVResult<(Snapshot, ChainHeight, TransactionManager)> {
    let (instruction_iterator, header_offset) = reader.read_all()?;
    let mut current_position = header_offset as u64;
    let mut height = ChainHeight::new(0);
    let mut snapshot: HashMap<KVKey, HashMap<Option<TransactionId>, LogPointer>> = HashMap::new();
    let mut transaction_manager = TransactionManager::new();

    for (instruction, instruction_length) in instruction_iterator {
        match instruction {
            Instruction::Set { key, value: _ } => {
                let log_pointer = LogPointer::new(current_position, instruction_length);
                update_snapshot(&key, &log_pointer, &mut snapshot, None);
            }
            Instruction::RevertOne { key, height: _ } => {
                let log_pointer = LogPointer::new(current_position, instruction_length);
                update_snapshot(&key, &log_pointer, &mut snapshot, None);
            }
            Instruction::RevertAll { height } => {
                let (new_snapshot, _, _) = load_snapshot(&mut reader, Some(&height))?;
                transaction_manager = TransactionManager::new();
                snapshot = new_snapshot;
            }
            Instruction::RemoveOne { key } => {
                let log_pointer = LogPointer::new(current_position, instruction_length);
                update_snapshot(&key, &log_pointer, &mut snapshot, None);
            }
            Instruction::RemoveAll => {
                for log_pointers in snapshot.values_mut() {
                    log_pointers.remove(&None);
                }
            }
            Instruction::TransactionStart { transaction_id } => {
                let snapshot = snapshot.clone();
                transaction_manager.initialize_transaction(&transaction_id, snapshot);
                transaction_manager.update_transaction_id(&transaction_id);
            }
            Instruction::TransactionalSet {
                key,
                value: _,
                transaction_id,
            } => {
                let log_pointer = LogPointer::new(current_position, instruction_length);
                update_snapshot(&key, &log_pointer, &mut snapshot, Some(transaction_id));
                transaction_manager.add_affected_keys(&transaction_id, &key)?;
            }
            Instruction::TransactionalRevertOne {
                key,
                height: _,
                transaction_id,
            } => {
                let log_pointer = LogPointer::new(current_position, instruction_length);
                update_snapshot(&key, &log_pointer, &mut snapshot, Some(transaction_id));

                transaction_manager.add_affected_keys(&transaction_id, &key)?;
            }
            Instruction::TransactionalRemoveOne {
                key,
                transaction_id,
            } => {
                let log_pointer = LogPointer::new(current_position, instruction_length);
                update_snapshot(&key, &log_pointer, &mut snapshot, Some(transaction_id));

                transaction_manager.add_affected_keys(&transaction_id, &key)?;
            }
            Instruction::TransactionCommit { transaction_id } => {
                update_committed_log_pointers(
                    &mut transaction_manager,
                    &mut snapshot,
                    transaction_id,
                );
            }
            Instruction::TransactionAbort { transaction_id } => {
                update_aborted_log_pointers(
                    &mut transaction_manager,
                    &mut snapshot,
                    transaction_id,
                );
            }
        }

        current_position += instruction_length as u64;

        if let Some(target_height) = target_height {
            if target_height == &height {
                break;
            }
        }

        height.increment()?;
    }

    return Ok((snapshot, height, transaction_manager));
}

fn update_committed_log_pointers(
    transaction_manager: &mut TransactionManager,
    snapshot: &mut Snapshot,
    transaction_id: TransactionId,
) {
    let affected_keys = transaction_manager.get_affected_keys(&transaction_id);
    for key in affected_keys {
        if let Some(log_pointers) = snapshot.get_mut(&key) {
            if let Some(target_log_pointer) = log_pointers.get(&Some(transaction_id)).cloned() {
                log_pointers.insert(None, target_log_pointer);
                log_pointers.remove(&Some(transaction_id));
            }
        }
    }

    transaction_manager.remove_transaction(&transaction_id);
}

fn update_aborted_log_pointers(
    transaction_manager: &mut TransactionManager,
    snapshot: &mut Snapshot,
    transaction_id: TransactionId,
) {
    let affected_keys = transaction_manager.get_affected_keys(&transaction_id);
    for key in affected_keys {
        if let Some(log_pointers) = snapshot.get_mut(&key) {
            log_pointers.remove(&Some(transaction_id));
        }
    }

    transaction_manager.remove_transaction(&transaction_id);
}

fn update_snapshot(
    key: &KVKey,
    log_pointer: &LogPointer,
    snapshot: &mut Snapshot,
    transaction_id: Option<TransactionId>,
) {
    if let Some(log_pointers) = snapshot.get_mut(&key) {
        log_pointers.insert(transaction_id, log_pointer.clone());
    } else {
        let mut log_pointers = HashMap::new();
        log_pointers.insert(transaction_id, log_pointer.to_owned());
        snapshot.insert(key.clone(), log_pointers);
    }
}

pub fn get_main_log_full_path(data_dir: &PathBuf) -> PathBuf {
    return data_dir.join(Path::new(&Constants::MAIN_LOG_FILENAME));
}
