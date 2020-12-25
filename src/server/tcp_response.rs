use crate::server::errors::ServerError;
use crate::storage::executor::outcome::{Outcome, OutcomeError};

pub enum TcpResponse {
    Outcome(Outcome),
    ServerError(ServerError),
}

enum TcpResponsePrefix {
    Outcome = 0x00,
    ServerError = 0x01,
}

#[derive(Debug)]
pub enum TcpResponseError {
    OutcomeError(OutcomeError),
    ServerError(ServerError),
}

impl From<OutcomeError> for TcpResponseError {
    fn from(error: OutcomeError) -> TcpResponseError {
        TcpResponseError::OutcomeError(error)
    }
}

impl From<ServerError> for TcpResponseError {
    fn from(error: ServerError) -> TcpResponseError {
        TcpResponseError::ServerError(error)
    }
}

impl TcpResponse {
    pub fn marshal(&self) -> Vec<u8> {
        match self {
            TcpResponse::Outcome(outcome) => {
                let mut result = vec![TcpResponsePrefix::Outcome as u8];
                let outcome_bytes = outcome.marshal();
                result.extend_from_slice(&outcome_bytes);
                result
            }
            TcpResponse::ServerError(error) => {
                let mut result = vec![TcpResponsePrefix::ServerError as u8];
                let error_bytes = error.marshal();
                result.extend_from_slice(&error_bytes);
                result
            }
        }
    }

    pub fn parse(data: &[u8]) -> Result<(TcpResponse, usize), TcpResponseError> {
        let mut position = 0;
        let prefix = data[position];
        position += 1;

        if prefix == TcpResponsePrefix::Outcome as u8 {
            let (outcome, offset) = Outcome::parse(&data[position..])?;
            position += offset;
            Ok((TcpResponse::Outcome(outcome), position))
        } else {
            let (error, offset) = ServerError::parse(&data[position..])?;
            position += offset;
            Ok((TcpResponse::ServerError(error), position))
        }
    }
}
