use std::convert::TryFrom;
use std::string::FromUtf8Error;

use crate::storage::chain_height::ChainHeight;
use crate::storage::executor::filter::{Filter, FilterError};
use crate::storage::executor::grouping_label::{GroupingLabel, GroupingLabelError};
use crate::storage::executor::unit_content::{UnitContent, UnitContentError};
use crate::storage::executor::unit_key::{UnitKey, UnitKeyError};
use crate::storage::instruction::Instruction;
use crate::storage::transaction_manager::TransactionId;
use crate::utils::ints::{u64_to_u8_array, u8_array_to_u64};

#[derive(Debug)]
pub enum CommandError {
    GroupingErr(GroupingLabelError),
    UnitContentErr(UnitContentError),
    InvalidPrefix,
    SelectConditionErr(SelectConditionError),
    UnitKeyError(UnitKeyError),
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

#[derive(Debug, PartialEq, Clone)]
pub enum SelectCondition {
    Key(GroupingLabel, UnitKey, Option<TransactionId>),
    UnconditionalMatch(GroupingLabel),
    Filter(GroupingLabel, Filter),
    AllGrouping,
}

#[derive(Debug)]
pub enum SelectConditionPrefix {
    KeyWithTransactionId = 0x00,
    KeyWithoutTransactionId = 0x01,
    UnconditionalMatch = 0x02,
    Filter = 0x03,
    AllGrouping = 0x04,
}

#[derive(Debug)]
pub enum SelectConditionError {
    InvalidPrefix,
    UnitKeyError(UnitKeyError),
    FromUtf8Error(FromUtf8Error),
    ParseFilterStringError(FilterError),
    GroupingError(GroupingLabelError),
}

impl From<FromUtf8Error> for SelectConditionError {
    fn from(error: FromUtf8Error) -> SelectConditionError {
        return SelectConditionError::FromUtf8Error(error);
    }
}

impl From<FilterError> for SelectConditionError {
    fn from(error: FilterError) -> SelectConditionError {
        return SelectConditionError::ParseFilterStringError(error);
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
        } else if prefix == SelectConditionPrefix::Filter as u8 {
            let (grouping, offset) = GroupingLabel::parse(&data[position..])?;
            position += offset;
            let (filter, offset) = Filter::parse(&data[position..])?;
            position += offset;
            return Ok((SelectCondition::Filter(grouping, filter), position));
        } else if prefix == SelectConditionPrefix::AllGrouping as u8 {
            return Ok((SelectCondition::AllGrouping, position));
        } else {
            return Err(SelectConditionError::InvalidPrefix);
        }
    }
    pub fn marshal(&self) -> Vec<u8> {
        match self {
            SelectCondition::UnconditionalMatch(grouping) => {
                let mut marshaled = vec![];
                marshaled.push(SelectConditionPrefix::UnconditionalMatch as u8);
                let grouping_bytes = grouping.marshal();
                marshaled.extend_from_slice(&grouping_bytes);
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
            SelectCondition::Filter(grouping, filter) => {
                let mut marshaled = vec![];
                marshaled.push(SelectConditionPrefix::Filter as u8);

                let grouping_bytes = grouping.marshal();
                marshaled.extend_from_slice(&grouping_bytes);

                let filter_bytes = filter.marshal();
                marshaled.extend_from_slice(&filter_bytes);

                return marshaled;
            }
            SelectCondition::AllGrouping => {
                let mut marshaled = vec![];
                marshaled.push(SelectConditionPrefix::AllGrouping as u8);
                return marshaled;
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

impl ToString for Command {
    fn to_string(&self) -> String {
        match self {
            Command::Select {
                condition,
            } => {
                match condition {
                    SelectCondition::Key(grouping, key, transaction_id) => {
                        return format!(
                            "Command Select, grouping: {:?}, key: {:?}, transaction_id: {:?}",
                            grouping.to_string(),
                            key.to_string(),
                            transaction_id,
                        );
                    }
                    SelectCondition::UnconditionalMatch(grouping) => {
                        return format!("Command Select, UnconditionalMatch on grouping {:?}", grouping.to_string());
                    },
                    SelectCondition::Filter(grouping, filters) => {
                        return format!("grouping {}, filter {}", grouping, filters);
                    },
                    SelectCondition::AllGrouping => {
                        return format!("List all grouping");
                    }
                }
            },
            Command::InspectOne { grouping, key } => {
                format!("Command InspectOne, grouping: {:?}, key: {:?}", grouping.to_string(), key.to_string())
            }
            Command::InspectAll => format!("Command InspectAll"),
            Command::Insert { grouping, key, content } => format!(
                "Command Insert, grouping: {:?}, key: {:?}, content: {:?}",
                grouping.to_string(),
                key.to_string(),
                content
            ),
            Command::RevertOne { grouping, key, height } => format!(
                "Command RevertOne, grouping: {:?}, key: {:?}, height: {:?}",
                grouping.to_string(),
                key.to_string(),
                height
            ),
            Command::RevertAll { height } => {
                format!("Command RevertAll, height: {:?}", height)
            }
            Command::RemoveOne { grouping, key } => {
                format!("Command RemoveOne, grouping: {:?}, key: {:?}", grouping.to_string(), key.to_string())
            }
            Command::RemoveAll => format!("Command RemoveAll"),
            Command::CreateTransaction => format!("Command CreateTransaction"),
            Command::TransactionalInsert {
                grouping,
                key,
                content,
                transaction_id,
            } => format!(
                "Command TransactionalInsert, grouping: {:?}, key: {:?}, content: {:?}, transaction_id: {:?}",
                grouping.to_string(),
                key.to_string(),
                content,
                transaction_id
            ),
            Command::TransactionalRevertOne {
                grouping,
                key,
                height,
                transaction_id,
            } => format!(
                "Command TransactionalRevertOne, grouping: {:?}, key: {:?}, height: {:?}, transaction_id: {:?}",
                grouping.to_string(),
                key.to_string(),
                height,
                transaction_id
            ),
            Command::TransactionalRemoveOne {
                grouping,
                key,
                transaction_id,
            } => format!(
                "Command TransactionalRemoveOne, grouping: {:?}, key: {:?}, transaction_id: {:?}",
                grouping.to_string(),
                key.to_string(),
                transaction_id
            ),
            Command::TransactionCommit { transaction_id } => format!(
                "Command TransactionCommit, transaction_id {:?}",
                transaction_id
            ),
            Command::TransactionAbort { transaction_id } => format!(
                "Command TransactionAbort, transaction_id {:?}",
                transaction_id
            ),
        }
    }
}

#[cfg(test)]
mod command_tests {
    use crate::storage::chain_height::ChainHeight;
    use crate::storage::executor::command::{Command, SelectCondition};
    use crate::storage::executor::filter::parse_filter_string;
    use crate::storage::executor::grouping_label::GroupingLabel;
    use crate::storage::executor::unit_content::UnitContent;
    use crate::storage::executor::unit_key::UnitKey;
    use crate::storage::transaction_manager::TransactionId;

    #[test]
    fn select_condition_reversibility() {
        let unit_key = UnitKey::new(&[0x01, 0x02, 0x03]);
        let transaction_id = TransactionId::new(1);
        let filter = parse_filter_string(String::from("x>3")).unwrap();
        let grouping = GroupingLabel::from("any_grouping");

        let conditions = vec![
            SelectCondition::Key(grouping.clone(), unit_key.clone(), None),
            SelectCondition::Key(grouping.clone(), unit_key.clone(), Some(transaction_id)),
            SelectCondition::UnconditionalMatch(grouping.clone()),
            SelectCondition::Filter(grouping.clone(), filter),
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
    fn command_reversibility() {
        let grouping = GroupingLabel::from("any_grouping");
        let unit_key = UnitKey::new(&[0x01, 0x02, 0x03]);
        let transaction_id = TransactionId::new(1);
        let filter = parse_filter_string(String::from("x>3")).unwrap();
        let target_height = ChainHeight::new(1);

        let conditions = vec![
            SelectCondition::Key(grouping.clone(), unit_key.clone(), None),
            SelectCondition::Key(grouping.clone(), unit_key.clone(), Some(transaction_id)),
            SelectCondition::UnconditionalMatch(grouping.clone()),
            SelectCondition::Filter(grouping.clone(), filter),
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
