use std::fmt;
use std::io::Error;
use std::num::ParseIntError;

use crate::storage::chain_height::ChainHeightError;
use crate::storage::instruction::InstructionError;
use crate::storage::log_version::LogVersionError;
use crate::storage::transaction_manager::TransactionManagerError;
use crate::system_error::SystemError;

#[derive(Debug, Clone, PartialEq)]
pub enum KVError {
    IOError(SystemError),
    InstructionError(InstructionError),
    RevertOutOfRange,
    ParseIntError(SystemError),
    ChainHeightError(ChainHeightError),
    PointToUnexpectedInstruction,
    TransactionManagerError(TransactionManagerError),
    LogVersionError(LogVersionError),
    ParseKVErrorError,
    SystemError(SystemError),
}

enum KVErrorPrefix {
    IOError = 0x01,
    InstructionError = 0x02,
    RevertOutOfRange = 0x03,
    ParseIntError = 0x04,
    ChainHeightError = 0x05,
    PointToUnexpectedInstruction = 0x06,
    TransactionManagerError = 0x07,
    LogVersionError = 0x08,
    ParseKVErrorError = 0x09,
    SystemError = 0x10,
}

impl KVError {
    pub fn marshal(&self) -> Vec<u8> {
        match self {
            KVError::IOError(error) => {
                let mut result = vec![KVErrorPrefix::IOError as u8];
                let error_bytes = error.marshal();
                result.extend_from_slice(&error_bytes);
                result
            }
            KVError::InstructionError(error) => {
                let mut result = vec![KVErrorPrefix::InstructionError as u8];
                let error_bytes = error.marshal();
                result.extend_from_slice(&error_bytes);
                result
            }
            KVError::RevertOutOfRange => vec![KVErrorPrefix::RevertOutOfRange as u8],
            KVError::ParseIntError(error) => {
                let mut result = vec![KVErrorPrefix::ParseIntError as u8];
                let error_bytes = error.marshal();
                result.extend_from_slice(&error_bytes);
                result
            }
            KVError::ChainHeightError(error) => {
                let mut result = vec![KVErrorPrefix::ChainHeightError as u8];
                let error_bytes = error.marshal();
                result.extend_from_slice(&error_bytes);
                result
            }
            KVError::PointToUnexpectedInstruction => {
                vec![KVErrorPrefix::PointToUnexpectedInstruction as u8]
            }
            KVError::TransactionManagerError(error) => {
                let mut result = vec![KVErrorPrefix::TransactionManagerError as u8];
                let error_bytes = error.marshal();
                result.extend_from_slice(&error_bytes);
                result
            }
            KVError::LogVersionError(error) => {
                let mut result = vec![KVErrorPrefix::LogVersionError as u8];
                let error_bytes = error.marshal();
                result.extend_from_slice(&error_bytes);
                result
            }
            KVError::SystemError(error) => {
                let mut result = vec![KVErrorPrefix::SystemError as u8];
                let error_byte = error.marshal();
                result.extend_from_slice(&error_byte);
                result
            }
            KVError::ParseKVErrorError => vec![KVErrorPrefix::ParseKVErrorError as u8],
        }
    }

