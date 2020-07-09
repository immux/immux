use crate::storage::chain_height::ChainHeight;
use crate::storage::kvkey::KVKey;
use crate::storage::kvvalue::KVValue;
use crate::storage::transaction_manager::TransactionId;
use crate::utils::varint::{varint_decode, VarIntError};

#[derive(Debug)]
pub enum CommandError {
    MissingPrefixByte,
    KeyExceedsMaxLength,
    VarInt(VarIntError),
    UnknownPrefix(u8),
}

impl From<VarIntError> for CommandError {
    fn from(error: VarIntError) -> CommandError {
        CommandError::VarInt(error)
    }
}

enum CommandPrefix {
    Set = 0x00,
    RevertOne = 0x01,
    RevertAll = 0x02,
    RemoveOne = 0x03,
    RemoveAll = 0x04,
    TransactionStart = 0x05,
    TransactionalSet = 0x06,
    TransactionalRevertOne = 0x07,
    TransactionalRemoveOne = 0x08,
    TransactionCommit = 0x09,
    TransactionAbort = 0x10,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    Set {
        key: KVKey,
        value: KVValue,
    },
    RevertOne {
        key: KVKey,
        height: ChainHeight,
    },
    RevertAll {
        height: ChainHeight,
    },
    RemoveOne {
        key: KVKey,
    },
    RemoveAll,
    TransactionStart {
        transaction_id: TransactionId,
    },
    TransactionalSet {
        key: KVKey,
        value: KVValue,
        transaction_id: TransactionId,
    },
    TransactionalRevertOne {
        key: KVKey,
        height: ChainHeight,
        transaction_id: TransactionId,
    },
    TransactionalRemoveOne {
        key: KVKey,
        transaction_id: TransactionId,
    },
    TransactionCommit {
        transaction_id: TransactionId,
    },
    TransactionAbort {
        transaction_id: TransactionId,
    },
}

impl Command {
    pub fn serialize(&self) -> Vec<u8> {
        let mut command_bytes: Vec<u8> = Vec::new();

        match self {
            Command::Set { key, value } => {
                command_bytes.push(CommandPrefix::Set as u8);
                command_bytes.extend(key.serialize());
                command_bytes.extend(value.serialize());
            }
            Command::RevertOne { key, height } => {
                command_bytes.push(CommandPrefix::RevertOne as u8);
                command_bytes.extend(key.serialize());
                command_bytes.extend(height.serialize());
            }
            Command::RevertAll { height } => {
                command_bytes.push(CommandPrefix::RevertAll as u8);
                command_bytes.extend(height.serialize());
            }
            Command::RemoveOne { key } => {
                command_bytes.push(CommandPrefix::RemoveOne as u8);
                command_bytes.extend(key.serialize());
            }
            Command::RemoveAll => {
                command_bytes.push(CommandPrefix::RemoveAll as u8);
            }
            Command::TransactionStart { transaction_id } => {
                command_bytes.push(CommandPrefix::TransactionStart as u8);
                command_bytes.extend(transaction_id.serialize());
            }
            Command::TransactionalSet { key, value, transaction_id } => {
                command_bytes.push(CommandPrefix::TransactionalSet as u8);

                command_bytes.extend(key.serialize());
                command_bytes.extend(value.serialize());
                command_bytes.extend(transaction_id.serialize());
            }
            Command::TransactionalRevertOne { key, height, transaction_id } => {
                command_bytes.push(CommandPrefix::TransactionalRevertOne as u8);

                command_bytes.extend(key.serialize());
                command_bytes.extend(height.serialize());
                command_bytes.extend(transaction_id.serialize());
            }
            Command::TransactionalRemoveOne { key, transaction_id } => {
                command_bytes.push(CommandPrefix::TransactionalRemoveOne as u8);
                command_bytes.extend(key.serialize());
                command_bytes.extend(transaction_id.serialize());
            }
            Command::TransactionCommit { transaction_id } => {
                command_bytes.push(CommandPrefix::TransactionCommit as u8);
                command_bytes.extend(transaction_id.serialize());
            }
            Command::TransactionAbort { transaction_id } => {
                command_bytes.push(CommandPrefix::TransactionAbort as u8);
                command_bytes.extend(transaction_id.serialize());
            }
        }
        return command_bytes;
    }

