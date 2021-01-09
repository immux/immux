use std::fmt::Formatter;
use std::num::ParseIntError;

use crate::storage::errors::KVError;
use crate::storage::executor::command::CommandError;
use crate::storage::executor::predicate::PredicateError;
use crate::storage::executor::unit_content::UnitContentError;
use crate::storage::kvkey::KVKeyError;
use crate::system_error::SystemError;
use crate::utils::varint::VarIntError;
use std::string::FromUtf8Error;

#[derive(Debug, Clone, PartialEq)]
pub enum ExecutorError {
    KVError(KVError),
    UnitContentError(UnitContentError),
    ParseIntError(SystemError),
    CommandError(CommandError),
    PredicateError(PredicateError),
    KVKeyError(KVKeyError),
    UnexpectedOutcome,
    ParseExecutorErrorToStringError,
    SystemError(SystemError),
}

#[derive(Debug)]
pub enum ExecutorErrorPrefix {
    KVError = 0x01,
    UnitContentError = 0x02,
    ParseIntError = 0x03,
    CommandError = 0x04,
    PredicateError = 0x05,
    KVKeyError = 0x06,
    UnexpectedOutcome = 0x07,
    ParseExecutorErrorToStringError = 0x08,
    SystemError = 0x09,
}

impl ExecutorError {
    pub fn marshal(&self) -> Vec<u8> {
        match self {
            ExecutorError::KVError(error) => {
                let mut result = vec![ExecutorErrorPrefix::KVError as u8];
                let error_bytes = error.marshal();
                result.extend_from_slice(&error_bytes);
                return result;
            }
            ExecutorError::UnitContentError(error) => {
                let mut result = vec![ExecutorErrorPrefix::UnitContentError as u8];
                let error_bytes = error.marshal();
                result.extend_from_slice(&error_bytes);
                return result;
            }
            ExecutorError::ParseIntError(error) => {
                let mut result = vec![ExecutorErrorPrefix::ParseIntError as u8];
                let error_bytes = error.marshal();
                result.extend_from_slice(&error_bytes);
                return result;
            }
            ExecutorError::CommandError(error) => {
                let mut result = vec![ExecutorErrorPrefix::CommandError as u8];
                let error_bytes = error.marshal();
                result.extend_from_slice(&error_bytes);
                return result;
            }
            ExecutorError::PredicateError(error) => {
                let mut result = vec![ExecutorErrorPrefix::PredicateError as u8];
                let error_bytes = error.marshal();
                result.extend_from_slice(&error_bytes);
                return result;
            }
            ExecutorError::KVKeyError(error) => {
                let mut result = vec![ExecutorErrorPrefix::KVKeyError as u8];
                let error_bytes = error.marshal();
                result.extend_from_slice(&error_bytes);
                return result;
            }
            ExecutorError::UnexpectedOutcome => vec![ExecutorErrorPrefix::UnexpectedOutcome as u8],
            ExecutorError::SystemError(error) => {
                let mut result = vec![ExecutorErrorPrefix::SystemError as u8];
                let error_bytes = error.marshal();
                result.extend_from_slice(&error_bytes);
                return result;
            }
            ExecutorError::ParseExecutorErrorToStringError => {
                vec![ExecutorErrorPrefix::ParseExecutorErrorToStringError as u8]
            }
        }
    }

