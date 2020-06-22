use std::collections::{HashMap, HashSet};
use std::convert::TryInto;
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

pub struct LogKeyValueStore {
    reader: BufReader<File>,
    writer: BufWriter<File>,
    key_pointer_map: HashMap<KVKey, LogPointer>,
    current_height: ChainHeight,
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

        let (key_pointer_map, current_height) = load_key_pointer_map(&mut reader, None)?;

        let engine = LogKeyValueStore {
            reader,
            writer,
            key_pointer_map,
            current_height,
        };

        return Ok(engine);
    }

    pub fn set(&mut self, key: KVKey, value: KVValue) -> KVResult<()> {
        if key.as_bytes().len() > Constants::MAX_KEY_LENGTH {
            return Err(CommandError::KeyExceedsMaxLength.into());
        }

        let command = Command::Set {
            key: key.clone(),
            value,
        };

        let log_pointer = append_command(command, &mut self.writer)?;

        self.current_height.increment()?;

        self.key_pointer_map.insert(key, log_pointer);

        return Ok(());
    }

    pub fn get(&mut self, key: &KVKey) -> KVResult<Option<KVValue>> {
        match self.key_pointer_map.get(&key) {
            None => Ok(None),
            Some(log_pointer) => {
                self.reader.seek(SeekFrom::Start(log_pointer.pos))?;

                let mut log_pointer_reader = self.reader.by_ref().take(log_pointer.len as u64);

                let mut buffer = vec![0; log_pointer.len];
                log_pointer_reader.read(&mut buffer)?;

                let (command, _) = Command::try_from(buffer.as_slice())?;

                match command {
                    Command::Set { key: _, value } => {
                        return Ok(Some(value));
                    }
                    Command::RevertOne { key, height } => {
                        let mut command_buffer: Vec<u8> = Vec::new();
                        self.reader.seek(SeekFrom::Start(0))?;
                        self.reader.read_to_end(&mut command_buffer)?;
                        let command_buffer_parser = CommandBufferParser::new(&command_buffer, 0);

                        let commands: Vec<Command> =
                            command_buffer_parser.map(|(command, _)| command).collect();
                        let res = recursive_find(&key, &commands, &height)?;

                        return Ok(res);
                    }
                    _ => Err(KVError::PointToUnexpectedCommand),
                }
            }
        }
    }

    pub fn revert_one(&mut self, key: KVKey, height: &ChainHeight) -> KVResult<()> {
        if key.as_bytes().len() > Constants::MAX_KEY_LENGTH {
            return Err(CommandError::KeyExceedsMaxLength.into());
        }

        if height > &self.current_height {
            return Err(KVError::RevertOutOfRange);
        }

        let command = Command::RevertOne {
            key: key.clone(),
            height: height.clone(),
        };

        let log_pointer = append_command(command, &mut self.writer)?;
        self.current_height.increment()?;
        self.key_pointer_map.insert(key, log_pointer);

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
        let (new_key_pointer_map, _) = load_key_pointer_map(&mut self.reader, Some(height))?;
        self.key_pointer_map = new_key_pointer_map;

        return Ok(());
    }

    pub fn remove(&mut self, key: KVKey) -> KVResult<()> {
        if key.as_bytes().len() > Constants::MAX_KEY_LENGTH {
            return Err(CommandError::KeyExceedsMaxLength.into());
        }

        let command = Command::Remove { key: key.clone() };

        append_command(command, &mut self.writer)?;

        self.current_height.increment()?;
        self.key_pointer_map.remove(&key);

        return Ok(());
    }

    pub fn remove_all(&mut self) -> KVResult<()> {
        let command = Command::RemoveAll;

        append_command(command, &mut self.writer)?;

        self.current_height.increment()?;
        self.key_pointer_map.clear();

        return Ok(());
    }

    pub fn inspect(&mut self, key: Option<&KVKey>) -> KVResult<Vec<Command>> {
        let mut command_buffer: Vec<u8> = Vec::new();
        self.reader.seek(SeekFrom::Start(0))?;
        self.reader.read_to_end(&mut command_buffer)?;

        let command_buffer_parser = CommandBufferParser::new(&command_buffer, 0);
        match key {
            None => {
                let commands = command_buffer_parser.map(|(command, _)| command).collect();
                return Ok(commands);
            }
            Some(target_key) => {
                let mut appeared_key = HashSet::new();
                let ret = command_buffer_parser
                    .filter_map(|(command, _)| match &command {
                        Command::Set { key, value: _ } => {
                            appeared_key.insert(key.clone());

                            if target_key == key {
                                return Some(command);
                            } else {
                                return None;
                            }
                        }
                        Command::RevertOne { key, height: _ } => {
                            appeared_key.insert(key.clone());

                            if target_key == key {
                                return Some(command);
                            } else {
                                return None;
                            }
                        }
                        Command::RevertAll { height: _ } => {
                            if appeared_key.contains(&target_key) {
                                return Some(command);
                            } else {
                                return None;
                            }
                        }
                        Command::Remove { key } => {
                            appeared_key.insert(key.clone());

                            if target_key == key {
                                return Some(command);
                            } else {
                                return None;
                            }
                        }
                        Command::RemoveAll => {
                            if appeared_key.contains(&target_key) {
                                return Some(command);
                            } else {
                                return None;
                            }
                        }
                    })
                    .collect();
                return Ok(ret);
            }
        }
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
        Command::Remove { key } => {
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
    }
}

