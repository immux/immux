use std::fmt;

use crate::storage::chain_height::ChainHeight;
use crate::storage::executor::command::{Command, CommandError};
use crate::storage::executor::unit_content::{UnitContent, UnitContentError};
use crate::storage::transaction_manager::TransactionId;
use crate::utils::ints::{u64_to_u8_array, u8_array_to_u64};
use crate::utils::varint::{varint_decode, varint_encode, VarIntError};

#[derive(Debug)]
pub enum OutcomeError {
    UnexpectedPrefix,
    VarIntError(VarIntError),
    UnitContentError(UnitContentError),
    CommandError(CommandError),
}

impl From<VarIntError> for OutcomeError {
    fn from(error: VarIntError) -> OutcomeError {
        OutcomeError::VarIntError(error)
    }
}

impl From<UnitContentError> for OutcomeError {
    fn from(error: UnitContentError) -> OutcomeError {
        OutcomeError::UnitContentError(error)
    }
}

impl From<CommandError> for OutcomeError {
    fn from(error: CommandError) -> OutcomeError {
        OutcomeError::CommandError(error)
    }
}

#[derive(Debug)]
pub enum OutcomePrefix {
    SelectSuccess = 0x11,
    InspectOneSuccess = 0x12,
    InspectAllSuccess = 0x13,
    InsertSuccess = 0x14,
    RevertOneSuccess = 0x15,
    RevertAllSuccess = 0x16,
    RemoveOneSuccess = 0x17,
    RemoveAllSuccess = 0x18,

    TransactionalInsertSuccess = 0x64,
    TransactionalRevertOneSuccess = 0x65,
    TransactionalRemoveOneSuccess = 0x67,

    CreateTransactionSuccess = 0xd0,
    TransactionCommitSuccess = 0xd1,
    TransactionAbortSuccess = 0xd2,
}

#[derive(Debug, PartialEq)]
pub enum Outcome {
    Select(Vec<UnitContent>),
    InspectOne(Vec<(Command, ChainHeight)>),
    InspectAll(Vec<(Command, ChainHeight)>),
    InsertSuccess,
    RevertOneSuccess,
    RevertAllSuccess,
    RemoveOneSuccess,
    RemoveAllSuccess,
    CreateTransaction(TransactionId),
    TransactionalInsertSuccess,
    TransactionalRevertOneSuccess,
    TransactionalRemoveOneSuccess,
    TransactionCommitSuccess,
    TransactionAbortSuccess,
}

