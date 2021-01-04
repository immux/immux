use std::io::Error;

use immuxsys::server::tcp_response::TcpResponseError;
use immuxsys::storage::executor::outcome::OutcomeError;
use reqwest::StatusCode;
use std::fmt::Formatter;

#[derive(Debug)]
pub enum ImmuxDBHttpClientError {
    Reqwest(reqwest::Error),
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
    OutcomeError(OutcomeError),
    TcpResponseError(TcpResponseError),
}

impl From<Error> for ImmuxDBTcpClientError {
    fn from(error: Error) -> ImmuxDBTcpClientError {
        return ImmuxDBTcpClientError::StreamError(error);
    }
}

impl From<OutcomeError> for ImmuxDBTcpClientError {
    fn from(error: OutcomeError) -> ImmuxDBTcpClientError {
        return ImmuxDBTcpClientError::OutcomeError(error);
    }
}

impl From<TcpResponseError> for ImmuxDBTcpClientError {
    fn from(error: TcpResponseError) -> ImmuxDBTcpClientError {
        return ImmuxDBTcpClientError::TcpResponseError(error);
    }
}

pub type ImmuxDBTcpClientResult<T> = Result<T, ImmuxDBTcpClientError>;
