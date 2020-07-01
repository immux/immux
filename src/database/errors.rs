use crate::database::unit_content::UnitContentError;
use crate::storage::errors::KVError;
use std::num::ParseIntError;

#[derive(Debug)]
pub enum DatabaseError {
    KVError(KVError),
    UnitContentError(UnitContentError),
    ParseIntError(ParseIntError),
}

impl From<KVError> for DatabaseError {
    fn from(err: KVError) -> DatabaseError {
        DatabaseError::KVError(err)
    }
}

impl From<UnitContentError> for DatabaseError {
    fn from(err: UnitContentError) -> DatabaseError {
        DatabaseError::UnitContentError(err)
    }
}

impl From<ParseIntError> for DatabaseError {
    fn from(err: ParseIntError) -> DatabaseError {
        DatabaseError::ParseIntError(err)
    }
}

pub type DatabaseResult<T> = Result<T, DatabaseError>;