impl Outcome {
    pub fn marshal(&self) -> Vec<u8> {
        match self {
            Outcome::Select(contents) => {
                let mut result = vec![OutcomePrefix::SelectSuccess as u8];
                let total_items = contents.len();
                let total_items_bytes = varint_encode(total_items as u64);
                result.extend_from_slice(&total_items_bytes);
                for content in contents {
                    let content_bytes = content.marshal();
                    result.extend_from_slice(&content_bytes);
                }
                return result;
            }
            Outcome::InspectOne(commands_with_heights) => {
                let mut result = vec![OutcomePrefix::InspectOneSuccess as u8];
                let total_items = commands_with_heights.len();
                let total_items_bytes = varint_encode(total_items as u64);
                result.extend_from_slice(&total_items_bytes);
                for (command, height) in commands_with_heights {
                    let command_bytes = command.marshal();
                    result.extend_from_slice(&command_bytes);
                    let height_bytes = u64_to_u8_array(height.as_u64());
                    result.extend_from_slice(&height_bytes);
                }
                return result;
            }
            Outcome::InspectAll(commands_with_heights) => {
                let mut result = vec![OutcomePrefix::InspectAllSuccess as u8];
                let total_items = commands_with_heights.len();
                let total_items_bytes = varint_encode(total_items as u64);
                result.extend_from_slice(&total_items_bytes);
                for (command, height) in commands_with_heights {
                    let command_bytes = command.marshal();
                    result.extend_from_slice(&command_bytes);
                    let height_bytes = u64_to_u8_array(height.as_u64());
                    result.extend_from_slice(&height_bytes);
                }
                return result;
            }
            Outcome::InsertSuccess => vec![OutcomePrefix::InsertSuccess as u8],
            Outcome::RevertOneSuccess => vec![OutcomePrefix::RevertOneSuccess as u8],
            Outcome::RevertAllSuccess => vec![OutcomePrefix::RevertAllSuccess as u8],
            Outcome::RemoveOneSuccess => vec![OutcomePrefix::RemoveOneSuccess as u8],
            Outcome::RemoveAllSuccess => vec![OutcomePrefix::RemoveAllSuccess as u8],
            Outcome::CreateTransaction(transaction_id) => {
                let mut result = vec![OutcomePrefix::CreateTransactionSuccess as u8];
                let transaction_id_bytes = u64_to_u8_array(transaction_id.as_u64());
                result.extend_from_slice(&transaction_id_bytes);
                return result;
            }
            Outcome::TransactionalInsertSuccess => {
                vec![OutcomePrefix::TransactionalInsertSuccess as u8]
            }
            Outcome::TransactionalRevertOneSuccess => {
                vec![OutcomePrefix::TransactionalRevertOneSuccess as u8]
            }
            Outcome::TransactionalRemoveOneSuccess => {
                vec![OutcomePrefix::TransactionalRemoveOneSuccess as u8]
            }
            Outcome::TransactionCommitSuccess => {
                vec![OutcomePrefix::TransactionCommitSuccess as u8]
            }
            Outcome::TransactionAbortSuccess => vec![OutcomePrefix::TransactionAbortSuccess as u8],
        }
    }
    pub fn parse(data: &[u8]) -> Result<(Self, usize), OutcomeError> {
        let mut position = 0;
        let prefix = data[position];
        position += 1;

        if prefix == OutcomePrefix::SelectSuccess as u8 {
            let (total_number_items, offset) = varint_decode(&data[position..])?;
            position += offset;

            let mut result = vec![];
            for _ in 0..total_number_items {
                let (content, offset) = UnitContent::parse(&data[position..])?;
                position += offset;
                result.push(content);
            }

            return Ok((Outcome::Select(result), position));
        } else if prefix == OutcomePrefix::InspectOneSuccess as u8 {
            let (total_number_items, offset) = varint_decode(&data[position..])?;
            position += offset;

            let mut result = vec![];
            for _ in 0..total_number_items {
                let (command, offset) = Command::parse(&data[position..])?;
                position += offset;
                let height = u8_array_to_u64(&[
                    data[position],
                    data[position + 1],
                    data[position + 2],
                    data[position + 3],
                    data[position + 4],
                    data[position + 5],
                    data[position + 6],
                    data[position + 7],
                ]);
                let chain_height = ChainHeight::new(height);
                position += 8;

                result.push((command, chain_height));
            }

            return Ok((Outcome::InspectOne(result), position));
        } else if prefix == OutcomePrefix::InspectAllSuccess as u8 {
            let (total_number_items, offset) = varint_decode(&data[position..])?;
            position += offset;

            let mut result = vec![];
            for _ in 0..total_number_items {
                let (command, offset) = Command::parse(&data[position..])?;
                position += offset;
                let height = u8_array_to_u64(&[
                    data[position],
                    data[position + 1],
                    data[position + 2],
                    data[position + 3],
                    data[position + 4],
                    data[position + 5],
                    data[position + 6],
                    data[position + 7],
                ]);
                let chain_height = ChainHeight::new(height);
                position += 8;

                result.push((command, chain_height));
            }

            return Ok((Outcome::InspectAll(result), position));
        } else if prefix == OutcomePrefix::InsertSuccess as u8 {
            return Ok((Outcome::InsertSuccess, position));
        } else if prefix == OutcomePrefix::RevertOneSuccess as u8 {
            return Ok((Outcome::RevertOneSuccess, position));
        } else if prefix == OutcomePrefix::RevertAllSuccess as u8 {
            return Ok((Outcome::RevertAllSuccess, position));
        } else if prefix == OutcomePrefix::RemoveOneSuccess as u8 {
            return Ok((Outcome::RemoveOneSuccess, position));
        } else if prefix == OutcomePrefix::RemoveAllSuccess as u8 {
            return Ok((Outcome::RemoveAllSuccess, position));
        } else if prefix == OutcomePrefix::CreateTransactionSuccess as u8 {
            let transaction_id_num = u8_array_to_u64(&[
                data[position],
                data[position + 1],
                data[position + 2],
                data[position + 3],
                data[position + 4],
                data[position + 5],
                data[position + 6],
                data[position + 7],
            ]);
            let transaction_id = TransactionId::new(transaction_id_num);
            position += 8;
            return Ok((Outcome::CreateTransaction(transaction_id), position));
        } else if prefix == OutcomePrefix::TransactionalInsertSuccess as u8 {
            return Ok((Outcome::TransactionalInsertSuccess, position));
        } else if prefix == OutcomePrefix::TransactionalRevertOneSuccess as u8 {
            return Ok((Outcome::TransactionalRevertOneSuccess, position));
        } else if prefix == OutcomePrefix::TransactionalRemoveOneSuccess as u8 {
            return Ok((Outcome::TransactionalRemoveOneSuccess, position));
        } else if prefix == OutcomePrefix::TransactionCommitSuccess as u8 {
            return Ok((Outcome::TransactionCommitSuccess, position));
        } else if prefix == OutcomePrefix::TransactionAbortSuccess as u8 {
            return Ok((Outcome::TransactionAbortSuccess, position));
        } else {
            return Err(OutcomeError::UnexpectedPrefix);
        }
    }
}

