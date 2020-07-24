use reqwest::StatusCode;
use std::fmt::Formatter;

#[derive(Debug)]
pub enum ImmuxDBClientError {
    Everything,
    Reqwest(reqwest::Error),
    Unimplemented,
}

impl std::fmt::Display for ImmuxDBClientError {
    fn fmt(&self, _f: &mut Formatter) -> Result<(), std::fmt::Error> {
        return Ok(());
    }
}

impl std::error::Error for ImmuxDBClientError {
    fn description(&self) -> &str {
        return "ImmuxDB client error";
    }
}

impl From<reqwest::Error> for ImmuxDBClientError {
    fn from(error: reqwest::Error) -> ImmuxDBClientError {
        return ImmuxDBClientError::Reqwest(error);
    }
}

pub type ClientResult = Result<(StatusCode, String), ImmuxDBClientError>;
