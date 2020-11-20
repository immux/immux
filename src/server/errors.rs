use std::io::Error;
use std::num::ParseIntError;
use std::sync::mpsc::{RecvError, SendError};

use crate::storage::executor::command::CommandError;
use crate::storage::executor::errors::ExecutorError;
use crate::storage::executor::predicate::PredicateError;

#[derive(Debug)]
pub enum ServerError {
    TinyHTTPError,
    ExecutorError(ExecutorError),
    BodyExtractionError(Error),
    UrlParsingError,
    BodyParsingError,
    ParseIntError(ParseIntError),
    HttpResponseError(Error),
    PredicateError(PredicateError),
    SenderError,
    ReceiverError(RecvError),
    TCPServerError(Error),
    CommandError(CommandError),
    ThreadError,
}

impl From<Error> for ServerError {
    fn from(err: Error) -> ServerError {
        ServerError::TCPServerError(err)
    }
}

impl From<RecvError> for ServerError {
    fn from(err: RecvError) -> ServerError {
        ServerError::ReceiverError(err)
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
    fn from(err: ParseIntError) -> ServerError {
        ServerError::ParseIntError(err)
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

pub type ServerResult<S> = Result<S, ServerError>;
