use crate::storage::chain_height::ChainHeight;
use crate::storage::executor::instruction::Instruction;
use crate::storage::executor::unit_content::UnitContent;
use crate::storage::transaction_manager::TransactionId;

#[derive(Debug)]
pub enum Outcome {
    Select(Option<UnitContent>),
    InspectOne(Vec<(Instruction, ChainHeight)>),
    InspectAll(Vec<(Instruction, ChainHeight)>),
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
