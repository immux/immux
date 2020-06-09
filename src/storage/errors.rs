use crate::storage::command::CommandError;
use std::io::Error;
use std::num::ParseIntError;

#[derive(Debug)]
pub enum KVError {
    IOError(Error),
    CommandError(CommandError),
    RervertOutOfRange,
    ParseIntError(ParseIntError),
}

impl From<Error> for KVError {
    fn from(err: Error) -> KVError {
        KVError::IOError(err)
    }
}

impl From<CommandError> for KVError {
    fn from(err: CommandError) -> KVError {
        KVError::CommandError(err)
    }
}

impl From<ParseIntError> for KVError {
    fn from(err: ParseIntError) -> KVError {
        KVError::ParseIntError(err)
    }
}

pub type KVResult<T> = Result<T, KVError>;
