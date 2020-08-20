use crate::storage::chain_height::ChainHeight;
use crate::storage::kvkey::KVKey;
use crate::storage::kvvalue::KVValue;
use crate::storage::transaction_manager::TransactionId;
use crate::utils::varint::{varint_decode, VarIntError};

#[derive(Debug)]
pub enum InstructionError {
    MissingPrefixByte,
    KeyExceedsMaxLength,
    VarIntError(VarIntError),
    UnknownPrefix(u8),
}

impl From<VarIntError> for InstructionError {
    fn from(error: VarIntError) -> InstructionError {
        InstructionError::VarIntError(error)
    }
}

enum InstructionPrefix {
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
pub enum Instruction {
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

impl Instruction {
    pub fn serialize(&self) -> Vec<u8> {
        let mut instruction_bytes: Vec<u8> = Vec::new();

        match self {
            Instruction::Set { key, value } => {
                instruction_bytes.push(InstructionPrefix::Set as u8);
                instruction_bytes.extend(key.serialize());
                instruction_bytes.extend(value.serialize());
            }
            Instruction::RevertOne { key, height } => {
                instruction_bytes.push(InstructionPrefix::RevertOne as u8);
                instruction_bytes.extend(key.serialize());
                instruction_bytes.extend(height.serialize());
            }
            Instruction::RevertAll { height } => {
                instruction_bytes.push(InstructionPrefix::RevertAll as u8);
                instruction_bytes.extend(height.serialize());
            }
            Instruction::RemoveOne { key } => {
                instruction_bytes.push(InstructionPrefix::RemoveOne as u8);
                instruction_bytes.extend(key.serialize());
            }
            Instruction::RemoveAll => {
                instruction_bytes.push(InstructionPrefix::RemoveAll as u8);
            }
            Instruction::TransactionStart { transaction_id } => {
                instruction_bytes.push(InstructionPrefix::TransactionStart as u8);
                instruction_bytes.extend(transaction_id.serialize());
            }
            Instruction::TransactionalSet {
                key,
                value,
                transaction_id,
            } => {
                instruction_bytes.push(InstructionPrefix::TransactionalSet as u8);

                instruction_bytes.extend(key.serialize());
                instruction_bytes.extend(value.serialize());
                instruction_bytes.extend(transaction_id.serialize());
            }
            Instruction::TransactionalRevertOne {
                key,
                height,
                transaction_id,
            } => {
                instruction_bytes.push(InstructionPrefix::TransactionalRevertOne as u8);

                instruction_bytes.extend(key.serialize());
                instruction_bytes.extend(height.serialize());
                instruction_bytes.extend(transaction_id.serialize());
            }
            Instruction::TransactionalRemoveOne {
                key,
                transaction_id,
            } => {
                instruction_bytes.push(InstructionPrefix::TransactionalRemoveOne as u8);
                instruction_bytes.extend(key.serialize());
                instruction_bytes.extend(transaction_id.serialize());
            }
            Instruction::TransactionCommit { transaction_id } => {
                instruction_bytes.push(InstructionPrefix::TransactionCommit as u8);
                instruction_bytes.extend(transaction_id.serialize());
            }
            Instruction::TransactionAbort { transaction_id } => {
                instruction_bytes.push(InstructionPrefix::TransactionAbort as u8);
                instruction_bytes.extend(transaction_id.serialize());
            }
        }
        return instruction_bytes;
    }

