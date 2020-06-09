use std::collections::{HashMap, HashSet};
use std::fs::{create_dir_all, File, OpenOptions};
use std::io::Read;
use std::io::{BufReader, BufWriter, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

use crate::constants as Constants;
use crate::storage::buffer_parser::CommandBufferParser;
use crate::storage::command::Command;
use crate::storage::errors::{KVError, KVResult};
use crate::storage::log_pointer::LogPointer;
use std::convert::TryInto;

pub struct KeyValueEngine {
    reader: BufReader<File>,
    writer: BufWriter<File>,
    index: HashMap<Vec<u8>, LogPointer>,
    current_height: u64,
}

impl KeyValueEngine {
    pub fn open(path: &PathBuf) -> KVResult<KeyValueEngine> {
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

        let mut index: HashMap<Vec<u8>, LogPointer> = HashMap::new();
        let current_height = load_index(&mut reader, &mut index, None)?;

        let engine = KeyValueEngine {
            reader,
            writer,
            index,
            current_height,
        };

        return Ok(engine);
    }

    pub fn set(&mut self, key: Vec<u8>, value: Vec<u8>) -> KVResult<()> {
        let command = Command::Set {
            key: key.clone(),
            value,
        };

        let log_pointer = append_command(command, &mut self.writer)?;

        self.current_height += 1;

        self.index.insert(key, log_pointer);

        return Ok(());
    }

    pub fn get(&mut self, key: Vec<u8>) -> KVResult<Option<Vec<u8>>> {
        match self.index.get(&key) {
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
                        let command_buffer_parser = CommandBufferParser::new(command_buffer, 0);

                        let commands: Vec<Command> =
                            command_buffer_parser.map(|(command, _)| command).collect();
                        let res = recursive_find(&key, &commands, height)?;

                        return Ok(res);
                    }
                    Command::RevertAll { height: _ } => unreachable!(),
                    Command::Remove { key: _ } => unreachable!(),
                    Command::RemoveAll => unreachable!(),
                }
            }
        }
    }

    pub fn revert_one(&mut self, key: Vec<u8>, height: u64) -> KVResult<()> {
        if height > self.current_height {
            return Err(KVError::RervertOutOfRange);
        }

        let command = Command::RevertOne {
            key: key.clone(),
            height,
        };

        let log_pointer = append_command(command, &mut self.writer)?;
        self.current_height += 1;
        self.index.insert(key, log_pointer);

        return Ok(());
    }

    pub fn revert_all(&mut self, height: u64) -> KVResult<()> {
        if height > self.current_height {
            return Err(KVError::RervertOutOfRange);
        }

        let command = Command::RevertAll { height };

        append_command(command, &mut self.writer)?;

        self.current_height += 1;
        self.index = HashMap::new();
        load_index(&mut self.reader, &mut self.index, Some(height))?;

        return Ok(());
    }

    pub fn remove(&mut self, key: Vec<u8>) -> KVResult<()> {
        let command = Command::Remove { key: key.clone() };

        append_command(command, &mut self.writer)?;

        self.current_height += 1;
        self.index.remove(&key);

        return Ok(());
    }

    pub fn remove_all(&mut self) -> KVResult<()> {
        let command = Command::RemoveAll;

        append_command(command, &mut self.writer)?;

        self.current_height += 1;
        self.index.clear();

        return Ok(());
    }

    pub fn inspect(&mut self, key: Option<Vec<u8>>) -> KVResult<Vec<Command>> {
        let mut command_buffer: Vec<u8> = Vec::new();
        self.reader.seek(SeekFrom::Start(0))?;
        self.reader.read_to_end(&mut command_buffer)?;

        let command_buffer_parser = CommandBufferParser::new(command_buffer, 0);
        match key {
            None => {
                let ret = command_buffer_parser.map(|(command, _)| command).collect();
                return Ok(ret);
            }
            Some(target_key) => {
                let mut appeared_key = HashSet::new();
                let ret = command_buffer_parser
                    .filter_map(|(command, _)| match &command {
                        Command::Set { key, value: _ } => {
                            appeared_key.insert(key.clone());

                            if &target_key == key {
                                return Some(command);
                            } else {
                                return None;
                            }
                        }
                        Command::RevertOne { key, height: _ } => {
                            appeared_key.insert(key.clone());

                            if &target_key == key {
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

                            if &target_key == key {
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
    target_key: &Vec<u8>,
    commands: &Vec<Command>,
    target_height: u64,
) -> KVResult<Option<Vec<u8>>> {
    let target_command = &commands.as_slice()[target_height as usize];

    match target_command {
        Command::Set { key, value } => {
            if target_key == key {
                return Ok(Some(value.to_owned()));
            } else {
                return recursive_find(&target_key, &commands, target_height - 1);
            }
        }
        Command::RevertOne { key, height } => {
            if target_key == key {
                return recursive_find(&target_key, &commands, height.to_owned());
            } else {
                return recursive_find(&target_key, &commands, target_height - 1);
            }
        }
        Command::RevertAll { height } => {
            return recursive_find(&target_key, &commands, height.to_owned());
        }
        Command::Remove { key } => {
            if target_key == key {
                return Ok(None);
            } else {
                return recursive_find(&target_key, &commands, target_height - 1);
            }
        }
        Command::RemoveAll => {
            return Ok(None);
        }
    }
}

fn load_index(
    mut reader: &mut BufReader<File>,
    mut index: &mut HashMap<Vec<u8>, LogPointer>,
    target_height: Option<u64>,
) -> KVResult<u64> {
    let mut command_buffer: Vec<u8> = Vec::new();
    let mut current_position = 0;
    let mut height = 0;

    reader.seek(SeekFrom::Start(0))?;
    reader.read_to_end(&mut command_buffer)?;
    let command_buffer_parser = CommandBufferParser::new(command_buffer, 0);

    for (command, command_length) in command_buffer_parser {
        match command {
            Command::Set { key, value: _ } => {
                let log_pointer = LogPointer::new(current_position, command_length);
                index.insert(key, log_pointer);
            }
            Command::RevertOne { key, height: _ } => {
                let log_pointer = LogPointer::new(current_position, command_length);
                index.insert(key, log_pointer);
            }
            Command::RevertAll { height } => {
                *index = HashMap::new();
                load_index(&mut reader, &mut index, Some(height))?;
            }
            Command::Remove { key } => {
                index.remove(&key);
            }
            Command::RemoveAll => {
                index.clear();
            }
        }

        current_position += command_length as u64;

        if let Some(target_height) = target_height {
            if target_height == height {
                break;
            }
        }

        height += 1;
    }

    return Ok(height);
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
    use std::fs::remove_file;
    use std::path::PathBuf;

    use crate::storage::kv::{get_log_file_dir, KeyValueEngine};
    use std::collections::HashMap;

    fn get_store_engine(path: &PathBuf) -> KeyValueEngine {
        let log_file_path = get_log_file_dir(&path);

        if log_file_path.exists() {
            remove_file(log_file_path).unwrap();
        }

        let store_engine = KeyValueEngine::open(&path).unwrap();

        return store_engine;
    }

    #[test]
    fn kv_set() {
        let path = PathBuf::from("/tmp/test_set");
        let mut store_engine = get_store_engine(&path);

        let key: Vec<u8> = [0x00, 0x01, 0x03].to_vec();
        let expected_value: Vec<u8> = [0xff, 0xff, 0xff, 0xff].to_vec();

        store_engine
            .set(key.clone(), expected_value.clone())
            .unwrap();
        let actual_value = store_engine.get(key).unwrap().unwrap();

        assert_eq!(actual_value, expected_value);
    }

    #[test]
    fn kv_revert_one() {
        let path = PathBuf::from("/tmp/test_revert_one");
        let mut store_engine = get_store_engine(&path);
        let target_height = 2;

        let key: Vec<u8> = [0x00, 0x01, 0x03].to_vec();
        let values: Vec<Vec<u8>> = vec![
            [0x00].to_vec(),
            [0x01].to_vec(),
            [0x02].to_vec(),
            [0x03].to_vec(),
            [0x04].to_vec(),
            [0x05].to_vec(),
        ];

        for value in &values {
            store_engine.set(key.clone(), value.clone()).unwrap();
        }

        store_engine.revert_one(key.clone(), target_height).unwrap();

        let actual_output = store_engine.get(key.clone()).unwrap().unwrap();
        let expected_output = &values.as_slice()[target_height as usize];

        assert_eq!(&actual_output, expected_output);
    }

    #[test]
    fn kv_revert_all() {
        let path = PathBuf::from("/tmp/test_revert_all");
        let mut store_engine = get_store_engine(&path);
        let target_height = 5;

        let key_value_pairs = vec![
            ([0x00].to_vec(), [0x00].to_vec()),
            ([0x00].to_vec(), [0xff].to_vec()),
            ([0x00].to_vec(), [0x22].to_vec()),
            ([0x01].to_vec(), [0x01].to_vec()),
            ([0x00].to_vec(), [0x19].to_vec()),
            ([0x02].to_vec(), [0x02].to_vec()),
            ([0x03].to_vec(), [0x03].to_vec()),
            ([0x04].to_vec(), [0x04].to_vec()),
            ([0x05].to_vec(), [0x05].to_vec())
        ];

        for kv_pair in &key_value_pairs {
            store_engine.set(kv_pair.0.clone(), kv_pair.1.clone()).unwrap();
        }

        store_engine.revert_all(target_height).unwrap();

        let mut expected_hashmap = HashMap::new();

        for expected_kv_pair in &key_value_pairs[..target_height as usize] {
            expected_hashmap.insert(&expected_kv_pair.0, &expected_kv_pair.1);
        }

        for (key, expected_value) in expected_hashmap {
            let actual_value = store_engine.get(key.clone()).unwrap().unwrap();
            assert_eq!(expected_value, &actual_value);
        }

    }

    #[test]
    fn kv_remove() {
        let path = PathBuf::from("/tmp/remove");
        let mut store_engine = get_store_engine(&path);

        let key: Vec<u8> = [0x00, 0x01, 0x03].to_vec();
        let expected_value: Vec<u8> = [0xff, 0xff, 0xff, 0xff].to_vec();

        store_engine
            .set(key.clone(), expected_value.clone())
            .unwrap();
        store_engine.remove(key.clone()).unwrap();

        let actual_value = store_engine.get(key).unwrap();

        assert_eq!(actual_value, None);
    }

    #[test]
    fn kv_remove_all() {
        let path = PathBuf::from("/tmp/remove_all");
        let mut store_engine = get_store_engine(&path);

        let key_value_pairs = vec![
            ([0xff].to_vec(), [0x00].to_vec()),
            ([0xf2].to_vec(), [0xff].to_vec()),
            ([0x23].to_vec(), [0x22].to_vec()),
            ([0x11].to_vec(), [0x01].to_vec()),
        ];

        for kv_pair in &key_value_pairs {
            store_engine.set(kv_pair.0.clone(), kv_pair.1.clone()).unwrap();
        }

        store_engine.remove_all().unwrap();

        for kv_pair in &key_value_pairs {
            let out_put = store_engine.get(kv_pair.0.clone()).unwrap();
            assert_eq!(out_put, None);
        }
    }
}
