use std::collections::{HashMap, HashSet};
use std::fs::{create_dir_all, File, OpenOptions};
use std::io::Read;
use std::io::{BufReader, BufWriter, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

use crate::constants as Constants;
use crate::storage::buffer_parser::CommandBufferParser;
use crate::storage::chain_height::ChainHeight;
use crate::storage::command::{Command, CommandError};
use crate::storage::errors::{KVError, KVResult};
use crate::storage::kvkey::KVKey;
use crate::storage::kvvalue::KVValue;
use crate::storage::log_pointer::LogPointer;
use crate::storage::transaction_manager::{TransactionId, TransactionManager};

pub struct LogKeyValueStore {
    reader: BufReader<File>,
    writer: BufWriter<File>,
    key_pointer_map: HashMap<KVKey, HashMap<Option<TransactionId>, LogPointer>>,
    current_height: ChainHeight,
    transaction_manager: TransactionManager,
}

impl LogKeyValueStore {
    pub fn open(path: &PathBuf) -> KVResult<LogKeyValueStore> {
        create_dir_all(&path)?;

        let log_file_path = get_log_file_dir(&path);

        let writer_file_option = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_file_path.to_owned())?;
        let writer = BufWriter::new(writer_file_option);

        let reader_file_option = OpenOptions::new()
            .read(true)
            .open(&log_file_path.to_owned())?;
        let mut reader = BufReader::new(reader_file_option);

        let (key_pointer_map, current_height, transaction_manager) =
            load_key_pointer_map(&mut reader, None)?;

        let engine = LogKeyValueStore {
            reader,
            writer,
            key_pointer_map,
            current_height,
            transaction_manager,
        };

        return Ok(engine);
    }

    pub fn set(
        &mut self,
        key: &KVKey,
        value: &KVValue,
        transaction_id: Option<TransactionId>,
    ) -> KVResult<()> {
        if key.as_bytes().len() > Constants::MAX_KEY_LENGTH {
            return Err(CommandError::KeyExceedsMaxLength.into());
        }

        let command = {
            if let Some(transaction_id) = transaction_id {
                self.transaction_manager
                    .validate_transaction_id(&transaction_id)?;
                self.transaction_manager
                    .add_affected_keys(&transaction_id, &key);

                Command::TransactionalSet {
                    key: key.clone(),
                    value: value.clone(),
                    transaction_id,
                }
            } else {
                Command::Set {
                    key: key.clone(),
                    value: value.clone(),
                }
            }
        };

        let log_pointer = append_command(command, &mut self.writer)?;

        self.current_height.increment()?;

        update_key_pointer_map(key, &log_pointer, &mut self.key_pointer_map, transaction_id);

        return Ok(());
    }

    pub fn get(
        &mut self,
        key: &KVKey,
        transaction_id: Option<TransactionId>,
    ) -> KVResult<Option<KVValue>> {
        match self.key_pointer_map.get(&key) {
            None => Ok(None),
            Some(log_pointers) => {
                if let Some(log_pointer) = log_pointers.get(&transaction_id) {
                    self.reader.seek(SeekFrom::Start(log_pointer.pos))?;

                    let mut log_pointer_reader = self.reader.by_ref().take(log_pointer.len as u64);

                    let mut buffer = vec![0; log_pointer.len];
                    log_pointer_reader.read(&mut buffer)?;

                    let (command, _) = Command::try_from(buffer.as_slice())?;

                    match command {
                        Command::Set { key: _, value } => Ok(Some(value)),
                        Command::RevertOne { key, height } => self.get_revert_value(&key, &height),
                        Command::RemoveOne { key: _ } => Ok(None),
                        Command::TransactionalSet {
                            key: _,
                            value,
                            transaction_id: _,
                        } => Ok(Some(value)),
                        Command::TransactionalRevertOne {
                            key,
                            height,
                            transaction_id: _,
                        } => self.get_revert_value(&key, &height),
                        Command::TransactionalRemoveOne {
                            key: _,
                            transaction_id: _,
                        } => Ok(None),
                        _ => Err(KVError::PointToUnexpectedCommand),
                    }
                } else {
                    return if transaction_id.is_some() {
                        self.get(&key, None)
                    } else {
                        Ok(None)
                    };
                }
            }
        }
    }

    pub fn revert_one(
        &mut self,
        key: &KVKey,
        height: &ChainHeight,
        transaction_id: Option<TransactionId>,
    ) -> KVResult<()> {
        if key.as_bytes().len() > Constants::MAX_KEY_LENGTH {
            return Err(CommandError::KeyExceedsMaxLength.into());
        }

        if height > &self.current_height {
            return Err(KVError::RevertOutOfRange);
        }

        let command = {
            if let Some(transaction_id) = transaction_id {
                Command::TransactionalRevertOne {
                    key: key.clone(),
                    height: height.clone(),
                    transaction_id,
                }
            } else {
                Command::RevertOne {
                    key: key.clone(),
                    height: height.clone(),
                }
            }
        };

        if let Some(transaction_id) = transaction_id {
            self.transaction_manager
                .validate_transaction_id(&transaction_id)?;
            self.transaction_manager
                .add_affected_keys(&transaction_id, &key);
        }

        let log_pointer = append_command(command, &mut self.writer)?;

        self.current_height.increment()?;

        update_key_pointer_map(key, &log_pointer, &mut self.key_pointer_map, transaction_id);

        return Ok(());
    }

    pub fn revert_all(&mut self, height: &ChainHeight) -> KVResult<()> {
        if height > &self.current_height {
            return Err(KVError::RevertOutOfRange);
        }

        let command = Command::RevertAll {
            height: height.clone(),
        };

        append_command(command, &mut self.writer)?;

        self.current_height.increment()?;
        let (new_key_pointer_map, _, _) = load_key_pointer_map(&mut self.reader, Some(height))?;
        self.key_pointer_map = new_key_pointer_map;
        self.transaction_manager = TransactionManager::new();

        return Ok(());
    }

    pub fn remove_one(
        &mut self,
        key: &KVKey,
        transaction_id: Option<TransactionId>,
    ) -> KVResult<()> {
        if key.as_bytes().len() > Constants::MAX_KEY_LENGTH {
            return Err(CommandError::KeyExceedsMaxLength.into());
        }

        let command = {
            if let Some(transaction_id) = transaction_id {
                Command::TransactionalRemoveOne {
                    key: key.clone(),
                    transaction_id,
                }
            } else {
                Command::RemoveOne { key: key.clone() }
            }
        };

        let log_pointer = append_command(command, &mut self.writer)?;

        if let Some(transaction_id) = transaction_id {
            self.transaction_manager
                .validate_transaction_id(&transaction_id)?;
            self.transaction_manager
                .add_affected_keys(&transaction_id, &key);
        }

        self.current_height.increment()?;

        update_key_pointer_map(key, &log_pointer, &mut self.key_pointer_map, transaction_id);

        return Ok(());
    }

    pub fn remove_all(&mut self) -> KVResult<()> {
        let command = Command::RemoveAll;

        append_command(command, &mut self.writer)?;

        for log_pointers in self.key_pointer_map.values_mut() {
            log_pointers.remove(&None);
        }

        self.current_height.increment()?;

        return Ok(());
    }

    pub fn inspect_all(&mut self) -> KVResult<Vec<(Command, ChainHeight)>> {
        let mut command_buffer: Vec<u8> = Vec::new();
        self.reader.seek(SeekFrom::Start(0))?;
        self.reader.read_to_end(&mut command_buffer)?;

        let command_buffer_parser = CommandBufferParser::new(&command_buffer, 0);

        let commands_with_height = command_buffer_parser
            .enumerate()
            .map(|(index, (command, _))| (command, ChainHeight::new((index + 1) as u64)))
            .collect();
        return Ok(commands_with_height);
    }

    pub fn inspect_one(&mut self, target_key: &KVKey) -> KVResult<Vec<(Command, ChainHeight)>> {
        let mut command_buffer: Vec<u8> = Vec::new();
        self.reader.seek(SeekFrom::Start(0))?;
        self.reader.read_to_end(&mut command_buffer)?;

        let command_buffer_parser = CommandBufferParser::new(&command_buffer, 0);

        let mut appeared_key = HashSet::new();
        let ret = command_buffer_parser
            .enumerate()
            .filter_map(|(index, (command, _))| match &command {
                Command::Set { key, value: _ } => {
                    appeared_key.insert(key.clone());

                    if target_key == key {
                        return Some((command, ChainHeight::new((index + 1) as u64)));
                    } else {
                        return None;
                    }
                }
                Command::RevertOne { key, height: _ } => {
                    appeared_key.insert(key.clone());

                    if target_key == key {
                        return Some((command, ChainHeight::new((index + 1) as u64)));
                    } else {
                        return None;
                    }
                }
                Command::RevertAll { height: _ } => {
                    if appeared_key.contains(&target_key) {
                        return Some((command, ChainHeight::new((index + 1) as u64)));
                    } else {
                        return None;
                    }
                }
                Command::RemoveOne { key } => {
                    appeared_key.insert(key.clone());

                    if target_key == key {
                        return Some((command, ChainHeight::new((index + 1) as u64)));
                    } else {
                        return None;
                    }
                }
                Command::RemoveAll => {
                    if appeared_key.contains(&target_key) {
                        return Some((command, ChainHeight::new((index + 1) as u64)));
                    } else {
                        return None;
                    }
                }
                _ => None,
            })
            .collect();
        return Ok(ret);
    }

    pub fn start_transaction(&mut self) -> KVResult<TransactionId> {
        let transaction_id = self.transaction_manager.generate_new_transaction_id()?;
        let command = Command::TransactionStart { transaction_id };
        append_command(command, &mut self.writer)?;

        self.transaction_manager
            .initialize_affected_keys(&transaction_id);

        self.current_height.increment()?;

        return Ok(transaction_id);
    }

    pub fn commit_transaction(&mut self, transaction_id: TransactionId) -> KVResult<()> {
        self.transaction_manager
            .validate_transaction_id(&transaction_id)?;

        let command = Command::TransactionCommit { transaction_id };
        append_command(command, &mut self.writer)?;

        update_committed_log_pointers(
            &mut self.transaction_manager,
            &mut self.key_pointer_map,
            transaction_id,
        );

        self.current_height.increment()?;

        return Ok(());
    }

    pub fn abort_transaction(&mut self, transaction_id: TransactionId) -> KVResult<()> {
        self.transaction_manager
            .validate_transaction_id(&transaction_id)?;

        let command = Command::TransactionAbort { transaction_id };
        append_command(command, &mut self.writer)?;

        update_aborted_log_pointers(
            &mut self.transaction_manager,
            &mut self.key_pointer_map,
            transaction_id,
        );

        self.current_height.increment()?;

        return Ok(());
    }

    fn get_revert_value(&mut self, key: &KVKey, height: &ChainHeight) -> KVResult<Option<KVValue>> {
        let mut command_buffer: Vec<u8> = Vec::new();
        self.reader.seek(SeekFrom::Start(0))?;
        self.reader.read_to_end(&mut command_buffer)?;
        let command_buffer_parser = CommandBufferParser::new(&command_buffer, 0);

        let commands: Vec<Command> = command_buffer_parser.map(|(command, _)| command).collect();
        let value = recursive_find(&key, &commands, &height)?;

        return Ok(value);
    }
}