    pub fn parse(data: &[u8]) -> Result<(Self, usize), InstructionError> {
        let mut position = 0;
        match data.get(position) {
            None => return Err(InstructionError::MissingPrefixByte),
            Some(&prefix) => {
                position += 1;

                if prefix == InstructionPrefix::Set as u8 {
                    let (key_length, varint_size) = varint_decode(&data[position..])?;
                    position += varint_size;

                    let key = KVKey::new(&data[position..position + key_length as usize]);
                    position += key_length as usize;

                    let (value_length, varint_size) = varint_decode(&data[position..])?;
                    position += varint_size;

                    let value = KVValue::new(&data[position..position + value_length as usize]);
                    position += value_length as usize;

                    let instruction = Instruction::Set { key, value };

                    return Ok((instruction, position));
                } else if prefix == InstructionPrefix::RevertOne as u8 {
                    let (key_length, varint_size) = varint_decode(&data[position..])?;
                    position += varint_size;

                    let key = KVKey::new(&data[position..position + key_length as usize]);
                    position += key_length as usize;

                    let (height, varint_size) = varint_decode(&data[position..])?;
                    position += varint_size;

                    let instruction = Instruction::RevertOne {
                        key,
                        height: ChainHeight::new(height),
                    };

                    return Ok((instruction, position));
                } else if prefix == InstructionPrefix::RevertAll as u8 {
                    let (height, varint_size) = varint_decode(&data[position..])?;
                    position += varint_size;

                    let instruction = Instruction::RevertAll {
                        height: ChainHeight::new(height),
                    };

                    return Ok((instruction, position));
                } else if prefix == InstructionPrefix::RemoveOne as u8 {
                    let (key_length, varint_size) = varint_decode(&data[position..])?;
                    position += varint_size;

                    let key = KVKey::new(&data[position..position + key_length as usize]);
                    position += key_length as usize;

                    let instruction = Instruction::RemoveOne { key };

                    return Ok((instruction, position));
                } else if prefix == InstructionPrefix::RemoveAll as u8 {
                    let instruction = Instruction::RemoveAll;

                    return Ok((instruction, position));
                } else if prefix == InstructionPrefix::TransactionStart as u8 {
                    let (transaction_id, varint_size) = varint_decode(&data[position..])?;
                    position += varint_size;

                    let instruction = Instruction::TransactionStart {
                        transaction_id: TransactionId::new(transaction_id),
                    };

                    return Ok((instruction, position));
                } else if prefix == InstructionPrefix::TransactionalSet as u8 {
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

                    let instruction = Instruction::TransactionalSet {
                        key,
                        value,
                        transaction_id: TransactionId::new(transaction_id),
                    };

                    return Ok((instruction, position));
                } else if prefix == InstructionPrefix::TransactionalRemoveOne as u8 {
                    let (key_length, varint_size) = varint_decode(&data[position..])?;
                    position += varint_size;

                    let key = KVKey::new(&data[position..position + key_length as usize]);
                    position += key_length as usize;

                    let (transaction_id, varint_size) = varint_decode(&data[position..])?;
                    position += varint_size;

                    let instruction = Instruction::TransactionalRemoveOne {
                        key,
                        transaction_id: TransactionId::new(transaction_id),
                    };

                    return Ok((instruction, position));
                } else if prefix == InstructionPrefix::TransactionalRevertOne as u8 {
                    let (key_length, varint_size) = varint_decode(&data[position..])?;
                    position += varint_size;

                    let key = KVKey::new(&data[position..position + key_length as usize]);
                    position += key_length as usize;

                    let (height, varint_size) = varint_decode(&data[position..])?;
                    position += varint_size;

                    let (transaction_id, varint_size) = varint_decode(&data[position..])?;
                    position += varint_size;

                    let instruction = Instruction::TransactionalRevertOne {
                        key,
                        height: ChainHeight::new(height),
                        transaction_id: TransactionId::new(transaction_id),
                    };

                    return Ok((instruction, position));
                } else if prefix == InstructionPrefix::TransactionCommit as u8 {
                    let (transaction_id, varint_size) = varint_decode(&data[position..])?;
                    position += varint_size;

                    let instruction = Instruction::TransactionCommit {
                        transaction_id: TransactionId::new(transaction_id),
                    };

                    return Ok((instruction, position));
                } else if prefix == InstructionPrefix::TransactionAbort as u8 {
                    let (transaction_id, varint_size) = varint_decode(&data[position..])?;
                    position += varint_size;

                    let instruction = Instruction::TransactionAbort {
                        transaction_id: TransactionId::new(transaction_id),
                    };

                    return Ok((instruction, position));
                } else {
                    return Err(InstructionError::UnknownPrefix(prefix));
                }
            }
        }
    }
}

#[test]
fn parse_set_instruction() {
    let key = KVKey::new(&[0x00, 0x01]);
    let value = KVValue::new(&[0xff, 0xf2, 0xfe]);
    let expected_instruction = Instruction::Set { key, value };

    let instruction_bytes: Vec<u8> = expected_instruction.serialize();
    let (actual_instruction, _) = Instruction::parse(&instruction_bytes).unwrap();

    assert_eq!(expected_instruction, actual_instruction);
}

#[test]
fn serialize_revert_one_instruction() {
    let key = KVKey::new(&[0x00, 0x01]);
    let height = ChainHeight::new(32);
    let expected_instruction = Instruction::RevertOne { key, height };

    let instruction_bytes: Vec<u8> = expected_instruction.serialize();
    let (actual_instruction, _) = Instruction::parse(&instruction_bytes).unwrap();

    assert_eq!(expected_instruction, actual_instruction);
}

#[test]
fn serialize_revert_all_instruction() {
    let height = ChainHeight::new(32);
    let expected_instruction = Instruction::RevertAll { height };

    let instruction_bytes: Vec<u8> = expected_instruction.serialize();
    let (actual_instruction, _) = Instruction::parse(&instruction_bytes).unwrap();

    assert_eq!(expected_instruction, actual_instruction);
}

#[test]
fn serialize_remove_instruction() {
    let key = KVKey::new(&[0x00, 0x01]);
    let expected_instruction = Instruction::RemoveOne { key };

    let instruction_bytes: Vec<u8> = expected_instruction.serialize();
    let (actual_instruction, _) = Instruction::parse(&instruction_bytes).unwrap();

    assert_eq!(expected_instruction, actual_instruction);
}

#[test]
fn serialize_remove_all_instruction() {
    let expected_instruction = Instruction::RemoveAll;

    let instruction_bytes: Vec<u8> = expected_instruction.serialize();
    let (actual_instruction, _) = Instruction::parse(&instruction_bytes).unwrap();

    assert_eq!(expected_instruction, actual_instruction);
}
