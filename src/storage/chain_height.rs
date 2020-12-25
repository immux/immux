use std::fmt;

use crate::utils::varint::varint_encode;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ChainHeight(u64);

#[derive(Debug, PartialEq)]
pub enum ChainHeightError {
    NegativeChainHeight,
    ChainHeightOutOfRange,
    ParseChainHeightErrorError,
}

enum ChainHeightErrorPrefix {
    NegativeChainHeight = 0x01,
    ChainHeightOutOfRange = 0x02,
    ParseChainHeightErrorError = 0x03,
}

impl ChainHeightError {
    pub fn marshal(&self) -> Vec<u8> {
        match self {
            ChainHeightError::NegativeChainHeight => {
                vec![ChainHeightErrorPrefix::NegativeChainHeight as u8]
            }
            ChainHeightError::ChainHeightOutOfRange => {
                vec![ChainHeightErrorPrefix::ChainHeightOutOfRange as u8]
            }
            ChainHeightError::ParseChainHeightErrorError => {
                vec![ChainHeightErrorPrefix::ParseChainHeightErrorError as u8]
            }
        }
    }

    pub fn parse(data: &[u8]) -> Result<(ChainHeightError, usize), ChainHeightError> {
        let mut position = 0;
        let prefix = data[position];
        position += 1;

        if prefix == ChainHeightError::NegativeChainHeight as u8 {
            Ok((ChainHeightError::NegativeChainHeight, position))
        } else if prefix == ChainHeightError::ChainHeightOutOfRange as u8 {
            Ok((ChainHeightError::ChainHeightOutOfRange, position))
        } else {
            Ok((ChainHeightError::ParseChainHeightErrorError, position))
        }
    }
}

impl fmt::Display for ChainHeightError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ChainHeightError::NegativeChainHeight => {
                write!(f, "{}", "ChainHeightError::NegativeChainHeight")
            }
            ChainHeightError::ChainHeightOutOfRange => {
                write!(f, "{}", "ChainHeightError::ChainHeightOutOfRange")
            }
            ChainHeightError::ParseChainHeightErrorError => {
                write!(f, "{}", "ChainHeightError::ParseChainHeightErrorError")
            }
        }
    }
}

impl ChainHeight {
    pub fn new(data: u64) -> Self {
        Self(data)
    }
    pub fn decrement(&mut self) -> Result<Self, ChainHeightError> {
        if self.0 == 0 {
            return Err(ChainHeightError::NegativeChainHeight);
        }
        self.0 -= 1;
        return Ok(Self(self.0));
    }
    pub fn increment(&mut self) -> Result<Self, ChainHeightError> {
        if self.0 == u64::MAX {
            return Err(ChainHeightError::ChainHeightOutOfRange);
        }
        self.0 += 1;
        return Ok(Self(self.0));
    }
    pub fn as_u64(&self) -> u64 {
        self.0
    }
    pub fn serialize(&self) -> Vec<u8> {
        varint_encode(self.as_u64())
    }
}

impl fmt::Display for ChainHeight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ChainHeight : {}", self.0)
    }
}
