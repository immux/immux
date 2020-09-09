use std::io::Error;

use reqwest::StatusCode;
use std::fmt::Formatter;

#[derive(Debug)]
pub enum ImmuxDBHttpClientError {
    Everything,
    Reqwest(reqwest::Error),
    Unimplemented,
}

impl std::fmt::Display for ImmuxDBHttpClientError {
    fn fmt(&self, _f: &mut Formatter) -> Result<(), std::fmt::Error> {
        return Ok(());
    }
}

impl std::error::Error for ImmuxDBHttpClientError {
    fn description(&self) -> &str {
        return "ImmuxDB client error";
    }
}

impl From<reqwest::Error> for ImmuxDBHttpClientError {
    fn from(error: reqwest::Error) -> ImmuxDBHttpClientError {
        return ImmuxDBHttpClientError::Reqwest(error);
    }
}

pub type HttpClientResult = Result<(StatusCode, String), ImmuxDBHttpClientError>;

#[derive(Debug)]
pub enum ImmuxDBTcpClientError {
    StreamError(Error),
}

impl From<Error> for ImmuxDBTcpClientError {
    fn from(error: Error) -> ImmuxDBTcpClientError {
        return ImmuxDBTcpClientError::StreamError(error);
    }
}

pub type ImmuxDBTcpClientResult<T> = Result<T, ImmuxDBTcpClientError>;
