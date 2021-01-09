use std::convert::TryFrom;
use std::fmt;
use std::string::FromUtf8Error;

use crate::storage::chain_height::ChainHeight;
use crate::storage::executor::grouping_label::{GroupingLabel, GroupingLabelError};
use crate::storage::executor::predicate::{Predicate, PredicateError};
use crate::storage::executor::unit_content::{UnitContent, UnitContentError};
use crate::storage::executor::unit_key::{UnitKey, UnitKeyError};
use crate::storage::instruction::Instruction;
use crate::storage::transaction_manager::TransactionId;
use crate::system_error::SystemError;
use crate::utils::ints::{u64_to_u8_array, u8_array_to_u64};
use crate::utils::varint::{varint_decode, varint_encode, VarIntError};

#[derive(Debug, Clone, PartialEq)]
pub enum CommandError {
    GroupingErr(GroupingLabelError),
    UnitContentErr(UnitContentError),
    InvalidPrefix,
    SelectConditionErr(SelectConditionError),
    UnitKeyError(UnitKeyError),
    VarIntError(VarIntError),
    ParseCommandErrorToStringError,
}

#[derive(Debug)]
pub enum CommandErrorPrefix {
    GroupingErr = 0x01,
    UnitContentErr = 0x02,
    InvalidPrefix = 0x03,
    SelectConditionErr = 0x04,
    UnitKeyError = 0x05,
    VarIntError = 0x06,
    ParseCommandErrorToStringError = 0x07,
}

impl CommandError {
    pub fn marshal(&self) -> Vec<u8> {
        match self {
            CommandError::GroupingErr(error) => {
                let mut result = vec![CommandErrorPrefix::GroupingErr as u8];
                let error_bytes = error.marshal();
                result.extend_from_slice(&error_bytes);
                return result;
            }
            CommandError::UnitContentErr(error) => {
                let mut result = vec![CommandErrorPrefix::UnitContentErr as u8];
                let error_bytes = error.marshal();
                result.extend_from_slice(&error_bytes);
                return result;
            }
            CommandError::InvalidPrefix => vec![CommandErrorPrefix::InvalidPrefix as u8],
            CommandError::SelectConditionErr(error) => {
                let mut result = vec![CommandErrorPrefix::SelectConditionErr as u8];
                let error_bytes = error.marshal();
                result.extend_from_slice(&error_bytes);
                return result;
            }
            CommandError::UnitKeyError(error) => {
                let mut result = vec![CommandErrorPrefix::UnitKeyError as u8];
                let error_bytes = error.marshal();
                result.extend_from_slice(&error_bytes);
                return result;
            }
            CommandError::VarIntError(error) => {
                let mut result = vec![CommandErrorPrefix::VarIntError as u8];
                let error_bytes = error.marshal();
                result.extend_from_slice(&error_bytes);
                return result;
            }
            CommandError::ParseCommandErrorToStringError => {
                vec![CommandErrorPrefix::ParseCommandErrorToStringError as u8]
            }
        }
    }

    pub fn parse(data: &[u8]) -> Result<(CommandError, usize), CommandError> {
        let mut position = 0;
        let prefix = data[position];
        position += 1;

        if prefix == CommandErrorPrefix::GroupingErr as u8 {
            let (error, offset) = GroupingLabelError::parse(&data[position..])?;
            position += offset;
            Ok((CommandError::GroupingErr(error), position))
        } else if prefix == CommandErrorPrefix::UnitContentErr as u8 {
            let (error, offset) = UnitContentError::parse(&data[position..])?;
            position += offset;
            Ok((CommandError::UnitContentErr(error), position))
        } else if prefix == CommandErrorPrefix::InvalidPrefix as u8 {
            Ok((CommandError::InvalidPrefix, position))
        } else if prefix == CommandErrorPrefix::SelectConditionErr as u8 {
            let (error, offset) = SelectConditionError::parse(&data[position..])?;
            position += offset;
            Ok((CommandError::SelectConditionErr(error), position))
        } else if prefix == CommandErrorPrefix::UnitKeyError as u8 {
            let (error, offset) = UnitKeyError::parse(&data[position..])?;
            position += offset;
            Ok((CommandError::UnitKeyError(error), position))
        } else if prefix == CommandErrorPrefix::VarIntError as u8 {
            let (error, offset) = VarIntError::parse(&data[position..])?;
            position += offset;
            Ok((CommandError::VarIntError(error), position))
        } else {
            Ok((CommandError::ParseCommandErrorToStringError, position))
        }
    }
}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommandError::GroupingErr(error) => {
                let error_string = format!("{}", error);
                write!(f, "{}::{}", "CommandError::GroupingErr", error_string)
            }
            CommandError::UnitContentErr(error) => {
                let error_string = format!("{}", error);
                write!(f, "{}::{}", "CommandError::UnitContentErr", error_string)
            }
            CommandError::InvalidPrefix => {
                write!(f, "{}", "CommandError::InvalidPrefix")
            }
            CommandError::SelectConditionErr(error) => {
                let error_string = format!("{}", error);
                write!(
                    f,
                    "{}::{}",
                    "CommandError::SelectConditionErr", error_string
                )
            }
            CommandError::UnitKeyError(error) => {
                let error_string = format!("{}", error);
                write!(f, "{}::{}", "CommandError::UnitKeyError", error_string)
            }
            CommandError::VarIntError(error) => {
                let error_string = format!("{}", error);
                write!(f, "{}::{}", "CommandError::VarIntError", error_string)
            }
            CommandError::ParseCommandErrorToStringError => {
                write!(f, "{}", "CommandError::ParseCommandErrorToStringError")
            }
        }
    }
}

