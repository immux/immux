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
pub enum ImmuxDBTCPClientError {
    StreamError(Error),
}

impl From<Error> for ImmuxDBTCPClientError {
    fn from(error: Error) -> ImmuxDBTCPClientError {
        return ImmuxDBTCPClientError::StreamError(error);
    }
}

pub type TCPClientResult<T> = Result<T, ImmuxDBTCPClientError>;