fn recursive_find(
    target_key: &KVKey,
    commands: &Vec<Command>,
    target_height: &ChainHeight,
) -> KVResult<Option<KVValue>> {
    let target_command = &commands.as_slice()[target_height.as_u64() as usize];

    match target_command {
        Command::Set { key, value } => {
            if target_key == key {
                return Ok(Some(value.to_owned()));
            } else {
                let next_target_height = &target_height.clone().decrement()?;
                return recursive_find(&target_key, &commands, &next_target_height);
            }
        }
        Command::RevertOne { key, height } => {
            if target_key == key {
                return recursive_find(&target_key, &commands, height);
            } else {
                let next_target_height = &target_height.clone().decrement()?;
                return recursive_find(&target_key, &commands, &next_target_height);
            }
        }
        Command::RevertAll { height } => {
            return recursive_find(&target_key, &commands, height);
        }
        Command::RemoveOne { key } => {
            if target_key == key {
                return Ok(None);
            } else {
                let next_target_height = &target_height.clone().decrement()?;
                return recursive_find(&target_key, &commands, next_target_height);
            }
        }
        Command::RemoveAll => {
            return Ok(None);
        }
        _ => {
            let next_target_height = &target_height.clone().decrement()?;
            return recursive_find(&target_key, &commands, &next_target_height);
        }
    }
}

