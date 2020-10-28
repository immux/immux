use std::fmt::Formatter;
use std::num::ParseIntError;

use crate::storage::errors::KVError;
use crate::storage::executor::command::CommandError;
use crate::storage::executor::predicate::PredicateError;
use crate::storage::executor::unit_content::UnitContentError;
use crate::storage::kvkey::KVKeyError;

#[derive(Debug)]
pub enum ExecutorError {
    KVError(KVError),
    UnitContentError(UnitContentError),
    ParseIntError(ParseIntError),
    CommandError(CommandError),
    PredicateError(PredicateError),
    KVKeyError(KVKeyError),
    UnexpectedOutcome,
}

impl From<KVError> for ExecutorError {
    fn from(err: KVError) -> ExecutorError {
        ExecutorError::KVError(err)
    }
}

impl From<KVKeyError> for ExecutorError {
    fn from(err: KVKeyError) -> ExecutorError {
        ExecutorError::KVKeyError(err)
    }
}

impl From<UnitContentError> for ExecutorError {
    fn from(err: UnitContentError) -> ExecutorError {
        ExecutorError::UnitContentError(err)
    }
}

impl From<ParseIntError> for ExecutorError {
    fn from(err: ParseIntError) -> ExecutorError {
        ExecutorError::ParseIntError(err)
    }
}

impl From<CommandError> for ExecutorError {
    fn from(err: CommandError) -> ExecutorError {
        ExecutorError::CommandError(err)
    }
}

impl From<PredicateError> for ExecutorError {
    fn from(err: PredicateError) -> ExecutorError {
        ExecutorError::PredicateError(err)
    }
}

impl std::error::Error for ExecutorError {
    fn description(&self) -> &str {
        return "Executor error";
    }
}

impl std::fmt::Display for ExecutorError {
    fn fmt(&self, _f: &mut Formatter) -> Result<(), std::fmt::Error> {
        return Ok(());
    }
}

pub type ExecutorResult<T> = Result<T, ExecutorError>;