impl fmt::Display for Outcome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string = match self {
            Outcome::Select(contents) => {
                let output_vec: Vec<String> = contents
                    .into_iter()
                    .map(|content| content.to_string())
                    .collect();
                output_vec.join("\r\n")
            }
            Outcome::InspectOne(commands_with_heights) => {
                let output_vec: Vec<String> = commands_with_heights
                    .into_iter()
                    .map(|(command, height)| format!("{} {}", command.to_string(), height.as_u64()))
                    .collect();
                output_vec.join("\r\n")
            }
            Outcome::InspectAll(commands_with_heights) => {
                let output_vec: Vec<String> = commands_with_heights
                    .into_iter()
                    .map(|(command, height)| format!("{} {}", command.to_string(), height.as_u64()))
                    .collect();
                output_vec.join("\r\n")
            }
            Outcome::InsertSuccess => String::from("Insert Success"),
            Outcome::RevertOneSuccess => String::from("Revert One Success"),
            Outcome::RevertAllSuccess => String::from("Revert All Success"),
            Outcome::RemoveOneSuccess => String::from("Remove One Success"),
            Outcome::RemoveAllSuccess => String::from("Remove All Success"),
            Outcome::CreateTransaction(transaction_id) => format!("{}", transaction_id.as_u64()),
            Outcome::TransactionalInsertSuccess => String::from("Transactional Insert Success"),
            Outcome::TransactionalRevertOneSuccess => {
                String::from("Transactional Revert One Success")
            }
            Outcome::TransactionalRemoveOneSuccess => {
                String::from("Transactional Remove One Success")
            }
            Outcome::TransactionCommitSuccess => String::from("Transaction Commit Success"),
            Outcome::TransactionAbortSuccess => String::from("Transaction Abort Success"),
        };
        write!(f, "{}", string)
    }
}

#[cfg(test)]
mod outcome_tests {
    use std::collections::HashMap;

    use crate::storage::chain_height::ChainHeight;
    use crate::storage::executor::command::Command;
    use crate::storage::executor::grouping_label::GroupingLabel;
    use crate::storage::executor::outcome::Outcome;
    use crate::storage::executor::unit_content::UnitContent;
    use crate::storage::executor::unit_key::UnitKey;
    use crate::storage::transaction_manager::TransactionId;

    fn get_hashmap_content() -> UnitContent {
        let mut map = HashMap::new();
        map.insert(
            String::from("brand"),
            UnitContent::String(String::from("apple")),
        );
        map.insert(String::from("price"), UnitContent::Float64(4000.0));
        let map_content = UnitContent::Map(map);

        return map_content;
    }

