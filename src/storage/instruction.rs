use std::fmt;

use crate::constants::{INSTRUCTION_PACK_MAGIC, INSTRUCTION_PACK_VERSION};
use crate::storage::chain_height::ChainHeight;
use crate::storage::ecc::{
    ECCMode, ErrorCorrectionCodec, ErrorCorrectionError, IdentityCode, TripleRedundancyCode,
};
use crate::storage::kvkey::KVKey;
use crate::storage::kvvalue::KVValue;
use crate::storage::transaction_manager::TransactionId;
use crate::utils::ints::{byte_slice_to_u32, u32_to_u8_array, u64_to_u8_array, u8_array_to_u64};
use crate::utils::varint::{varint_decode, VarIntError};

#[derive(Debug, PartialEq)]
pub enum InstructionError {
    MissingPrefixByte,
    KeyExceedsMaxLength,
    VarIntError(VarIntError),
    UnknownPrefix(u8),
    PackTooShort(usize),
    UnexpectedMagicNumber([u8; 4]),
    UnexpectedPackVersion(u8),
    ErrorCorrection(ErrorCorrectionError),
    UnexpectedECCMode(u8),
    ParseInstructionErrorError,
}

pub enum InstructionErrorPrefix {
    MissingPrefixByte = 0x01,
    KeyExceedsMaxLength = 0x02,
    VarIntError = 0x03,
    UnknownPrefix = 0x04,
    PackTooShort = 0x05,
    UnexpectedMagicNumber = 0x06,
    UnexpectedPackVersion = 0x07,
    ErrorCorrection = 0x08,
    UnexpectedECCMode = 0x09,
    ParseInstructionErrorError = 0x10,
}

impl InstructionError {
    pub fn marshal(&self) -> Vec<u8> {
        match self {
            InstructionError::MissingPrefixByte => {
                vec![InstructionErrorPrefix::MissingPrefixByte as u8]
            }
            InstructionError::KeyExceedsMaxLength => {
                vec![InstructionErrorPrefix::KeyExceedsMaxLength as u8]
            }
            InstructionError::VarIntError(error) => {
                let mut result = vec![InstructionErrorPrefix::VarIntError as u8];
                let error_bytes = error.marshal();
                result.extend_from_slice(&error_bytes);
                return result;
            }
            InstructionError::UnknownPrefix(byte) => {
                let mut result = vec![InstructionErrorPrefix::UnknownPrefix as u8];
                result.push(byte.clone());
                return result;
            }
            InstructionError::PackTooShort(size) => {
                let mut result = vec![InstructionErrorPrefix::PackTooShort as u8];
                result.extend_from_slice(&u64_to_u8_array(*size as u64));
                return result;
            }
            InstructionError::UnexpectedMagicNumber(numbers) => {
                let mut result = vec![InstructionErrorPrefix::UnexpectedMagicNumber as u8];
                result.extend_from_slice(&numbers.to_vec());
                return result;
            }
            InstructionError::UnexpectedPackVersion(byte) => {
                let mut result = vec![InstructionErrorPrefix::UnexpectedPackVersion as u8];
                result.push(byte.clone());
                return result;
            }
            InstructionError::ErrorCorrection(error) => {
                let mut result = vec![InstructionErrorPrefix::ErrorCorrection as u8];
                result.extend_from_slice(&error.marshal());
                return result;
            }
            InstructionError::UnexpectedECCMode(byte) => {
                let mut result = vec![InstructionErrorPrefix::UnexpectedECCMode as u8];
                result.push(byte.clone());
                return result;
            }
            InstructionError::ParseInstructionErrorError => {
                vec![InstructionErrorPrefix::ParseInstructionErrorError as u8]
            }
        }
    }

    pub fn parse(data: &[u8]) -> Result<(InstructionError, usize), InstructionError> {
        let mut position = 0;
        let prefix = data[position];
        position += 1;

        if prefix == InstructionErrorPrefix::MissingPrefixByte as u8 {
            Ok((InstructionError::MissingPrefixByte, position))
        } else if prefix == InstructionErrorPrefix::KeyExceedsMaxLength as u8 {
            Ok((InstructionError::KeyExceedsMaxLength, position))
        } else if prefix == InstructionErrorPrefix::VarIntError as u8 {
            let (error, offset) = VarIntError::parse(&data[position..])?;
            position += offset;
            Ok((InstructionError::VarIntError(error), position))
        } else if prefix == InstructionErrorPrefix::UnknownPrefix as u8 {
            let byte = &data[position];
            position += 1;
            Ok((InstructionError::UnknownPrefix(byte.clone()), position))
        } else if prefix == InstructionErrorPrefix::PackTooShort as u8 {
            let size = u8_array_to_u64(&[
                data[position],
                data[position + 1],
                data[position + 2],
                data[position + 3],
                data[position + 4],
                data[position + 5],
                data[position + 6],
                data[position + 7],
            ]);
            position += 8;
            Ok((InstructionError::PackTooShort(size as usize), position))
        } else if prefix == InstructionErrorPrefix::UnexpectedMagicNumber as u8 {
            let numbers = &[
                data[position],
                data[position + 1],
                data[position + 2],
                data[position + 3],
            ];
            position += 4;
            Ok((
                InstructionError::UnexpectedMagicNumber(numbers.clone()),
                position,
            ))
        } else if prefix == InstructionErrorPrefix::UnexpectedPackVersion as u8 {
            let byte = &data[position];
            position += 1;
            Ok((
                InstructionError::UnexpectedPackVersion(byte.clone()),
                position,
            ))
        } else if prefix == InstructionErrorPrefix::ErrorCorrection as u8 {
            let (error, offset) = ErrorCorrectionError::parse(&data[position..])?;
            position += offset;
            Ok((InstructionError::ErrorCorrection(error), position))
        } else if prefix == InstructionErrorPrefix::UnexpectedECCMode as u8 {
            let byte = &data[position];
            position += 1;
            Ok((InstructionError::UnexpectedECCMode(byte.clone()), position))
        } else {
            Ok((InstructionError::ParseInstructionErrorError, position))
        }
    }
}