fn load_key_pointer_map(
    mut reader: &mut BufReader<File>,
    target_height: Option<&ChainHeight>,
) -> KVResult<(
    HashMap<KVKey, HashMap<Option<TransactionId>, LogPointer>>,
    ChainHeight,
    TransactionManager,
)> {
    let mut command_buffer: Vec<u8> = Vec::new();
    let mut current_position = 0;
    let mut height = ChainHeight::new(0);
    let mut key_pointer_map: HashMap<KVKey, HashMap<Option<TransactionId>, LogPointer>> =
        HashMap::new();
    let mut transaction_manager = TransactionManager::new();

    reader.seek(SeekFrom::Start(0))?;
    reader.read_to_end(&mut command_buffer)?;
    let command_buffer_parser = CommandBufferParser::new(&command_buffer, 0);

    for (command, command_length) in command_buffer_parser {
        match command {
            Command::Set { key, value: _ } => {
                let log_pointer = LogPointer::new(current_position, command_length);
                update_key_pointer_map(&key, &log_pointer, &mut key_pointer_map, None);
            }
            Command::RevertOne { key, height: _ } => {
                let log_pointer = LogPointer::new(current_position, command_length);
                update_key_pointer_map(&key, &log_pointer, &mut key_pointer_map, None);
            }
            Command::RevertAll { height } => {
                let (new_key_pointer_map, _, _) = load_key_pointer_map(&mut reader, Some(&height))?;
                transaction_manager = TransactionManager::new();
                key_pointer_map = new_key_pointer_map;
            }
            Command::RemoveOne { key } => {
                let log_pointer = LogPointer::new(current_position, command_length);
                update_key_pointer_map(&key, &log_pointer, &mut key_pointer_map, None);
            }
            Command::RemoveAll => {
                for log_pointers in key_pointer_map.values_mut() {
                    log_pointers.remove(&None);
                }
            }
            Command::TransactionStart { transaction_id } => {
                transaction_manager.initialize_affected_keys(&transaction_id);
                transaction_manager.update_transaction_id(&transaction_id);
            }
            Command::TransactionalSet {
                key,
                value: _,
                transaction_id,
            } => {
                let log_pointer = LogPointer::new(current_position, command_length);
                update_key_pointer_map(
                    &key,
                    &log_pointer,
                    &mut key_pointer_map,
                    Some(transaction_id),
                );
                transaction_manager.add_affected_keys(&transaction_id, &key);
            }
            Command::TransactionalRevertOne {
                key,
                height: _,
                transaction_id,
            } => {
                let log_pointer = LogPointer::new(current_position, command_length);
                update_key_pointer_map(
                    &key,
                    &log_pointer,
                    &mut key_pointer_map,
                    Some(transaction_id),
                );

                transaction_manager.add_affected_keys(&transaction_id, &key);
            }
            Command::TransactionalRemoveOne {
                key,
                transaction_id,
            } => {
                let log_pointer = LogPointer::new(current_position, command_length);
                update_key_pointer_map(
                    &key,
                    &log_pointer,
                    &mut key_pointer_map,
                    Some(transaction_id),
                );

                transaction_manager.add_affected_keys(&transaction_id, &key);
            }
            Command::TransactionCommit { transaction_id } => {
                update_committed_log_pointers(
                    &mut transaction_manager,
                    &mut key_pointer_map,
                    transaction_id,
                );
            }
            Command::TransactionAbort { transaction_id } => {
                update_aborted_log_pointers(
                    &mut transaction_manager,
                    &mut key_pointer_map,
                    transaction_id,
                );
            }
        }

        current_position += command_length as u64;

        if let Some(target_height) = target_height {
            if target_height == &height {
                break;
            }
        }

        height.increment()?;
    }

    return Ok((key_pointer_map, height, transaction_manager));
}