    pub fn parse(data: &[u8]) -> Result<(KVError, usize), KVError> {
        let mut position = 0;
        let prefix = data[position];
        position += 1;

        if prefix == KVErrorPrefix::IOError as u8 {
            let (error, offset) = SystemError::parse(&data[position..])?;
            position += offset;
            Ok((KVError::IOError(error), position))
        } else if prefix == KVErrorPrefix::InstructionError as u8 {
            let (error, offset) = InstructionError::parse(&data[position..])?;
            position += offset;
            Ok((KVError::InstructionError(error), position))
        } else if prefix == KVErrorPrefix::RevertOutOfRange as u8 {
            Ok((KVError::RevertOutOfRange, position))
        } else if prefix == KVErrorPrefix::ParseIntError as u8 {
            let (error, offset) = SystemError::parse(&data[position..])?;
            position += offset;
            Ok((KVError::ParseIntError(error), position))
        } else if prefix == KVErrorPrefix::ChainHeightError as u8 {
            let (error, offset) = ChainHeightError::parse(&data[position..])?;
            position += offset;
            Ok((KVError::ChainHeightError(error), position))
        } else if prefix == KVErrorPrefix::PointToUnexpectedInstruction as u8 {
            Ok((KVError::PointToUnexpectedInstruction, position))
        } else if prefix == KVErrorPrefix::TransactionManagerError as u8 {
            let (error, offset) = TransactionManagerError::parse(&data[position..])?;
            position += offset;
            Ok((KVError::TransactionManagerError(error), position))
        } else if prefix == KVErrorPrefix::LogVersionError as u8 {
            let (error, offset) = LogVersionError::parse(&data[position..])?;
            position += offset;
            Ok((KVError::LogVersionError(error), position))
        } else if prefix == KVErrorPrefix::SystemError as u8 {
            let (error, offset) = SystemError::parse(&data[position..])?;
            position += offset;
            Ok((KVError::SystemError(error), position))
        } else {
            Ok((KVError::ParseKVErrorError, position))
        }
    }
}

impl fmt::Display for KVError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KVError::IOError(error) => {
                let error_string = format!("{}", error);
                write!(f, "{}::{}", "VarIntError::UnexpectedFormat", error_string)
            }
            KVError::InstructionError(error) => {
                let error_string = format!("{}", error);
                write!(f, "{}::{}", "KVError::InstructionError", error_string)
            }
            KVError::RevertOutOfRange => write!(f, "{}", "KVError::RevertOutOfRange"),
            KVError::ParseIntError(error) => {
                let error_string = format!("{}", error);
                write!(f, "{}::{}", "KVError::ParseIntError", error_string)
            }
            KVError::ChainHeightError(error) => {
                let error_string = format!("{}", error);
                write!(f, "{}::{}", "KVError::ChainHeightError", error_string)
            }
            KVError::PointToUnexpectedInstruction => {
                write!(f, "{}", "KVError::PointToUnexpectedInstruction")
            }
            KVError::TransactionManagerError(error) => {
                let error_string = format!("{}", error);
                write!(
                    f,
                    "{}::{}",
                    "KVError::TransactionManagerError", error_string
                )
            }
            KVError::LogVersionError(error) => {
                let error_string = format!("{}", error);
                write!(f, "{}::{}", "KVError::LogVersionError", error_string)
            }
            KVError::ParseKVErrorError => {
                write!(f, "{}", "KVError::ParseKVErrorError")
            }
            KVError::SystemError(error) => write!(f, "{}::{}", "KVError::SystemError", error),
        }
    }
}

impl From<Error> for KVError {
    fn from(_err: Error) -> KVError {
        KVError::IOError(SystemError::IOError)
    }
}

impl From<SystemError> for KVError {
    fn from(error: SystemError) -> KVError {
        return KVError::SystemError(error);
    }
}

impl From<InstructionError> for KVError {
    fn from(err: InstructionError) -> KVError {
        KVError::InstructionError(err)
    }
}

impl From<ParseIntError> for KVError {
    fn from(_err: ParseIntError) -> KVError {
        KVError::ParseIntError(SystemError::ParseIntError)
    }
}

impl From<ChainHeightError> for KVError {
    fn from(err: ChainHeightError) -> KVError {
        KVError::ChainHeightError(err)
    }
}

impl From<TransactionManagerError> for KVError {
    fn from(err: TransactionManagerError) -> KVError {
        KVError::TransactionManagerError(err)
    }
}

impl From<LogVersionError> for KVError {
    fn from(err: LogVersionError) -> KVError {
        KVError::LogVersionError(err)
    }
}

pub type KVResult<T> = Result<T, KVError>;

#[cfg(test)]
mod kv_error_tests {
    use immuxsys_dev_utils::dev_utils::{get_kv_errors, KVError};

    #[test]
    fn kv_error_reversibility() {
        let errors = get_kv_errors();

        for expected_error in errors {
            let error_bytes = expected_error.marshal();
            let (actual_error, _) = KVError::parse(&error_bytes).unwrap();

            assert_eq!(expected_error, actual_error);
        }
    }
}
