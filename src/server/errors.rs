use std::fmt;
use std::io::Error;
use std::num::ParseIntError;
use std::string::FromUtf8Error;
use std::sync::mpsc::{RecvError, SendError};

use crate::storage::executor::command::CommandError;
use crate::storage::executor::errors::ExecutorError;
use crate::storage::executor::predicate::PredicateError;
use crate::system_error::SystemError;
use crate::utils::varint::VarIntError;

#[derive(Debug, PartialEq)]
pub enum ServerError {
    TinyHTTPError,
    ExecutorError(ExecutorError),
    BodyExtractionError(SystemError),
    UrlParsingError,
    BodyParsingError,
    ParseIntError(SystemError),
    HttpResponseError(SystemError),
    PredicateError(PredicateError),
    SenderError,
    ReceiverError(SystemError),
    TCPServerError(SystemError),
    CommandError(CommandError),
    ThreadError,
    SystemError(SystemError),
    ParseServerErrorError,
}

#[derive(Debug)]
pub enum ServerErrorPrefix {
    TinyHTTPError = 0x01,
    ExecutorError = 0x02,
    BodyExtractionError = 0x03,
    UrlParsingError = 0x04,
    BodyParsingError = 0x05,
    ParseIntError = 0x06,
    HttpResponseError = 0x07,
    PredicateError = 0x08,
    SenderError = 0x09,
    ReceiverError = 0x0A,
    TCPServerError = 0x0B,
    CommandError = 0x0C,
    ThreadError = 0x0D,
    ParseServerErrorError = 0x0E,
    SystemError = 0x0F,
}

impl ServerError {
    pub fn marshal(&self) -> Vec<u8> {
        match self {
            ServerError::TinyHTTPError => vec![ServerErrorPrefix::TinyHTTPError as u8],
            ServerError::ExecutorError(executor_error) => {
                let mut result = vec![ServerErrorPrefix::ExecutorError as u8];
                let error_bytes = executor_error.marshal();
                result.extend_from_slice(&error_bytes);
                return result;
            }
            ServerError::BodyExtractionError(error) => {
                let mut result = vec![ServerErrorPrefix::BodyExtractionError as u8];
                let error_bytes = error.marshal();
                result.push(error_bytes);
                return result;
            }
            ServerError::UrlParsingError => vec![ServerErrorPrefix::UrlParsingError as u8],
            ServerError::BodyParsingError => vec![ServerErrorPrefix::BodyParsingError as u8],
            ServerError::ParseIntError(error) => {
                let mut result = vec![ServerErrorPrefix::ParseIntError as u8];
                let error_bytes = error.marshal();
                result.push(error_bytes);
                return result;
            }
            ServerError::HttpResponseError(error) => {
                let mut result = vec![ServerErrorPrefix::HttpResponseError as u8];
                let error_bytes = error.marshal();
                result.push(error_bytes);
                return result;
            }
            ServerError::PredicateError(predicate_error) => {
                let mut result = vec![ServerErrorPrefix::PredicateError as u8];
                let error_bytes = predicate_error.marshal();
                result.extend_from_slice(&error_bytes);
                return result;
            }
            ServerError::SenderError => vec![ServerErrorPrefix::SenderError as u8],
            ServerError::ReceiverError(_error) => {
                let mut result = vec![ServerErrorPrefix::ReceiverError as u8];
                let error_byte = SystemError::ReceiverError.marshal();
                result.push(error_byte);
                return result;
            }
            ServerError::TCPServerError(error) => {
                let mut result = vec![ServerErrorPrefix::TCPServerError as u8];
                let error_bytes = error.marshal();
                result.push(error_bytes);
                return result;
            }
            ServerError::CommandError(error) => {
                let mut result = vec![ServerErrorPrefix::CommandError as u8];
                let error_bytes = error.marshal();
                result.extend_from_slice(&error_bytes);
                return result;
            }
            ServerError::ThreadError => vec![ServerErrorPrefix::ThreadError as u8],
            ServerError::ParseServerErrorError => {
                vec![ServerErrorPrefix::ParseServerErrorError as u8]
            }
            ServerError::SystemError(error) => {
                let mut result = vec![ServerErrorPrefix::SystemError as u8];
                let error_byte = error.marshal();
                result.push(error_byte);
                return result;
            }
        }
    }