impl From<GroupingLabelError> for CommandError {
    fn from(error: GroupingLabelError) -> CommandError {
        CommandError::GroupingErr(error)
    }
}

impl From<UnitKeyError> for CommandError {
    fn from(error: UnitKeyError) -> CommandError {
        CommandError::UnitKeyError(error)
    }
}

impl From<UnitContentError> for CommandError {
    fn from(error: UnitContentError) -> CommandError {
        CommandError::UnitContentErr(error)
    }
}

impl From<SelectConditionError> for CommandError {
    fn from(error: SelectConditionError) -> CommandError {
        CommandError::SelectConditionErr(error)
    }
}

impl From<VarIntError> for CommandError {
    fn from(error: VarIntError) -> CommandError {
        CommandError::VarIntError(error)
    }
}

impl From<FromUtf8Error> for CommandError {
    fn from(_err: FromUtf8Error) -> CommandError {
        CommandError::ParseCommandErrorToStringError
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum SelectCondition {
    Key(GroupingLabel, UnitKey, Option<TransactionId>),
    UnconditionalMatch(GroupingLabel),
    Predicate(GroupingLabel, Predicate),
    AllGrouping,
}

#[derive(Debug)]
pub enum SelectConditionPrefix {
    KeyWithTransactionId = 0x00,
    KeyWithoutTransactionId = 0x01,
    UnconditionalMatch = 0x02,
    Predicate = 0x03,
    AllGrouping = 0x04,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SelectConditionError {
    InvalidPrefix,
    UnitKeyError(UnitKeyError),
    FromUtf8Error(SystemError),
    PredicateError(PredicateError),
    GroupingError(GroupingLabelError),
    ParseSelectConditionErrorError,
    SystemError(SystemError),
}

enum SelectConditionErrorPrefix {
    InvalidPrefix = 0x01,
    UnitKeyError = 0x02,
    FromUtf8Error = 0x03,
    PredicateError = 0x04,
    GroupingError = 0x05,
    ParseSelectConditionErrorError = 0x06,
    SystemError = 0x07,
}

impl SelectConditionError {
    pub fn marshal(&self) -> Vec<u8> {
        match self {
            SelectConditionError::InvalidPrefix => {
                vec![SelectConditionErrorPrefix::InvalidPrefix as u8]
            }
            SelectConditionError::UnitKeyError(error) => {
                let mut result = vec![SelectConditionErrorPrefix::UnitKeyError as u8];
                let error_bytes = error.marshal();
                result.extend_from_slice(&error_bytes);
                result
            }
            SelectConditionError::FromUtf8Error(error) => {
                let mut result = vec![SelectConditionErrorPrefix::FromUtf8Error as u8];
                let error_bytes = error.marshal();
                result.extend_from_slice(&error_bytes);
                result
            }
            SelectConditionError::PredicateError(error) => {
                let mut result = vec![SelectConditionErrorPrefix::PredicateError as u8];
                let error_bytes = error.marshal();
                result.extend_from_slice(&error_bytes);
                result
            }
            SelectConditionError::GroupingError(error) => {
                let mut result = vec![SelectConditionErrorPrefix::GroupingError as u8];
                let error_bytes = error.marshal();
                result.extend_from_slice(&error_bytes);
                result
            }
            SelectConditionError::ParseSelectConditionErrorError => {
                vec![SelectConditionErrorPrefix::ParseSelectConditionErrorError as u8]
            }
            SelectConditionError::SystemError(error) => {
                let mut result = vec![SelectConditionErrorPrefix::SystemError as u8];
                let error_bytes = error.marshal();
                result.extend_from_slice(&error_bytes);
                result
            }
        }
    }

    pub fn parse(data: &[u8]) -> Result<(SelectConditionError, usize), SelectConditionError> {
        let mut position = 0;
        let prefix = data[position];
        position += 1;

        if prefix == SelectConditionErrorPrefix::InvalidPrefix as u8 {
            Ok((SelectConditionError::InvalidPrefix, position))
        } else if prefix == SelectConditionErrorPrefix::UnitKeyError as u8 {
            let (error, offset) = UnitKeyError::parse(&data[position..])?;
            position += offset;
            Ok((SelectConditionError::UnitKeyError(error), position))
        } else if prefix == SelectConditionErrorPrefix::FromUtf8Error as u8 {
            let (error, offset) = SystemError::parse(&data[position..])?;
            position += offset;
            Ok((SelectConditionError::FromUtf8Error(error), position))
        } else if prefix == SelectConditionErrorPrefix::PredicateError as u8 {
            let (error, offset) = PredicateError::parse(&data[position..])?;
            position += offset;
            Ok((SelectConditionError::PredicateError(error), position))
        } else if prefix == SelectConditionErrorPrefix::GroupingError as u8 {
            let (error, offset) = GroupingLabelError::parse(&data[position..])?;
            position += offset;
            Ok((SelectConditionError::GroupingError(error), position))
        } else if prefix == SelectConditionErrorPrefix::SystemError as u8 {
            let (error, offset) = SystemError::parse(&data[position..])?;
            position += offset;
            Ok((SelectConditionError::SystemError(error), position))
        } else {
            Ok((
                SelectConditionError::ParseSelectConditionErrorError,
                position,
            ))
        }
    }
}

impl fmt::Display for SelectConditionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SelectConditionError::InvalidPrefix => {
                write!(f, "{}", "SelectConditionError::InvalidPrefix")
            }
            SelectConditionError::UnitKeyError(error) => {
                write!(f, "{}::{}", "SelectConditionError::UnitKeyError", error)
            }
            SelectConditionError::FromUtf8Error(error) => {
                write!(f, "{}::{}", "SelectConditionError::FromUtf8Error", error)
            }
            SelectConditionError::PredicateError(error) => {
                write!(f, "{}::{}", "SelectConditionError::PredicateError", error)
            }
            SelectConditionError::GroupingError(error) => {
                write!(f, "{}::{}", "SelectConditionError::GroupingError", error)
            }
            SelectConditionError::ParseSelectConditionErrorError => write!(
                f,
                "{}",
                "SelectConditionError::ParseSelectConditionErrorError",
            ),
            SelectConditionError::SystemError(error) => {
                write!(f, "{}::{}", "SelectConditionError::SystemError", error,)
            }
        }
    }
}