    pub fn parse(data: &[u8]) -> Result<(ExecutorError, usize), ExecutorError> {
        let mut position = 0;
        let prefix = data[position];
        position += 1;

        if prefix == ExecutorErrorPrefix::KVError as u8 {
            let (error, offset) = KVError::parse(&data[position..])?;
            position += offset;
            Ok((ExecutorError::KVError(error), position))
        } else if prefix == ExecutorErrorPrefix::UnitContentError as u8 {
            let (error, offset) = UnitContentError::parse(&data[position..])?;
            position += offset;
            Ok((ExecutorError::UnitContentError(error), position))
        } else if prefix == ExecutorErrorPrefix::ParseIntError as u8 {
            let (error, offset) = SystemError::parse(&data[position..])?;
            position += offset;
            Ok((ExecutorError::ParseIntError(error), position))
        } else if prefix == ExecutorErrorPrefix::CommandError as u8 {
            let (error, offset) = CommandError::parse(&data[position..])?;
            position += offset;
            Ok((ExecutorError::CommandError(error), position))
        } else if prefix == ExecutorErrorPrefix::PredicateError as u8 {
            let (error, offset) = PredicateError::parse(&data[position..])?;
            position += offset;
            Ok((ExecutorError::PredicateError(error), position))
        } else if prefix == ExecutorErrorPrefix::KVKeyError as u8 {
            let (error, offset) = KVKeyError::parse(&data[position..])?;
            position += offset;
            Ok((ExecutorError::KVKeyError(error), position))
        } else if prefix == ExecutorErrorPrefix::UnexpectedOutcome as u8 {
            Ok((ExecutorError::UnexpectedOutcome, position))
        } else if prefix == ExecutorErrorPrefix::SystemError as u8 {
            let (error, offset) = SystemError::parse(&data[position..])?;
            position += offset;
            Ok((ExecutorError::SystemError(error), position))
        } else {
            return Ok((ExecutorError::ParseExecutorErrorToStringError, position));
        }
    }
}

impl From<KVError> for ExecutorError {
    fn from(err: KVError) -> ExecutorError {
        ExecutorError::KVError(err)
    }
}

impl From<SystemError> for ExecutorError {
    fn from(err: SystemError) -> ExecutorError {
        ExecutorError::SystemError(err)
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
    fn from(_err: ParseIntError) -> ExecutorError {
        ExecutorError::ParseIntError(SystemError::ParseIntError)
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

impl From<FromUtf8Error> for ExecutorError {
    fn from(_err: FromUtf8Error) -> ExecutorError {
        ExecutorError::ParseExecutorErrorToStringError
    }
}

impl From<VarIntError> for ExecutorError {
    fn from(_err: VarIntError) -> ExecutorError {
        ExecutorError::ParseExecutorErrorToStringError
    }
}

impl std::error::Error for ExecutorError {
    fn description(&self) -> &str {
        return "Executor error";
    }
}

impl std::fmt::Display for ExecutorError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        match self {
            ExecutorError::KVError(error) => {
                write!(f, "{}::{}", "ExecutorError::KVError", error)
            }
            ExecutorError::UnitContentError(error) => {
                write!(f, "{}::{}", "ExecutorError::UnitContentError", error)
            }
            ExecutorError::ParseIntError(error) => {
                write!(f, "{}::{}", "ExecutorError::ParseIntError", error)
            }
            ExecutorError::CommandError(error) => {
                write!(f, "{}::{}", "ExecutorError::CommandError", error)
            }
            ExecutorError::PredicateError(error) => {
                write!(f, "{}::{}", "ExecutorError::PredicateError", error)
            }
            ExecutorError::KVKeyError(error) => {
                write!(f, "{}::{}", "ExecutorError::PredicateError", error)
            }
            ExecutorError::UnexpectedOutcome => {
                write!(f, "{}", "ExecutorError::UnexpectedOutcome")
            }
            ExecutorError::ParseExecutorErrorToStringError => {
                write!(f, "{}", "ExecutorError::ParseExecutorErrorToStringError")
            }
            ExecutorError::SystemError(error) => {
                write!(f, "{}::{}", "ExecutorError::SystemError", error,)
            }
        }
    }
}

pub type ExecutorResult<T> = Result<T, ExecutorError>;

#[cfg(test)]
mod executor_error_tests {
    use immuxsys_dev_utils::dev_utils::{get_executor_errors, ExecutorError};

    #[test]
    fn executor_error_reversibility() {
        let errors = get_executor_errors();

        for expected_error in errors {
            let error_bytes = expected_error.marshal();
            let (actual_error, _) = ExecutorError::parse(&error_bytes).unwrap();

            assert_eq!(expected_error, actual_error);
        }
    }
}