    fn get_contents() -> Vec<UnitContent> {
        let map_content = get_hashmap_content();

        vec![
            map_content,
            UnitContent::String(String::from("this is a string")),
            UnitContent::Bool(true),
            UnitContent::Bool(false),
            UnitContent::Nil,
            UnitContent::String(String::from("true")),
            UnitContent::String(String::from("false")),
            UnitContent::String(String::from("Nil")),
            UnitContent::Array(vec![
                UnitContent::Float64(1.0),
                UnitContent::String(String::from("Andy")),
            ]),
        ]
    }

    fn get_command_height_vec() -> Vec<(Command, ChainHeight)> {
        let grouping = GroupingLabel::new("any_grouping".as_bytes());
        let key = UnitKey::new("any_key".as_bytes());
        let map_content = get_hashmap_content();

        let result = vec![
            (
                Command::Insert {
                    grouping: grouping.clone(),
                    key: key.clone(),
                    content: map_content,
                },
                ChainHeight::new(0),
            ),
            (
                Command::Insert {
                    grouping: grouping.clone(),
                    key: key.clone(),
                    content: UnitContent::String(String::from("this is a string")),
                },
                ChainHeight::new(1),
            ),
            (
                Command::Insert {
                    grouping: grouping.clone(),
                    key: key.clone(),
                    content: UnitContent::Bool(true),
                },
                ChainHeight::new(2),
            ),
            (
                Command::Insert {
                    grouping: grouping.clone(),
                    key: key.clone(),
                    content: UnitContent::Bool(false),
                },
                ChainHeight::new(3),
            ),
            (
                Command::Insert {
                    grouping: grouping.clone(),
                    key: key.clone(),
                    content: UnitContent::Nil,
                },
                ChainHeight::new(4),
            ),
            (
                Command::Insert {
                    grouping: grouping.clone(),
                    key: key.clone(),
                    content: UnitContent::String(String::from("true")),
                },
                ChainHeight::new(5),
            ),
            (
                Command::Insert {
                    grouping: grouping.clone(),
                    key: key.clone(),
                    content: UnitContent::String(String::from("false")),
                },
                ChainHeight::new(6),
            ),
            (
                Command::Insert {
                    grouping: grouping.clone(),
                    key: key.clone(),
                    content: UnitContent::String(String::from("Nil")),
                },
                ChainHeight::new(7),
            ),
            (
                Command::Insert {
                    grouping: grouping.clone(),
                    key: key.clone(),
                    content: UnitContent::Array(vec![
                        UnitContent::Float64(1.0),
                        UnitContent::String(String::from("Andy")),
                    ]),
                },
                ChainHeight::new(8),
            ),
            (Command::RemoveAll, ChainHeight::new(9)),
            (
                Command::RemoveOne {
                    grouping: grouping.clone(),
                    key: key.clone(),
                },
                ChainHeight::new(10),
            ),
            (
                Command::RevertOne {
                    grouping: grouping.clone(),
                    key: key.clone(),
                    height: ChainHeight::new(4),
                },
                ChainHeight::new(11),
            ),
        ];

        return result;
    }

    #[test]
    fn test_outcome_reversible() {
        let log = get_command_height_vec();
        let contents = get_contents();
        let outcomes = vec![
            Outcome::Select(contents),
            Outcome::InspectOne(log.clone()),
            Outcome::InspectAll(log),
            Outcome::InsertSuccess,
            Outcome::RevertOneSuccess,
            Outcome::RevertAllSuccess,
            Outcome::RemoveOneSuccess,
            Outcome::RemoveAllSuccess,
            Outcome::CreateTransaction(TransactionId::new(1)),
            Outcome::TransactionalInsertSuccess,
            Outcome::TransactionalRevertOneSuccess,
            Outcome::TransactionalRevertOneSuccess,
            Outcome::TransactionalRemoveOneSuccess,
            Outcome::TransactionCommitSuccess,
            Outcome::TransactionAbortSuccess,
        ];

        for outcome in outcomes.iter() {
            let outcome_bytes = outcome.marshal();
            let (actual_output, offset) = Outcome::parse(&outcome_bytes).unwrap();
            assert_eq!(outcome, &actual_output);
            assert_eq!(outcome_bytes.len(), offset);
        }
    }
}