fn load_key_pointer_map(
    mut reader: &mut BufReader<File>,
    target_height: Option<&ChainHeight>,
) -> KVResult<(HashMap<KVKey, LogPointer>, ChainHeight)> {
    let mut command_buffer: Vec<u8> = Vec::new();
    let mut current_position = 0;
    let mut height = ChainHeight::new(0);
    let mut key_pointer_map: HashMap<KVKey, LogPointer> = HashMap::new();

    reader.seek(SeekFrom::Start(0))?;
    reader.read_to_end(&mut command_buffer)?;
    let command_buffer_parser = CommandBufferParser::new(&command_buffer, 0);

    for (command, command_length) in command_buffer_parser {
        match command {
            Command::Set { key, value: _ } => {
                let log_pointer = LogPointer::new(current_position, command_length);
                key_pointer_map.insert(key, log_pointer);
            }
            Command::RevertOne { key, height: _ } => {
                let log_pointer = LogPointer::new(current_position, command_length);
                key_pointer_map.insert(key, log_pointer);
            }
            Command::RevertAll { height } => {
                let (new_key_pointer_map, _) = load_key_pointer_map(&mut reader, Some(&height))?;
                key_pointer_map = new_key_pointer_map;
            }
            Command::Remove { key } => {
                key_pointer_map.remove(&key);
            }
            Command::RemoveAll => {
                key_pointer_map.clear();
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

    return Ok((key_pointer_map, height));
}

fn append_command(command: Command, writer: &mut BufWriter<File>) -> KVResult<LogPointer> {
    let command_bytes: Vec<u8> = command.try_into()?;

    let current_pos = writer.seek(SeekFrom::Current(0))?;
    writer.write_all(command_bytes.as_slice())?;
    writer.flush()?;

    let log_pointer = LogPointer::new(current_pos, command_bytes.len());

    return Ok(log_pointer);
}

fn get_log_file_dir(dir: &PathBuf) -> PathBuf {
    let log_file_name = format!("{}.log", Constants::LOG_FILE_NAME);
    let log_path = dir.join(Path::new(&log_file_name));
    return log_path;
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::fs::remove_file;
    use std::path::PathBuf;

    use crate::storage::chain_height::ChainHeight;
    use crate::storage::kv::{get_log_file_dir, LogKeyValueStore};
    use crate::storage::kvkey::KVKey;
    use crate::storage::kvvalue::KVValue;

    fn get_store_engine(path: &PathBuf) -> LogKeyValueStore {
        let log_file_path = get_log_file_dir(&path);

        if log_file_path.exists() {
            remove_file(log_file_path).unwrap();
        }

        let store_engine = LogKeyValueStore::open(&path).unwrap();

        return store_engine;
    }

    #[test]
    fn kv_set() {
        let path = PathBuf::from("/tmp/test_set");
        let mut store_engine = get_store_engine(&path);

        let key = KVKey::new(&[0x00, 0x01, 0x03]);
        let expected_value = KVValue::new(&[0xff, 0xff, 0xff, 0xff]);

        store_engine
            .set(key.clone(), expected_value.clone())
            .unwrap();
        let actual_value = store_engine.get(&key).unwrap().unwrap();

        assert_eq!(actual_value, expected_value);
    }

    #[test]
    fn kv_revert_one() {
        let path = PathBuf::from("/tmp/test_revert_one");
        let mut store_engine = get_store_engine(&path);
        let target_height = ChainHeight::new(2);

        let key = KVKey::new(&[0x00, 0x01, 0x03]);
        let values: Vec<KVValue> = vec![
            KVValue::new(&[0x00]),
            KVValue::new(&[0x01]),
            KVValue::new(&[0x02]),
            KVValue::new(&[0x03]),
            KVValue::new(&[0x04]),
            KVValue::new(&[0x05]),
        ];

        for value in &values {
            store_engine.set(key.clone(), value.clone()).unwrap();
        }

        store_engine
            .revert_one(key.clone(), &target_height)
            .unwrap();

        let actual_output = store_engine.get(&key).unwrap().unwrap();
        let expected_output = &values.as_slice()[target_height.as_u64() as usize];

        assert_eq!(&actual_output, expected_output);
    }

    #[test]
    fn kv_revert_all() {
        let path = PathBuf::from("/tmp/test_revert_all");
        let mut store_engine = get_store_engine(&path);
        let target_height = ChainHeight::new(5);

        let key_value_pairs = vec![
            (KVKey::new(&[0x00]), KVValue::new(&[0x00])),
            (KVKey::new(&[0x00]), KVValue::new(&[0xff])),
            (KVKey::new(&[0x00]), KVValue::new(&[0x22])),
            (KVKey::new(&[0x01]), KVValue::new(&[0x01])),
            (KVKey::new(&[0x00]), KVValue::new(&[0x19])),
            (KVKey::new(&[0x02]), KVValue::new(&[0x02])),
            (KVKey::new(&[0x03]), KVValue::new(&[0x03])),
            (KVKey::new(&[0x04]), KVValue::new(&[0x04])),
            (KVKey::new(&[0x05]), KVValue::new(&[0x05])),
        ];

        for kv_pair in &key_value_pairs {
            store_engine
                .set(kv_pair.0.clone(), kv_pair.1.clone())
                .unwrap();
        }

        store_engine.revert_all(&target_height).unwrap();

        let mut expected_hashmap = HashMap::new();

        for expected_kv_pair in &key_value_pairs[..target_height.as_u64() as usize] {
            expected_hashmap.insert(&expected_kv_pair.0, &expected_kv_pair.1);
        }

        for (key, expected_value) in expected_hashmap {
            let actual_value = store_engine.get(&key).unwrap().unwrap();
            assert_eq!(expected_value, &actual_value);
        }
    }

    #[test]
    fn kv_remove() {
        let path = PathBuf::from("/tmp/remove");
        let mut store_engine = get_store_engine(&path);

        let key = KVKey::new(&[0x00, 0x01, 0x03]);
        let expected_value = KVValue::new(&[0xff, 0xff, 0xff, 0xff]);

        store_engine
            .set(key.clone(), expected_value.clone())
            .unwrap();
        store_engine.remove(key.clone()).unwrap();

        let actual_value = store_engine.get(&key).unwrap();

        assert_eq!(actual_value, None);
    }

    #[test]
    fn kv_remove_all() {
        let path = PathBuf::from("/tmp/remove_all");
        let mut store_engine = get_store_engine(&path);

        let key_value_pairs = vec![
            (KVKey::new(&[0xff]), KVValue::new(&[0x00])),
            (KVKey::new(&[0xf2]), KVValue::new(&[0xff])),
            (KVKey::new(&[0x23]), KVValue::new(&[0x22])),
            (KVKey::new(&[0x11]), KVValue::new(&[0x01])),
        ];

        for kv_pair in &key_value_pairs {
            store_engine
                .set(kv_pair.0.clone(), kv_pair.1.clone())
                .unwrap();
        }

        store_engine.remove_all().unwrap();

        for kv_pair in &key_value_pairs {
            let out_put = store_engine.get(&kv_pair.0).unwrap();
            assert_eq!(out_put, None);
        }
    }
}
