use crate::varint::{varint_decode, varint_encode, VarIntError};
use std::convert::TryInto;

pub const MAX_KEY_LENGTH: usize = 8 * 1024;

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
    Set { key: Vec<u8>, value: Vec<u8> },
    RevertOne { key: Vec<u8>, height: u64 },
    RevertAll { height: u64 },
    Remove { key: Vec<u8> },
    RemoveAll,
}

impl TryInto<Vec<u8>> for Command {
    type Error = CommandError;

    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        match self {
            Command::Set { key, value } => {
                if key.len() > MAX_KEY_LENGTH {
                    return Err(CommandError::KeyExceedsMaxLength);
                }

                let mut ret: Vec<u8> = Vec::new();

                ret.push(CommandPrefix::Set as u8);

                let key_length: u64 = key.len() as u64;
                ret.extend_from_slice(&varint_encode(key_length));
                ret.extend_from_slice(&key);

                let value_length: u64 = value.len() as u64;
                ret.extend_from_slice(&varint_encode(value_length));
                ret.extend_from_slice(&value);

                return Ok(ret);
            }
            Command::RevertOne { key, height } => {
                if key.len() > MAX_KEY_LENGTH {
                    return Err(CommandError::KeyExceedsMaxLength);
                }

                let mut ret: Vec<u8> = Vec::new();

                ret.push(CommandPrefix::RevertOne as u8);

                let key_length: u64 = key.len() as u64;
                ret.extend_from_slice(&varint_encode(key_length));
                ret.extend_from_slice(&key);

                ret.extend_from_slice(&varint_encode(height));

                return Ok(ret);
            }
            Command::RevertAll { height } => {
                let mut ret: Vec<u8> = Vec::new();

                ret.push(CommandPrefix::RevertAll as u8);
                ret.extend_from_slice(&varint_encode(height));

                return Ok(ret);
            }
            Command::Remove { key } => {
                if key.len() > MAX_KEY_LENGTH {
                    return Err(CommandError::KeyExceedsMaxLength);
                }

                let mut ret: Vec<u8> = Vec::new();

                ret.push(CommandPrefix::Remove as u8);

                let key_length: u64 = key.len() as u64;
                ret.extend_from_slice(&varint_encode(key_length));
                ret.extend_from_slice(&key);

                return Ok(ret);
            }
            Command::RemoveAll => {
                let mut ret: Vec<u8> = Vec::new();

                ret.push(CommandPrefix::RemoveAll as u8);
                return Ok(ret);
            }
        }
    }
}

impl Command {
    pub fn try_from(data: &[u8]) -> Result<(Self, usize), CommandError> {
        let mut index = 0;
        match data.get(index) {
            None => return Err(CommandError::UnexpectedFormat(None)),
            Some(&prefix) => {
                index += 1;

                if prefix == CommandPrefix::Set as u8 {
                    let (key_length, varint_size) = varint_decode(&data[index..])?;
                    index += varint_size;

                    let key = &data[index..index + key_length as usize];
                    index += key_length as usize;

                    let (value_length, varint_size) = varint_decode(&data[index..])?;
                    index += varint_size;

                    let value = &data[index..index + value_length as usize];
                    index += value_length as usize;

                    let command = Command::Set {
                        key: key.to_vec(),
                        value: value.to_vec(),
                    };

                    return Ok((command, index));
                } else if prefix == CommandPrefix::RevertOne as u8 {
                    let (key_length, varint_size) = varint_decode(&data[index..])?;
                    index += varint_size;

                    let key = &data[index..index + key_length as usize];
                    index += key_length as usize;

                    let (height, varint_size) = varint_decode(&data[index..])?;
                    index += varint_size;

                    let command = Command::RevertOne {
                        key: key.to_vec(),
                        height,
                    };

                    return Ok((command, index));
                } else if prefix == CommandPrefix::RevertAll as u8 {
                    let (height, varint_size) = varint_decode(&data[index..])?;
                    index += varint_size;

                    let command = Command::RevertAll { height };

                    return Ok((command, index));
                } else if prefix == CommandPrefix::Remove as u8 {
                    let (key_length, varint_size) = varint_decode(&data[index..])?;
                    index += varint_size;

                    let key = &data[index..index + key_length as usize];
                    index += key_length as usize;

                    let command = Command::Remove { key: key.to_vec() };

                    return Ok((command, index));
                } else if prefix == CommandPrefix::RemoveAll as u8 {
                    let command = Command::RemoveAll;

                    return Ok((command, index));
                } else {
                    return Err(CommandError::UnexpectedFormat(None));
                }
            }
        }
    }
}

#[test]
fn parse_set_command() {
    let key = [0x00, 0x01].to_vec();
    let value = [0xff, 0xf2, 0xfe].to_vec();
    let expected_command = Command::Set {
        key: key.clone(),
        value: value.clone(),
    };

    let command_bytes: Vec<u8> = expected_command.clone().try_into().unwrap();
    let (actual_command, _) = Command::try_from(&command_bytes).unwrap();

    assert_eq!(expected_command, actual_command);
}

#[test]
fn serde_revert_one_command() {
    let key = [0x00, 0x01].to_vec();
    let height = 32;
    let expected_command = Command::RevertOne {
        key: key.clone(),
        height,
    };

    let command_bytes: Vec<u8> = expected_command.clone().try_into().unwrap();
    let (actual_command, _) = Command::try_from(&command_bytes).unwrap();

    assert_eq!(expected_command, actual_command);
}

#[test]
fn serde_revert_all_command() {
    let height = 32;
    let expected_command = Command::RevertAll { height };

    let command_bytes: Vec<u8> = expected_command.clone().try_into().unwrap();
    let (actual_command, _) = Command::try_from(&command_bytes).unwrap();

    assert_eq!(expected_command, actual_command);
}

#[test]
fn serde_remove_command() {
    let key = [0x00, 0x01].to_vec();
    let expected_command = Command::Remove { key: key.clone() };

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
