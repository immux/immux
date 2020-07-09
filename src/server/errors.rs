use std::boxed::Box;
use std::io::Error;
use std::num::ParseIntError;

use crate::storage::executor::errors::ExecutorError;

pub type ServerGeneralError = Box<dyn std::error::Error + std::marker::Send + std::marker::Sync>;

#[derive(Debug)]
pub enum ServerError {
    GeneralError(ServerGeneralError),
    ExecutorError(ExecutorError),
    BodyExtractionError(Error),
    UrlParsingError,
    BodyParsingError,
    ParseIntError(ParseIntError),
    HttpResponseError(Error),

    //    unimplemented error
    UnimplementedForGetCollection,
}

impl From<ServerGeneralError> for ServerError {
    fn from(err: ServerGeneralError) -> ServerError {
        ServerError::GeneralError(err)
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

pub type ServerResult<T> = Result<T, ServerError>;