impl fmt::Display for InstructionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InstructionError::MissingPrefixByte => {
                write!(f, "{}", "InstructionError::MissingPrefixByte")
            }
            InstructionError::KeyExceedsMaxLength => {
                write!(f, "{}", "InstructionError::KeyExceedsMaxLength")
            }
            InstructionError::VarIntError(error) => {
                write!(f, "{}::{}", "InstructionError::KeyExceedsMaxLength", error)
            }
            InstructionError::UnknownPrefix(byte) => {
                write!(f, "{}::{}", "InstructionError::KeyExceedsMaxLength", byte)
            }
            InstructionError::PackTooShort(size) => {
                write!(f, "{}::{}", "InstructionError::KeyExceedsMaxLength", size)
            }
            InstructionError::UnexpectedMagicNumber(numbers) => {
                let numbers_string = numbers
                    .iter()
                    .map(|number| format!("{}", number))
                    .collect::<Vec<String>>()
                    .join(",");
                write!(
                    f,
                    "{}::{}",
                    "InstructionError::KeyExceedsMaxLength", numbers_string
                )
            }
            InstructionError::UnexpectedPackVersion(byte) => {
                write!(f, "{}::{}", "InstructionError::KeyExceedsMaxLength", byte)
            }
            InstructionError::ErrorCorrection(error) => {
                write!(f, "{}::{}", "InstructionError::KeyExceedsMaxLength", error)
            }
            InstructionError::UnexpectedECCMode(byte) => {
                write!(f, "{}::{}", "InstructionError::KeyExceedsMaxLength", byte)
            }
            InstructionError::ParseInstructionErrorError => {
                write!(f, "{}", "InstructionError::ParseInstructionErrorError",)
            }
        }
    }
}

impl From<VarIntError> for InstructionError {
    fn from(error: VarIntError) -> InstructionError {
        InstructionError::VarIntError(error)
    }
}