    pub fn parse(data: &[u8]) -> Result<(Self, usize), CommandError> {
        let mut position = 0;
        match data.get(position) {
            None => return Err(CommandError::MissingPrefixByte),
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
                } else if prefix == CommandPrefix::RemoveOne as u8 {
                    let (key_length, varint_size) = varint_decode(&data[position..])?;
                    position += varint_size;

                    let key = KVKey::new(&data[position..position + key_length as usize]);
                    position += key_length as usize;

                    let command = Command::RemoveOne { key };

                    return Ok((command, position));
                } else if prefix == CommandPrefix::RemoveAll as u8 {
                    let command = Command::RemoveAll;

                    return Ok((command, position));
                } else if prefix == CommandPrefix::TransactionStart as u8 {
                    let (transaction_id, varint_size) = varint_decode(&data[position..])?;
                    position += varint_size;

                    let command = Command::TransactionStart {
                        transaction_id: TransactionId::new(transaction_id),
                    };

                    return Ok((command, position));
                } else if prefix == CommandPrefix::TransactionalSet as u8 {
                    let (key_length, varint_size) = varint_decode(&data[position..])?;
                    position += varint_size;

                    let key = KVKey::new(&data[position..position + key_length as usize]);
                    position += key_length as usize;

                    let (value_length, varint_size) = varint_decode(&data[position..])?;
                    position += varint_size;

                    let value = KVValue::new(&data[position..position + value_length as usize]);
                    position += value_length as usize;

                    let (transaction_id, varint_size) = varint_decode(&data[position..])?;
                    position += varint_size;

                    let command = Command::TransactionalSet {
                        key,
                        value,
                        transaction_id: TransactionId::new(transaction_id),
                    };

                    return Ok((command, position));
                } else if prefix == CommandPrefix::TransactionalRemoveOne as u8 {
                    let (key_length, varint_size) = varint_decode(&data[position..])?;
                    position += varint_size;

                    let key = KVKey::new(&data[position..position + key_length as usize]);
                    position += key_length as usize;

                    let (transaction_id, varint_size) = varint_decode(&data[position..])?;
                    position += varint_size;

                    let command = Command::TransactionalRemoveOne {
                        key,
                        transaction_id: TransactionId::new(transaction_id),
                    };

                    return Ok((command, position));
                } else if prefix == CommandPrefix::TransactionalRevertOne as u8 {
                    let (key_length, varint_size) = varint_decode(&data[position..])?;
                    position += varint_size;

                    let key = KVKey::new(&data[position..position + key_length as usize]);
                    position += key_length as usize;

                    let (height, varint_size) = varint_decode(&data[position..])?;
                    position += varint_size;

                    let (transaction_id, varint_size) = varint_decode(&data[position..])?;
                    position += varint_size;

                    let command = Command::TransactionalRevertOne {
                        key,
                        height: ChainHeight::new(height),
                        transaction_id: TransactionId::new(transaction_id),
                    };

                    return Ok((command, position));
                } else if prefix == CommandPrefix::TransactionCommit as u8 {
                    let (transaction_id, varint_size) = varint_decode(&data[position..])?;
                    position += varint_size;

                    let command = Command::TransactionCommit {
                        transaction_id: TransactionId::new(transaction_id),
                    };

                    return Ok((command, position));
                } else if prefix == CommandPrefix::TransactionAbort as u8 {
                    let (transaction_id, varint_size) = varint_decode(&data[position..])?;
                    position += varint_size;

                    let command = Command::TransactionAbort {
                        transaction_id: TransactionId::new(transaction_id),
                    };

                    return Ok((command, position));
                } else {
                    return Err(CommandError::UnknownPrefix(prefix));
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

    let command_bytes: Vec<u8> = expected_command.serialize();
    let (actual_command, _) = Command::parse(&command_bytes).unwrap();

    assert_eq!(expected_command, actual_command);
}

#[test]
fn serialize_revert_one_command() {
    let key = KVKey::new(&[0x00, 0x01]);
    let height = ChainHeight::new(32);
    let expected_command = Command::RevertOne { key, height };

    let command_bytes: Vec<u8> = expected_command.serialize();
    let (actual_command, _) = Command::parse(&command_bytes).unwrap();

    assert_eq!(expected_command, actual_command);
}

#[test]
fn serialize_revert_all_command() {
    let height = ChainHeight::new(32);
    let expected_command = Command::RevertAll { height };

    let command_bytes: Vec<u8> = expected_command.serialize();
    let (actual_command, _) = Command::parse(&command_bytes).unwrap();

    assert_eq!(expected_command, actual_command);
}

#[test]
fn serialize_remove_command() {
    let key = KVKey::new(&[0x00, 0x01]);
    let expected_command = Command::RemoveOne { key };

    let command_bytes: Vec<u8> = expected_command.serialize();
    let (actual_command, _) = Command::parse(&command_bytes).unwrap();

    assert_eq!(expected_command, actual_command);
}

#[test]
fn serialize_remove_all_command() {
    let expected_command = Command::RemoveAll;

    let command_bytes: Vec<u8> = expected_command.serialize();
    let (actual_command, _) = Command::parse(&command_bytes).unwrap();

    assert_eq!(expected_command, actual_command);
}
