#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ChainHeight(u64);

#[derive(Debug)]
pub enum ChainHeightError {
    NegativeChainHeight,
    ChainHeightOutOfRange,
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
}
