use std::convert::TryInto;

use crate::storage::chain_height::ChainHeight;
use crate::storage::kvkey::KVKey;
use crate::storage::kvvalue::KVValue;
use crate::varint::{varint_decode, varint_encode, VarIntError};

#[derive(Debug)]
pub enum CommandError {
    UnexpectedFormat(Option<VarIntError>),
    KeyExceedsMaxLength,
}

impl From<VarIntError> for CommandError {
    fn from(error: VarIntError) -> CommandError {
        CommandError::UnexpectedFormat(Some(error))
    }
}

enum CommandPrefix {
    Set = 0x00,
    RevertOne = 0x01,
    RevertAll = 0x02,
    Remove = 0x03,
    RemoveAll = 0x04,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    Set { key: KVKey, value: KVValue },
    RevertOne { key: KVKey, height: ChainHeight },
    RevertAll { height: ChainHeight },
    Remove { key: KVKey },
    RemoveAll,
}

impl TryInto<Vec<u8>> for Command {
    type Error = CommandError;

    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        match self {
            Command::Set { key, value } => {
                let mut command_bytes: Vec<u8> = Vec::new();

                command_bytes.push(CommandPrefix::Set as u8);

                let key_length: u64 = key.as_bytes().len() as u64;
                command_bytes.extend_from_slice(&varint_encode(key_length));
                command_bytes.extend_from_slice(&key.as_bytes());

                let value_length: u64 = value.as_bytes().len() as u64;
                command_bytes.extend_from_slice(&varint_encode(value_length));
                command_bytes.extend_from_slice(&value.as_bytes());

                return Ok(command_bytes);
            }
            Command::RevertOne { key, height } => {
                let mut command_bytes: Vec<u8> = Vec::new();

                command_bytes.push(CommandPrefix::RevertOne as u8);

                let key_length: u64 = key.as_bytes().len() as u64;
                command_bytes.extend_from_slice(&varint_encode(key_length));
                command_bytes.extend_from_slice(&key.as_bytes());

                command_bytes.extend_from_slice(&varint_encode(height.as_u64()));

                return Ok(command_bytes);
            }
            Command::RevertAll { height } => {
                let mut command_bytes: Vec<u8> = Vec::new();

                command_bytes.push(CommandPrefix::RevertAll as u8);
                command_bytes.extend_from_slice(&varint_encode(height.as_u64()));

                return Ok(command_bytes);
            }
            Command::Remove { key } => {
                let mut command_bytes: Vec<u8> = Vec::new();

                command_bytes.push(CommandPrefix::Remove as u8);

                let key_length: u64 = key.as_bytes().len() as u64;
                command_bytes.extend_from_slice(&varint_encode(key_length));
                command_bytes.extend_from_slice(&key.as_bytes());

                return Ok(command_bytes);
            }
            Command::RemoveAll => {
                let mut command_bytes: Vec<u8> = Vec::new();

                command_bytes.push(CommandPrefix::RemoveAll as u8);
                return Ok(command_bytes);
            }
        }
    }
}

impl Command {
    pub fn try_from(data: &[u8]) -> Result<(Self, usize), CommandError> {
        let mut position = 0;
        match data.get(position) {
            None => return Err(CommandError::UnexpectedFormat(None)),
            Some(&prefix) => {
                position += 1;

                if prefix == CommandPrefix::Set as u8 {
                    let (key_length, varint_size) = varint_decode(&data[position..])?;
                    position += varint_size;

                    let key = KVKey::new(&data[position..position + key_length as usize]);
                    position += key_length as usize;

                    let (value_length, varint_size) = varint_decode(&data[position..])?;
                    position += varint_size;

                    let value = KVValue::new(&data[position..position + value_length as usize]);
                    position += value_length as usize;

                    let command = Command::Set { key, value };

                    return Ok((command, position));
                } else if prefix == CommandPrefix::RevertOne as u8 {
                    let (key_length, varint_size) = varint_decode(&data[position..])?;
                    position += varint_size;

                    let key = KVKey::new(&data[position..position + key_length as usize]);
                    position += key_length as usize;

                    let (height, varint_size) = varint_decode(&data[position..])?;
                    position += varint_size;

                    let command = Command::RevertOne {
                        key,
                        height: ChainHeight::new(height),
                    };

                    return Ok((command, position));
                } else if prefix == CommandPrefix::RevertAll as u8 {
                    let (height, varint_size) = varint_decode(&data[position..])?;
                    position += varint_size;

                    let command = Command::RevertAll {
                        height: ChainHeight::new(height),
                    };

                    return Ok((command, position));
                } else if prefix == CommandPrefix::Remove as u8 {
                    let (key_length, varint_size) = varint_decode(&data[position..])?;
                    position += varint_size;

                    let key = KVKey::new(&data[position..position + key_length as usize]);
                    position += key_length as usize;

                    let command = Command::Remove { key };

                    return Ok((command, position));
                } else if prefix == CommandPrefix::RemoveAll as u8 {
                    let command = Command::RemoveAll;

                    return Ok((command, position));
                } else {
                    return Err(CommandError::UnexpectedFormat(None));
                }
            }
        }
    }
}

#[test]
fn parse_set_command() {
    let key = KVKey::new(&[0x00, 0x01]);
    let value = KVValue::new(&[0xff, 0xf2, 0xfe]);
    let expected_command = Command::Set { key, value };

    let command_bytes: Vec<u8> = expected_command.clone().try_into().unwrap();
    let (actual_command, _) = Command::try_from(&command_bytes).unwrap();

    assert_eq!(expected_command, actual_command);
}

#[test]
fn serde_revert_one_command() {
    let key = KVKey::new(&[0x00, 0x01]);
    let height = ChainHeight::new(32);
    let expected_command = Command::RevertOne { key, height };

    let command_bytes: Vec<u8> = expected_command.clone().try_into().unwrap();
    let (actual_command, _) = Command::try_from(&command_bytes).unwrap();

    assert_eq!(expected_command, actual_command);
}

#[test]
fn serde_revert_all_command() {
    let height = ChainHeight::new(32);
    let expected_command = Command::RevertAll { height };

    let command_bytes: Vec<u8> = expected_command.clone().try_into().unwrap();
    let (actual_command, _) = Command::try_from(&command_bytes).unwrap();

    assert_eq!(expected_command, actual_command);
}

#[test]
fn serde_remove_command() {
    let key = KVKey::new(&[0x00, 0x01]);
    let expected_command = Command::Remove { key };

    let command_bytes: Vec<u8> = expected_command.clone().try_into().unwrap();
    let (actual_command, _) = Command::try_from(&command_bytes).unwrap();

    assert_eq!(expected_command, actual_command);
}

#[test]
fn serde_remove_all_command() {
    let expected_command = Command::RemoveAll;

    let command_bytes: Vec<u8> = expected_command.clone().try_into().unwrap();
    let (actual_command, _) = Command::try_from(&command_bytes).unwrap();

    assert_eq!(expected_command, actual_command);
}