fn update_committed_log_pointers(
    transaction_manager: &mut TransactionManager,
    key_pointer_map: &mut HashMap<KVKey, HashMap<Option<TransactionId>, LogPointer>>,
    transaction_id: TransactionId,
) {
    let affected_keys = transaction_manager.get_affected_keys(&transaction_id);
    for key in affected_keys {
        if let Some(log_pointers) = key_pointer_map.get_mut(&key) {
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
    key_pointer_map: &mut HashMap<KVKey, HashMap<Option<TransactionId>, LogPointer>>,
    transaction_id: TransactionId,
) {
    let affected_keys = transaction_manager.get_affected_keys(&transaction_id);
    for key in affected_keys {
        if let Some(log_pointers) = key_pointer_map.get_mut(&key) {
            log_pointers.remove(&Some(transaction_id));
        }
    }

    transaction_manager.remove_transaction(&transaction_id);
}

fn update_key_pointer_map(
    key: &KVKey,
    log_pointer: &LogPointer,
    key_pointer_map: &mut HashMap<KVKey, HashMap<Option<TransactionId>, LogPointer>>,
    transaction_id: Option<TransactionId>,
) {
    if let Some(log_pointers) = key_pointer_map.get_mut(&key) {
        log_pointers.insert(transaction_id, log_pointer.clone());
    } else {
        let mut log_pointers = HashMap::new();
        log_pointers.insert(transaction_id, log_pointer.to_owned());
        key_pointer_map.insert(key.clone(), log_pointers);
    }
}

fn append_command(command: Command, writer: &mut BufWriter<File>) -> KVResult<LogPointer> {
    let command_bytes: Vec<u8> = command.serialize();

    let current_pos = writer.seek(SeekFrom::Current(0))?;
    writer.write_all(command_bytes.as_slice())?;
    writer.flush()?;

    let log_pointer = LogPointer::new(current_pos, command_bytes.len());

    return Ok(log_pointer);
}

pub fn get_log_file_dir(dir: &PathBuf) -> PathBuf {
    let log_file_name = format!("{}.log", Constants::LOG_FILE_NAME);
    let log_path = dir.join(Path::new(&log_file_name));
    return log_path;
}