impl From<ErrorCorrectionError> for InstructionError {
    fn from(error: ErrorCorrectionError) -> InstructionError {
        InstructionError::ErrorCorrection(error)
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

#[cfg(test)]
mod instruction_tests {
    use crate::storage::chain_height::ChainHeight;
    use crate::storage::instruction::Instruction;
    use crate::storage::kvkey::KVKey;
    use crate::storage::kvvalue::KVValue;

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
}

/**
Structure of a instruction pack (serialized form for disk storage and transmitting on wire).

Item       | bytes | Description
-----------|-------|-------------------------------------------------------------------------------
MAGIC      |   4   | Always B10CDA7A (block data), marking the beginning of an instruction pack
VER        |   1   | Version of serialization format of the pack
ECC width  |   4   | Width the the ECC section, including ECC mode byte
ECC mode   |   1   | ECC mode was used to protect data, 1 for TMR, for example
ECC main   |  vary | Main data of instruction, protected by ECC

ECC main data (last part in the pack) can be decoded into raw instruction bytes by ECC decoders.
Raw instruction bytes can be then parsed into Instructions in memory for use in Rust.

Instruction pack are stored continuously on the instruction log file.
**/

const MAGIC_WIDTH: usize = 4;
const VERSION_BYTE_POS: usize = 4;
const ECC_WIDTH_POS: usize = 5;
const ECC_WIDTH_LENGTH: usize = 4;
const ECC_MODE_BYTE_POS: usize = 9;
const ECC_DATA_BYTE_POS: usize = 10;

pub fn pack_instruction(instruction: &Instruction, ecc_mode: ECCMode) -> Vec<u8> {
    let instruction_bytes = instruction.serialize();
    let mut pack_bytes = Vec::new();

    pack_bytes.extend_from_slice(&INSTRUCTION_PACK_MAGIC);
    pack_bytes.push(INSTRUCTION_PACK_VERSION);

    let ecc_data_bytes = match ecc_mode {
        ECCMode::Identity => IdentityCode::new().encode(&instruction_bytes),
        ECCMode::TMR => TripleRedundancyCode::new().encode(&instruction_bytes),
    };

    let data_length = 1 /*ECC Mode byte*/ + ecc_data_bytes.len();
    pack_bytes.extend_from_slice(&u32_to_u8_array(data_length as u32));
    pack_bytes.push(ecc_mode as u8);
    pack_bytes.extend(ecc_data_bytes);

    return pack_bytes;
}

pub fn unpack_instruction(pack_bytes: &[u8]) -> Result<(Instruction, usize), InstructionError> {
    if pack_bytes.len() < ECC_MODE_BYTE_POS {
        return Err(InstructionError::PackTooShort(pack_bytes.len()));
    } else if pack_bytes[0..MAGIC_WIDTH] != INSTRUCTION_PACK_MAGIC {
        return Err(InstructionError::UnexpectedMagicNumber([
            pack_bytes[0],
            pack_bytes[1],
            pack_bytes[2],
            pack_bytes[3],
        ]));
    } else if pack_bytes[VERSION_BYTE_POS] != INSTRUCTION_PACK_VERSION {
        return Err(InstructionError::UnexpectedPackVersion(pack_bytes[4]));
    } else {
        let ecc_width_slice = &pack_bytes[ECC_WIDTH_POS..ECC_WIDTH_POS + ECC_WIDTH_LENGTH];
        let ecc_data_width = byte_slice_to_u32(ecc_width_slice);
        let expected_full_width = ECC_MODE_BYTE_POS + ecc_data_width as usize;
        if expected_full_width > pack_bytes.len() {
            return Err(InstructionError::PackTooShort(pack_bytes.len()));
        } else {
            let ecc_mode = pack_bytes[ECC_MODE_BYTE_POS];
            let ecc_data_bytes = &pack_bytes[ECC_DATA_BYTE_POS..expected_full_width];
            let raw_instruction_bytes = if ecc_mode == ECCMode::Identity as u8 {
                IdentityCode::new().decode(ecc_data_bytes)?
            } else if ecc_mode == ECCMode::TMR as u8 {
                TripleRedundancyCode::new().decode(ecc_data_bytes)?
            } else {
                return Err(InstructionError::UnexpectedECCMode(ecc_mode));
            };
            let (instruction, _) = Instruction::parse(&raw_instruction_bytes)?;

            return Ok((instruction, expected_full_width));
        }
    }
}

#[cfg(test)]
mod instruction_packing_tests {
    use crate::storage::ecc::ECCMode;
    use crate::storage::instruction::{pack_instruction, unpack_instruction, Instruction};
    use crate::storage::kvkey::KVKey;
    use crate::storage::kvvalue::KVValue;
    use crate::storage::transaction_manager::TransactionId;

    #[test]
    fn test_pack_set_instruction_no_ecc() {
        let instruction = Instruction::Set {
            key: KVKey::from("hello"),
            value: KVValue::from("world"),
        };
        let correct_pack: Vec<u8> = vec![
            0xB1, 0x0C, 0xDA, 0x7A, // Magic
            0x01, // Version
            0x0E, 0x00, 0x00, 0x00, // ECC Width
            0x00, // ECC Mode
            0x00, // prefix for Set
            0x05, // key width
            0x68, 0x65, 0x6c, 0x6c, 0x6f, // key: "hello"
            0x05, // value width
            0x77, 0x6f, 0x72, 0x6c, 0x64, // value: "world"
        ];

        let packed = pack_instruction(&instruction, ECCMode::Identity);
        assert_eq!(correct_pack, packed);

        let (unpacked, width) = unpack_instruction(&correct_pack).unwrap();
        assert_eq!(unpacked, instruction);
        assert_eq!(correct_pack.len(), width);
    }

    #[test]
    fn test_pack_transaction_start_tmr() {
        let instruction = Instruction::TransactionStart {
            transaction_id: TransactionId::new(0x42),
        };
        let correct_pack: Vec<u8> = vec![
            0xB1, 0x0C, 0xDA, 0x7A, // Magic
            0x01, // Version
            0x07, 0x00, 0x00, 0x00, // ECC Width
            0x01, // ECC Mode
            // Duplication 1
            0x05, // prefix for TransactionStart
            0x42, // transaction_id
            // Duplication 2
            0x05, // prefix for TransactionStart
            0x42, // transaction_id
            // Duplication 3
            0x05, // prefix for TransactionStart
            0x42, // transaction_id
        ];

        let packed = pack_instruction(&instruction, ECCMode::TMR);
        assert_eq!(correct_pack, packed);

        let (unpacked, width) = unpack_instruction(&correct_pack).unwrap();
        assert_eq!(unpacked, instruction);
        assert_eq!(correct_pack.len(), width);
    }
}
