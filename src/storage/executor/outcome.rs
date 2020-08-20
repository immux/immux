use crate::storage::chain_height::ChainHeight;
use crate::storage::executor::command::Command;
use crate::storage::executor::unit_content::UnitContent;
use crate::storage::transaction_manager::TransactionId;

#[derive(Debug)]
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