impl From<FromUtf8Error> for SelectConditionError {
    fn from(_error: FromUtf8Error) -> SelectConditionError {
        return SelectConditionError::FromUtf8Error(SystemError::FromUtf8Error);
    }
}

impl From<SystemError> for SelectConditionError {
    fn from(error: SystemError) -> SelectConditionError {
        return SelectConditionError::SystemError(error);
    }
}

impl From<PredicateError> for SelectConditionError {
    fn from(error: PredicateError) -> SelectConditionError {
        return SelectConditionError::PredicateError(error);
    }
}

impl From<UnitKeyError> for SelectConditionError {
    fn from(error: UnitKeyError) -> SelectConditionError {
        return SelectConditionError::UnitKeyError(error);
    }
}

impl From<GroupingLabelError> for SelectConditionError {
    fn from(error: GroupingLabelError) -> SelectConditionError {
        return SelectConditionError::GroupingError(error);
    }
}

impl SelectCondition {
    pub fn parse(data: &[u8]) -> Result<(Self, usize), SelectConditionError> {
        let mut position = 0;
        let prefix = data[position];
        position += 1;

        if prefix == SelectConditionPrefix::KeyWithTransactionId as u8
            || prefix == SelectConditionPrefix::KeyWithoutTransactionId as u8
        {
            let (grouping, offset) = GroupingLabel::parse(&data[position..])?;
            position += offset;

            let transaction_id = {
                if prefix == SelectConditionPrefix::KeyWithoutTransactionId as u8 {
                    None
                } else {
                    let transaction_id = u8_array_to_u64(&[
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
                    Some(TransactionId::new(transaction_id))
                }
            };

            let (unit_key, offset) = UnitKey::parse(&data[position..])?;
            position += offset;

            return Ok((
                SelectCondition::Key(grouping, unit_key, transaction_id),
                position,
            ));
        } else if prefix == SelectConditionPrefix::UnconditionalMatch as u8 {
            let (grouping, offset) = GroupingLabel::parse(&data[position..])?;
            position += offset;
            return Ok((SelectCondition::UnconditionalMatch(grouping), position));
        } else if prefix == SelectConditionPrefix::Predicate as u8 {
            let (grouping, offset) = GroupingLabel::parse(&data[position..])?;
            position += offset;
            let (predicate, offset) = Predicate::parse(&data[position..])?;
            position += offset;
            return Ok((SelectCondition::Predicate(grouping, predicate), position));
        } else if prefix == SelectConditionPrefix::AllGrouping as u8 {
            return Ok((SelectCondition::AllGrouping, position));
        } else {
            return Err(SelectConditionError::InvalidPrefix);
        }
    }
    pub fn marshal(&self) -> Vec<u8> {
        match self {
            SelectCondition::UnconditionalMatch(grouping) => {
                let mut marshaled = vec![SelectConditionPrefix::UnconditionalMatch as u8];
                marshaled.extend(grouping.marshal());
                return marshaled;
            }
            SelectCondition::Key(grouping, key, transaction_id) => {
                let mut marshaled = vec![];

                if let Some(transaction_id) = transaction_id {
                    marshaled.push(SelectConditionPrefix::KeyWithTransactionId as u8);

                    let grouping_bytes = grouping.marshal();
                    marshaled.extend_from_slice(&grouping_bytes);

                    let transaction_id_bytes = u64_to_u8_array(transaction_id.as_u64());
                    marshaled.extend_from_slice(&transaction_id_bytes);
                } else {
                    marshaled.push(SelectConditionPrefix::KeyWithoutTransactionId as u8);
                    let grouping_bytes = grouping.marshal();
                    marshaled.extend_from_slice(&grouping_bytes);
                }

                let key_bytes = key.marshal();
                marshaled.extend_from_slice(&key_bytes);

                return marshaled;
            }
            SelectCondition::Predicate(grouping, predicate) => {
                let mut result = vec![SelectConditionPrefix::Predicate as u8];
                result.extend(grouping.marshal());
                result.extend(predicate.serialize());
                return result;
            }
            SelectCondition::AllGrouping => {
                return vec![SelectConditionPrefix::AllGrouping as u8];
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Command {
    Select {
        condition: SelectCondition,
    },
    InspectOne {
        grouping: GroupingLabel,
        key: UnitKey,
    },
    InspectAll,
    Insert {
        grouping: GroupingLabel,
        key: UnitKey,
        content: UnitContent,
    },
    RevertOne {
        grouping: GroupingLabel,
        key: UnitKey,
        height: ChainHeight,
    },
    RevertAll {
        height: ChainHeight,
    },
    RemoveOne {
        grouping: GroupingLabel,
        key: UnitKey,
    },
    RemoveAll,
    RemoveGroupings {
        groupings: Vec<GroupingLabel>,
    },
    CreateTransaction,
    TransactionalInsert {
        grouping: GroupingLabel,
        key: UnitKey,
        content: UnitContent,
        transaction_id: TransactionId,
    },
    TransactionalRevertOne {
        grouping: GroupingLabel,
        key: UnitKey,
        height: ChainHeight,
        transaction_id: TransactionId,
    },
    TransactionalRemoveOne {
        grouping: GroupingLabel,
        key: UnitKey,
        transaction_id: TransactionId,
    },
    TransactionCommit {
        transaction_id: TransactionId,
    },
    TransactionAbort {
        transaction_id: TransactionId,
    },
}

#[derive(Debug)]
pub enum CommandPrefix {
    Select = 0x01,
    InspectOne = 0x02,
    InspectAll = 0x03,
    Insert = 0x04,
    RevertOne = 0x05,
    RevertAll = 0x06,
    RemoveOne = 0x07,
    RemoveAll = 0x08,
    RemoveGroupings = 0x09,

    TransactionalInsert = 0x54,
    TransactionalRevertOne = 0x55,
    TransactionalRemoveOne = 0x57,

    CreateTransaction = 0xc0,
    TransactionCommit = 0xc1,
    TransactionAbort = 0xc2,
}

impl Command {
    pub fn parse(data: &[u8]) -> Result<(Command, usize), CommandError> {
        let mut position = 0;
        let prefix = data[position];
        position += 1;

        if prefix == CommandPrefix::Select as u8 {
            let (condition, offset) = SelectCondition::parse(&data[position..])?;
            position += offset;

            let command = Command::Select { condition };
            return Ok((command, position));
        } else if prefix == CommandPrefix::InspectOne as u8 {
            let (grouping, offset) = GroupingLabel::parse(&data[position..])?;
            position += offset;

            let (unit_key, offset) = UnitKey::parse(&data[position..])?;
            position += offset;

            let command = Command::InspectOne {
                grouping,
                key: unit_key,
            };

            return Ok((command, position));
        } else if prefix == CommandPrefix::InspectAll as u8 {
            return Ok((Command::InspectAll, position));
        } else if prefix == CommandPrefix::Insert as u8 {
            let (grouping, offset) = GroupingLabel::parse(&data[position..])?;
            position += offset;

            let (unit_key, offset) = UnitKey::parse(&data[position..])?;
            position += offset;

            let (content, offset) = UnitContent::parse(&data[position..])?;
            position += offset;

            let command = Command::Insert {
                grouping,
                key: unit_key,
                content,
            };

            return Ok((command, position));
        } else if prefix == CommandPrefix::RevertOne as u8 {
            let (grouping, offset) = GroupingLabel::parse(&data[position..])?;
            position += offset;

            let (unit_key, offset) = UnitKey::parse(&data[position..])?;
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
            position += 8;
            let height = ChainHeight::new(height);

            let command = Command::RevertOne {
                grouping,
                key: unit_key,
                height,
            };

            return Ok((command, position));
        } else if prefix == CommandPrefix::RevertAll as u8 {
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
            position += 8;
            let height = ChainHeight::new(height);

            let command = Command::RevertAll { height };

            return Ok((command, position));
        } else if prefix == CommandPrefix::RemoveOne as u8 {
            let (grouping, offset) = GroupingLabel::parse(&data[position..])?;
            position += offset;

            let (unit_key, offset) = UnitKey::parse(&data[position..])?;
            position += offset;

            let command = Command::RemoveOne {
                grouping,
                key: unit_key,
            };

            return Ok((command, position));
        } else if prefix == CommandPrefix::RemoveAll as u8 {
            let command = Command::RemoveAll;
            return Ok((command, position));
        } else if prefix == CommandPrefix::RemoveGroupings as u8 {
            let (total_length, offset) = varint_decode(&data[position..])?;
            position += offset;

            let mut parsed_length = 0;
            let mut groupings = vec![];

            while parsed_length < total_length as usize {
                let (grouping, offset) = GroupingLabel::parse(&data[position..])?;
                position += offset;
                parsed_length += offset;
                groupings.push(grouping);
            }

            let command = Command::RemoveGroupings { groupings };
            return Ok((command, position));
        } else if prefix == CommandPrefix::CreateTransaction as u8 {
            let command = Command::CreateTransaction;
            return Ok((command, position));
        } else if prefix == CommandPrefix::TransactionalInsert as u8 {
            let (grouping, offset) = GroupingLabel::parse(&data[position..])?;
            position += offset;

            let (unit_key, offset) = UnitKey::parse(&data[position..])?;
            position += offset;

            let (content, offset) = UnitContent::parse(&data[position..])?;
            position += offset;

            let transaction_id = u8_array_to_u64(&[
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
            let transaction_id = TransactionId::new(transaction_id);

            let command = Command::TransactionalInsert {
                grouping,
                key: unit_key,
                content,
                transaction_id,
            };

            return Ok((command, position));
        } else if prefix == CommandPrefix::TransactionalRevertOne as u8 {
            let (grouping, offset) = GroupingLabel::parse(&data[position..])?;
            position += offset;

            let (unit_key, offset) = UnitKey::parse(&data[position..])?;
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
            position += 8;
            let height = ChainHeight::new(height);

            let transaction_id = u8_array_to_u64(&[
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
            let transaction_id = TransactionId::new(transaction_id);

            let command = Command::TransactionalRevertOne {
                grouping,
                key: unit_key,
                height,
                transaction_id,
            };

            return Ok((command, position));
        } else if prefix == CommandPrefix::TransactionalRemoveOne as u8 {
            let (grouping, offset) = GroupingLabel::parse(&data[position..])?;
            position += offset;

            let (unit_key, offset) = UnitKey::parse(&data[position..])?;
            position += offset;

            let transaction_id = u8_array_to_u64(&[
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
            let transaction_id = TransactionId::new(transaction_id);

            let command = Command::TransactionalRemoveOne {
                grouping,
                key: unit_key,
                transaction_id,
            };

            return Ok((command, position));
        } else if prefix == CommandPrefix::TransactionCommit as u8 {
            let transaction_id = u8_array_to_u64(&[
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
            let transaction_id = TransactionId::new(transaction_id);

            let command = Command::TransactionCommit { transaction_id };

            return Ok((command, position));
        } else if prefix == CommandPrefix::TransactionAbort as u8 {
            let transaction_id = u8_array_to_u64(&[
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
            let transaction_id = TransactionId::new(transaction_id);

            let command = Command::TransactionAbort { transaction_id };

            return Ok((command, position));
        } else {
            return Err(CommandError::InvalidPrefix);
        }
    }
    pub fn marshal(&self) -> Vec<u8> {
        match self {
            Command::Insert {
                grouping,
                key,
                content,
            } => {
                let prefix = CommandPrefix::Insert as u8;
                let grouping_bytes = grouping.marshal();
                let key_bytes = key.marshal();
                let content_bytes = content.marshal();

                let mut result = vec![];
                result.push(prefix);
                result.extend_from_slice(&grouping_bytes);
                result.extend_from_slice(&key_bytes);
                result.extend_from_slice(&content_bytes);
                return result;
            }
            Command::Select { condition } => {
                let prefix = CommandPrefix::Select as u8;
                let condition_bytes = condition.marshal();

                let mut result = vec![];
                result.push(prefix);
                result.extend_from_slice(&condition_bytes);
                return result;
            }
            Command::InspectOne { grouping, key } => {
                let prefix = CommandPrefix::InspectOne as u8;
                let grouping_bytes = grouping.marshal();
                let key_bytes = key.marshal();

                let mut result = vec![];
                result.push(prefix);
                result.extend_from_slice(&grouping_bytes);
                result.extend_from_slice(&key_bytes);
                return result;
            }
            Command::InspectAll => {
                let prefix = CommandPrefix::InspectAll as u8;
                let result = vec![prefix];
                return result;
            }
            Command::RevertOne {
                grouping,
                key,
                height,
            } => {
                let prefix = CommandPrefix::RevertOne as u8;
                let grouping_bytes = grouping.marshal();
                let key_bytes = key.marshal();
                let height_bytes = u64_to_u8_array(height.as_u64());

                let mut result = vec![];
                result.push(prefix);
                result.extend_from_slice(&grouping_bytes);
                result.extend_from_slice(&key_bytes);
                result.extend_from_slice(&height_bytes);
                return result;
            }
            Command::RevertAll { height } => {
                let prefix = CommandPrefix::RevertAll as u8;
                let height_bytes = u64_to_u8_array(height.as_u64());

                let mut result = vec![];
                result.push(prefix);
                result.extend_from_slice(&height_bytes);
                return result;
            }
            Command::RemoveOne { grouping, key } => {
                let prefix = CommandPrefix::RemoveOne as u8;
                let grouping_bytes = grouping.marshal();
                let key_bytes = key.marshal();

                let mut result = vec![];
                result.push(prefix);
                result.extend_from_slice(&grouping_bytes);
                result.extend_from_slice(&key_bytes);
                return result;
            }
            Command::RemoveAll => {
                let prefix = CommandPrefix::RemoveAll as u8;
                let result = vec![prefix];
                return result;
            }
            Command::RemoveGroupings { groupings } => {
                let prefix = CommandPrefix::RemoveGroupings as u8;
                let groupings_bytes: Vec<Vec<u8>> = groupings
                    .iter()
                    .map(|grouping| grouping.marshal())
                    .collect();

                let mut result = vec![prefix];

                let serialized_bytes = groupings_bytes.into_iter().flatten().collect::<Vec<u8>>();

                let total_length = serialized_bytes.len();
                let number_varint = varint_encode(total_length as u64);
                result.extend_from_slice(&number_varint);
                result.extend_from_slice(&serialized_bytes);

                return result;
            }
            Command::CreateTransaction => {
                let prefix = CommandPrefix::CreateTransaction as u8;
                let result = vec![prefix];
                return result;
            }
            Command::TransactionalInsert {
                grouping,
                key,
                content,
                transaction_id,
            } => {
                let prefix = CommandPrefix::TransactionalInsert as u8;
                let grouping_bytes = grouping.marshal();
                let key_bytes = key.marshal();
                let content_bytes = content.marshal();
                let transaction_id_bytes = u64_to_u8_array(transaction_id.as_u64());

                let mut result = vec![];
                result.push(prefix);
                result.extend_from_slice(&grouping_bytes);
                result.extend_from_slice(&key_bytes);
                result.extend_from_slice(&content_bytes);
                result.extend_from_slice(&transaction_id_bytes);
                return result;
            }
            Command::TransactionalRevertOne {
                grouping,
                key,
                height,
                transaction_id,
            } => {
                let prefix = CommandPrefix::TransactionalRevertOne as u8;
                let grouping_bytes = grouping.marshal();
                let key_bytes = key.marshal();
                let height_bytes = u64_to_u8_array(height.as_u64());
                let transaction_id_bytes = u64_to_u8_array(transaction_id.as_u64());

                let mut result = vec![];
                result.push(prefix);
                result.extend_from_slice(&grouping_bytes);
                result.extend_from_slice(&key_bytes);
                result.extend_from_slice(&height_bytes);
                result.extend_from_slice(&transaction_id_bytes);
                return result;
            }
            Command::TransactionalRemoveOne {
                grouping,
                key,
                transaction_id,
            } => {
                let prefix = CommandPrefix::TransactionalRemoveOne as u8;
                let grouping_bytes = grouping.marshal();
                let key_bytes = key.marshal();
                let transaction_id_bytes = u64_to_u8_array(transaction_id.as_u64());

                let mut result = vec![];
                result.push(prefix);
                result.extend_from_slice(&grouping_bytes);
                result.extend_from_slice(&key_bytes);
                result.extend_from_slice(&transaction_id_bytes);
                return result;
            }
            Command::TransactionCommit { transaction_id } => {
                let prefix = CommandPrefix::TransactionCommit as u8;
                let transaction_id_bytes = u64_to_u8_array(transaction_id.as_u64());

                let mut result = vec![];
                result.push(prefix);
                result.extend_from_slice(&transaction_id_bytes);
                return result;
            }
            Command::TransactionAbort { transaction_id } => {
                let prefix = CommandPrefix::TransactionAbort as u8;
                let transaction_id_bytes = u64_to_u8_array(transaction_id.as_u64());

                let mut result = vec![];
                result.push(prefix);
                result.extend_from_slice(&transaction_id_bytes);
                return result;
            }
        }
    }
}

impl TryFrom<&Instruction> for Command {
    type Error = CommandError;

    fn try_from(instruction: &Instruction) -> Result<Self, Self::Error> {
        match instruction {
            Instruction::Set { key, value } => {
                let (grouping, offset) = GroupingLabel::parse(&key.as_bytes())?;
                let unit_key = UnitKey::from(&key.as_bytes()[offset..]);
                let (content, _) = UnitContent::parse(value.as_bytes())?;
                let command = Command::Insert {
                    grouping,
                    key: unit_key,
                    content,
                };
                return Ok(command);
            }
            Instruction::RevertOne { key, height } => {
                let (grouping, offset) = GroupingLabel::parse(&key.as_bytes())?;
                let unit_key = UnitKey::from(&key.as_bytes()[offset..]);
                let command = Command::RevertOne {
                    grouping,
                    key: unit_key,
                    height: height.to_owned(),
                };
                return Ok(command);
            }
            Instruction::RevertAll { height } => {
                let command = Command::RevertAll {
                    height: height.to_owned(),
                };
                return Ok(command);
            }
            Instruction::RemoveOne { key } => {
                let (grouping, offset) = GroupingLabel::parse(&key.as_bytes())?;
                let unit_key = UnitKey::from(&key.as_bytes()[offset..]);
                let command = Command::RemoveOne {
                    grouping,
                    key: unit_key,
                };
                return Ok(command);
            }
            Instruction::RemoveAll => {
                let command = Command::RemoveAll;
                return Ok(command);
            }
            Instruction::TransactionStart { transaction_id: _ } => {
                let command = Command::CreateTransaction;
                return Ok(command);
            }
            Instruction::TransactionalSet {
                key,
                value,
                transaction_id,
            } => {
                let (grouping, offset) = GroupingLabel::parse(&key.as_bytes())?;
                let unit_key = UnitKey::from(&key.as_bytes()[offset..]);
                let (content, _) = UnitContent::parse(value.as_bytes())?;
                let command = Command::TransactionalInsert {
                    grouping,
                    key: unit_key,
                    content,
                    transaction_id: transaction_id.to_owned(),
                };
                return Ok(command);
            }
            Instruction::TransactionalRevertOne {
                key,
                height,
                transaction_id,
            } => {
                let (grouping, offset) = GroupingLabel::parse(&key.as_bytes())?;
                let unit_key = UnitKey::from(&key.as_bytes()[offset..]);
                let command = Command::TransactionalRevertOne {
                    grouping,
                    key: unit_key,
                    height: height.to_owned(),
                    transaction_id: transaction_id.to_owned(),
                };
                return Ok(command);
            }
            Instruction::TransactionalRemoveOne {
                key,
                transaction_id,
            } => {
                let (grouping, offset) = GroupingLabel::parse(&key.as_bytes())?;
                let unit_key = UnitKey::from(&key.as_bytes()[offset..]);
                let command = Command::TransactionalRemoveOne {
                    grouping,
                    key: unit_key,
                    transaction_id: transaction_id.to_owned(),
                };
                return Ok(command);
            }
            Instruction::TransactionCommit { transaction_id } => {
                let command = Command::TransactionCommit {
                    transaction_id: transaction_id.to_owned(),
                };
                return Ok(command);
            }
            Instruction::TransactionAbort { transaction_id } => {
                let command = Command::TransactionAbort {
                    transaction_id: transaction_id.to_owned(),
                };
                return Ok(command);
            }
        }
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Command::Select {
                condition,
            } => {
                match condition {
                    SelectCondition::Key(grouping, key, transaction_id) => {
                        match transaction_id {
                            Some(transaction_id) => write!(
                                f,
                                "Command Select, grouping: {}, key: {}, transaction_id: {}",
                                grouping,
                                key,
                                transaction_id,
                            ),
                            None => write!(
                                f,
                                "Command Select, grouping: {}, key: {}, transaction_id: None",
                                grouping,
                                key,
                            ),
                        }
                    }
                    SelectCondition::UnconditionalMatch(grouping) => {
                        write!(f, "Command Select, UnconditionalMatch on grouping {}", grouping)
                    },
                    SelectCondition::Predicate(grouping, predicate) => {
                        write!(f, "grouping {}, predicate {}", grouping, predicate)
                    },
                    SelectCondition::AllGrouping => {
                        write!(f, "List all grouping")
                    }
                }
            },
            Command::InspectOne { grouping, key } => {
                write!(f, "Command InspectOne, grouping: {}, key: {}", grouping, key)
            }
            Command::InspectAll => write!(f, "Command InspectAll"),
            Command::Insert { grouping, key, content } => write!(
                f,
                "Command Insert, grouping: {}, key: {}, content: {}",
                grouping,
                key,
                content
            ),
            Command::RevertOne { grouping, key, height } => write!(
                f,
                "Command RevertOne, grouping: {}, key: {}, height: {}",
                grouping,
                key,
                height
            ),
            Command::RevertAll { height } => {
                write!(f, "Command RevertAll, height: {}", height)
            }
            Command::RemoveOne { grouping, key } => {
                write!(f, "Command RemoveOne, grouping: {}, key: {}", grouping, key)
            }
            Command::RemoveAll => write!(f, "Command RemoveAll"),
            Command::RemoveGroupings {groupings} => {
                let grouping_str_vec: Vec<String> = groupings
                    .into_iter()
                    .map(|grouping| format!("{}", grouping))
                    .collect();
                write!(f, "Command RemoveGroupings {}", grouping_str_vec.join("\r\n"))
            },
            Command::CreateTransaction => write!(f, "Command CreateTransaction"),
            Command::TransactionalInsert {
                grouping,
                key,
                content,
                transaction_id,
            } => write!(
                f,
                "Command TransactionalInsert, grouping: {}, key: {}, content: {}, transaction_id: {}",
                grouping,
                key,
                content,
                transaction_id
            ),
            Command::TransactionalRevertOne {
                grouping,
                key,
                height,
                transaction_id,
            } => write!(
                f,
                "Command TransactionalRevertOne, grouping: {}, key: {}, height: {}, transaction_id: {}",
                grouping,
                key,
                height,
                transaction_id
            ),
            Command::TransactionalRemoveOne {
                grouping,
                key,
                transaction_id,
            } => write!(
                f,
                "Command TransactionalRemoveOne, grouping: {}, key: {}, transaction_id: {}",
                grouping,
                key,
                transaction_id
            ),
            Command::TransactionCommit { transaction_id } => write!(
                f,
                "Command TransactionCommit, transaction_id {}",
                transaction_id
            ),
            Command::TransactionAbort { transaction_id } => write!(
                f,
                "Command TransactionAbort, transaction_id {}",
                transaction_id
            ),
        }
    }
}

#[cfg(test)]
mod command_tests {
    use crate::storage::chain_height::ChainHeight;
    use crate::storage::executor::command::{Command, SelectCondition};
    use crate::storage::executor::grouping_label::GroupingLabel;
    use crate::storage::executor::predicate::Predicate;
    use crate::storage::executor::unit_content::UnitContent;
    use crate::storage::executor::unit_key::UnitKey;
    use crate::storage::transaction_manager::TransactionId;
    use immuxsys_dev_utils::dev_utils::{
        get_command_errors, get_select_condition_errors, CommandError, SelectConditionError,
    };

    #[test]
    fn command_error_reversibility() {
        let command_errors = get_command_errors();

        for expected_error in command_errors {
            let error_bytes = expected_error.marshal();
            let (actual_error, _) = CommandError::parse(&error_bytes).unwrap();
            assert_eq!(expected_error, actual_error);
        }
    }

    #[test]
    fn select_condition_reversibility() {
        let unit_key = UnitKey::new(&[0x01, 0x02, 0x03]);
        let transaction_id = TransactionId::new(1);
        let predicate = Predicate::parse_str("this>3").unwrap();
        let grouping = GroupingLabel::from("grouping");

        let conditions = vec![
            SelectCondition::Key(grouping.clone(), unit_key.clone(), None),
            SelectCondition::Key(grouping.clone(), unit_key.clone(), Some(transaction_id)),
            SelectCondition::UnconditionalMatch(grouping.clone()),
            SelectCondition::Predicate(grouping.clone(), predicate),
            SelectCondition::AllGrouping,
        ];

        for condition in conditions.iter() {
            let condition_bytes = condition.marshal();
            let (actual_output, offset) = SelectCondition::parse(&condition_bytes).unwrap();
            assert_eq!(condition, &actual_output);
            assert_eq!(condition_bytes.len(), offset);
        }
    }

    #[test]
    fn select_condition_error_reversibility() {
        let select_condition_errors = get_select_condition_errors();

        for expected_error in select_condition_errors {
            let error_bytes = expected_error.marshal();
            let (actual_error, _) = SelectConditionError::parse(&error_bytes).unwrap();
            assert_eq!(expected_error, actual_error);
        }
    }

    #[test]
    fn command_reversibility() {
        let grouping = GroupingLabel::from("any_grouping");
        let unit_key = UnitKey::new(&[0x01, 0x02, 0x03]);
        let transaction_id = TransactionId::new(1);
        let predicate = Predicate::parse_str("x>3").unwrap();
        let target_height = ChainHeight::new(1);

        let conditions = vec![
            SelectCondition::Key(grouping.clone(), unit_key.clone(), None),
            SelectCondition::Key(grouping.clone(), unit_key.clone(), Some(transaction_id)),
            SelectCondition::UnconditionalMatch(grouping.clone()),
            SelectCondition::Predicate(grouping.clone(), predicate),
            SelectCondition::AllGrouping,
        ];

        let select_commands: Vec<Command> = conditions
            .iter()
            .map(|condition| Command::Select {
                condition: condition.clone(),
            })
            .collect();

        let mut commands = vec![
            Command::InspectOne {
                grouping: grouping.clone(),
                key: unit_key.clone(),
            },
            Command::InspectAll,
            Command::Insert {
                grouping: grouping.clone(),
                key: unit_key.clone(),
                content: UnitContent::String(String::from("123")),
            },
            Command::RevertOne {
                grouping: grouping.clone(),
                key: unit_key.clone(),
                height: target_height.clone(),
            },
            Command::RevertAll {
                height: target_height.clone(),
            },
            Command::RemoveOne {
                grouping: grouping.clone(),
                key: unit_key.clone(),
            },
            Command::RemoveAll,
            Command::TransactionalInsert {
                transaction_id: transaction_id.clone(),
                grouping: grouping.clone(),
                key: unit_key.clone(),
                content: UnitContent::String(String::from("123")),
            },
            Command::TransactionalRevertOne {
                transaction_id: transaction_id.clone(),
                grouping: grouping.clone(),
                key: unit_key.clone(),
                height: target_height.clone(),
            },
            Command::TransactionalRemoveOne {
                transaction_id: transaction_id.clone(),
                grouping: grouping.clone(),
                key: unit_key.clone(),
            },
            Command::CreateTransaction,
            Command::TransactionCommit {
                transaction_id: transaction_id.clone(),
            },
            Command::TransactionAbort {
                transaction_id: transaction_id.clone(),
            },
        ];

        commands.extend_from_slice(&select_commands);

        for command in commands.iter() {
            let command_bytes = &command.marshal();
            let (actual_output, _) = Command::parse(&command_bytes).unwrap();
            assert_eq!(*command, actual_output);
        }
    }
}