    pub fn parse(data: &[u8]) -> Result<(ServerError, usize), ServerError> {
        let mut position = 0;
        let prefix = data[position];
        position += 1;

        if prefix == ServerErrorPrefix::TinyHTTPError as u8 {
            return Ok((ServerError::TinyHTTPError, position));
        } else if prefix == ServerErrorPrefix::ExecutorError as u8 {
            let (error, offset) = ExecutorError::parse(&data[position..])?;
            position += offset;
            return Ok((ServerError::ExecutorError(error), position));
        } else if prefix == ServerErrorPrefix::BodyExtractionError as u8 {
            let (error, offset) = SystemError::parse(&data[position..])?;
            position += offset;
            return Ok((ServerError::BodyExtractionError(error), position));
        } else if prefix == ServerErrorPrefix::UrlParsingError as u8 {
            return Ok((ServerError::UrlParsingError, position));
        } else if prefix == ServerErrorPrefix::BodyParsingError as u8 {
            return Ok((ServerError::BodyParsingError, position));
        } else if prefix == ServerErrorPrefix::ParseIntError as u8 {
            let (error, offset) = SystemError::parse(&data[position..])?;
            position += offset;
            return Ok((ServerError::ParseIntError(error), position));
        } else if prefix == ServerErrorPrefix::HttpResponseError as u8 {
            let (error, offset) = SystemError::parse(&data[position..])?;
            position += offset;
            return Ok((ServerError::HttpResponseError(error), position));
        } else if prefix == ServerErrorPrefix::PredicateError as u8 {
            let (error, offset) = PredicateError::parse(&data[position..])?;
            position += offset;
            return Ok((ServerError::PredicateError(error), position));
        } else if prefix == ServerErrorPrefix::SenderError as u8 {
            return Ok((ServerError::SenderError, position));
        } else if prefix == ServerErrorPrefix::ReceiverError as u8 {
            let (error, offset) = SystemError::parse(&data[position..])?;
            position += offset;
            return Ok((ServerError::ReceiverError(error), position));
        } else if prefix == ServerErrorPrefix::TCPServerError as u8 {
            let (error, offset) = SystemError::parse(&data[position..])?;
            position += offset;
            return Ok((ServerError::TCPServerError(error), position));
        } else if prefix == ServerErrorPrefix::CommandError as u8 {
            let (error, offset) = CommandError::parse(&data[position..])?;
            position += offset;
            return Ok((ServerError::CommandError(error), position));
        } else if prefix == ServerErrorPrefix::ThreadError as u8 {
            return Ok((ServerError::ThreadError, position));
        } else if prefix == ServerErrorPrefix::SystemError as u8 {
            let (error, offset) = SystemError::parse(&data[position..])?;
            position += offset;
            return Ok((ServerError::SystemError(error), position));
        } else {
            return Ok((ServerError::ParseServerErrorError, position));
        }
    }
}

impl From<Error> for ServerError {
    fn from(_err: Error) -> ServerError {
        ServerError::TCPServerError(SystemError::IOError)
    }
}

impl From<SystemError> for ServerError {
    fn from(err: SystemError) -> ServerError {
        ServerError::SystemError(err)
    }
}

impl From<RecvError> for ServerError {
    fn from(_err: RecvError) -> ServerError {
        ServerError::ReceiverError(SystemError::ReceiverError)
    }
}

impl<T> From<SendError<T>> for ServerError {
    fn from(_err: SendError<T>) -> ServerError {
        ServerError::SenderError
    }
}

impl From<ExecutorError> for ServerError {
    fn from(err: ExecutorError) -> ServerError {
        ServerError::ExecutorError(err)
    }
}

impl From<ParseIntError> for ServerError {
    fn from(_err: ParseIntError) -> ServerError {
        ServerError::ParseIntError(SystemError::ParseIntError)
    }
}

impl From<CommandError> for ServerError {
    fn from(err: CommandError) -> ServerError {
        ServerError::CommandError(err)
    }
}

impl From<PredicateError> for ServerError {
    fn from(err: PredicateError) -> ServerError {
        ServerError::PredicateError(err)
    }
}

impl From<FromUtf8Error> for ServerError {
    fn from(_err: FromUtf8Error) -> ServerError {
        ServerError::ParseServerErrorError
    }
}

impl From<VarIntError> for ServerError {
    fn from(_err: VarIntError) -> ServerError {
        ServerError::ParseServerErrorError
    }
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServerError::TinyHTTPError => write!(f, "{}", "ServerError::TinyHTTPError"),
            ServerError::ExecutorError(executor_error) => {
                let executor_error_str = format!("{}", executor_error);
                write!(
                    f,
                    "{}::{}",
                    "ServerError::ExecutorError", executor_error_str
                )
            }
            ServerError::BodyExtractionError(error) => {
                let error_str = format!("{}", error);
                write!(f, "{}::{}", "ServerError::ExecutorError", error_str)
            }
            ServerError::UrlParsingError => {
                write!(f, "{}", "ServerError::UrlParsingError")
            }
            ServerError::BodyParsingError => {
                write!(f, "{}", "ServerError::BodyParsingError")
            }
            ServerError::ParseIntError(error) => {
                let error_str = format!("{}", error);
                write!(f, "{}::{}", "ServerError::ExecutorError", error_str)
            }
            ServerError::HttpResponseError(error) => {
                let error_str = format!("{}", error);
                write!(f, "{}::{}", "ServerError::ExecutorError", error_str)
            }
            ServerError::PredicateError(predicate_error) => {
                let predicate_error_str = format!("{}", predicate_error);
                write!(
                    f,
                    "{}::{}",
                    "ServerError::ExecutorError", predicate_error_str
                )
            }
            ServerError::SenderError => write!(f, "{}", "ServerError::SenderError"),
            ServerError::ReceiverError(error) => {
                let error_str = format!("{}", error);
                write!(f, "{}::{}", "ServerError::ExecutorError", error_str)
            }
            ServerError::TCPServerError(error) => {
                write!(f, "{}::{}", "ServerError::ExecutorError", error)
            }
            ServerError::CommandError(error) => {
                write!(f, "{}::{}", "ServerError::ExecutorError", error)
            }
            ServerError::ThreadError => write!(f, "{}", "ServerError::ThreadError"),
            ServerError::ParseServerErrorError => {
                write!(f, "{}", "ServerError::ParseServerErrorToStringError")
            }
            ServerError::SystemError(error) => {
                write!(f, "{}::{}", "ServerError::SystemError", error)
            }
        }
    }
}

pub type ServerResult<S> = Result<S, ServerError>;
