use crate::storage::chain_height::ChainHeight;
use crate::storage::executor::grouping_label::{GroupingLabel, GroupingLabelError};
use crate::storage::executor::unit_content::{UnitContent, UnitContentError};
use crate::storage::executor::unit_key::UnitKey;
use crate::storage::instruction::Instruction;
use crate::storage::transaction_manager::TransactionId;
use std::convert::TryFrom;

#[derive(Debug)]
pub enum CommandError {
    GroupingErr(GroupingLabelError),
    UnitContentErr(UnitContentError),
}

impl From<GroupingLabelError> for CommandError {
    fn from(error: GroupingLabelError) -> CommandError {
        CommandError::GroupingErr(error)
    }
}

impl From<UnitContentError> for CommandError {
    fn from(error: UnitContentError) -> CommandError {
        CommandError::UnitContentErr(error)
    }
}

#[derive(Debug)]
pub enum Command {
    Select {
        grouping: GroupingLabel,
        key: UnitKey,
        transaction_id: Option<TransactionId>,
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
                grouping,
                key,
                transaction_id,
            } => format!(
                "Command Select, grouping: {:?}, key: {:?}, transaction_id: {:?}",
                grouping.to_string(),
                key.to_string(),
                transaction_id
            ),
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
