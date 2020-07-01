use crate::executor::unit_content::UnitContentError;
use crate::storage::errors::KVError;
use std::num::ParseIntError;

#[derive(Debug)]
pub enum ExecutorError {
    KVError(KVError),
    UnitContentError(UnitContentError),
    ParseIntError(ParseIntError),
}

impl From<KVError> for ExecutorError {
    fn from(err: KVError) -> ExecutorError {
        ExecutorError::KVError(err)
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

pub type ExecutorResult<T> = Result<T, ExecutorError>;
