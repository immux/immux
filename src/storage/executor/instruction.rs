use crate::storage::chain_height::ChainHeight;
use crate::storage::command::Command;
use crate::storage::executor::unit_content::{UnitContent, UnitContentError};
use crate::storage::executor::unit_key::UnitKey;
use crate::storage::transaction_manager::TransactionId;
use std::convert::TryFrom;

#[derive(Debug)]
pub enum Instruction {
    Select {
        key: UnitKey,
        transaction_id: Option<TransactionId>,
    },
    InspectOne {
        key: UnitKey,
    },
    InspectAll,
    Insert {
        key: UnitKey,
        content: UnitContent,
    },
    RevertOne {
        key: UnitKey,
        height: ChainHeight,
    },
    RevertAll {
        height: ChainHeight,
    },
    RemoveOne {
        key: UnitKey,
    },
    RemoveAll,
    CreateTransaction,
    TransactionalInsert {
        key: UnitKey,
        content: UnitContent,
        transaction_id: TransactionId,
    },
    TransactionalRevertOne {
        key: UnitKey,
        height: ChainHeight,
        transaction_id: TransactionId,
    },
    TransactionalRemoveOne {
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

impl TryFrom<&Command> for Instruction {
    type Error = UnitContentError;

    fn try_from(command: &Command) -> Result<Self, Self::Error> {
        match command {
            Command::Set { key, value } => {
                let unit_key = UnitKey::from(key.as_bytes());
                let (content, _) = UnitContent::parse(value.as_bytes())?;
                let instruction = Instruction::Insert { key: unit_key, content };
                return Ok(instruction);
            }
            Command::RevertOne { key, height } => {
                let unit_key = UnitKey::from(key.as_bytes());
                let instruction = Instruction::RevertOne {
                    key: unit_key,
                    height: height.to_owned(),
                };
                return Ok(instruction);
            }
            Command::RevertAll { height } => {
                let instruction = Instruction::RevertAll { height: height.to_owned() };
                return Ok(instruction);
            }
            Command::RemoveOne { key } => {
                let unit_key = UnitKey::from(key.as_bytes());
                let instruction = Instruction::RemoveOne { key: unit_key };
                return Ok(instruction);
            }
            Command::RemoveAll => {
                let instruction = Instruction::RemoveAll;
                return Ok(instruction);
            }
            Command::TransactionStart { transaction_id: _ } => {
                let instruction = Instruction::CreateTransaction;
                return Ok(instruction);
            }
            Command::TransactionalSet { key, value, transaction_id } => {
                let unit_key = UnitKey::from(key.as_bytes());
                let (content, _) = UnitContent::parse(value.as_bytes())?;
                let instruction = Instruction::TransactionalInsert {
                    key: unit_key,
                    content,
                    transaction_id: transaction_id.to_owned(),
                };
                return Ok(instruction);
            }
            Command::TransactionalRevertOne { key, height, transaction_id } => {
                let unit_key = UnitKey::from(key.as_bytes());
                let instruction = Instruction::TransactionalRevertOne {
                    key: unit_key,
                    height: height.to_owned(),
                    transaction_id: transaction_id.to_owned(),
                };
                return Ok(instruction);
            }
            Command::TransactionalRemoveOne { key, transaction_id } => {
                let unit_key = UnitKey::from(key.as_bytes());
                let instruction = Instruction::TransactionalRemoveOne {
                    key: unit_key,
                    transaction_id: transaction_id.to_owned(),
                };
                return Ok(instruction);
            }
            Command::TransactionCommit { transaction_id } => {
                let instruction = Instruction::TransactionCommit {
                    transaction_id: transaction_id.to_owned(),
                };
                return Ok(instruction);
            }
            Command::TransactionAbort { transaction_id } => {
                let instruction = Instruction::TransactionAbort {
                    transaction_id: transaction_id.to_owned(),
                };
                return Ok(instruction);
            }
        }
    }
}

impl ToString for Instruction {
    fn to_string(&self) -> String {
        match self {
            Instruction::Select { key, transaction_id } => format!("Instruction Select, key: {:?}, transaction_id: {:?}", key.to_string(), transaction_id),
            Instruction::InspectOne { key } => format!("Instruction InspectOne, key: {:?}", key.to_string()),
            Instruction::InspectAll => format!("Instruction InspectAll"),
            Instruction::Insert { key, content } => format!("Instruction Insert, key: {:?}, content: {:?}", key.to_string(), content),
            Instruction::RevertOne { key, height } => format!("Instruction RevertOne, key: {:?}, height: {:?}", key.to_string(), height),
            Instruction::RevertAll { height } => format!("Instruction RevertAll, height: {:?}", height),
            Instruction::RemoveOne { key } => format!("Instruction RemoveOne, key: {:?}", key.to_string()),
            Instruction::RemoveAll => format!("Instruction RemoveAll"),
            Instruction::CreateTransaction => format!("Instruction CreateTransaction"),
            Instruction::TransactionalInsert { key, content, transaction_id } => format!(
                "Instruction TransactionalInsert, key: {:?}, content: {:?}, transaction_id: {:?}",
                key.to_string(),
                content,
                transaction_id
            ),
            Instruction::TransactionalRevertOne { key, height, transaction_id } => format!(
                "Instruction TransactionalRevertOne, key: {:?}, height: {:?}, transaction_id: {:?}",
                key.to_string(),
                height,
                transaction_id
            ),
            Instruction::TransactionalRemoveOne { key, transaction_id } => format!(
                "Instruction TransactionalRemoveOne, key: {:?}, transaction_id: {:?}",
                key.to_string(),
                transaction_id
            ),
            Instruction::TransactionCommit { transaction_id } => format!("Instruction TransactionCommit, transaction_id {:?}", transaction_id),
            Instruction::TransactionAbort { transaction_id } => format!("Instruction TransactionAbort, transaction_id {:?}", transaction_id),
        }
    }
}
