use std::cmp::Ordering;
use std::fmt::Formatter;
use std::num::ParseIntError;

#[derive(Debug, Clone, Copy)]
pub struct LogVersion {
    major: u8,
    minor: u8,
    revise: u8,
}

impl LogVersion {
    pub fn new(major: u8, minor: u8, revise: u8) -> Self {
        LogVersion {
            major,
            minor,
            revise,
        }
    }

    pub fn marshal(&self) -> Vec<u8> {
        vec![self.major, self.minor, self.revise]
    }

    pub fn parse(data: &[u8]) -> Result<(Self, usize), LogVersionError> {
        if data.len() < 3 {
            Err(LogVersionError::LogVersionParsingError)
        } else {
            Ok((LogVersion::new(data[0], data[1], data[2]), 3))
        }
    }

    pub fn try_from(items: &[&str]) -> Result<Self, LogVersionError> {
        if items.len() < 3 {
            Err(LogVersionError::InvalidString)
        } else {
            let major = items[0].parse::<u8>()?;
            let minor = items[1].parse::<u8>()?;
            let revise = items[2].parse::<u8>()?;

            Ok(LogVersion {
                major,
                minor,
                revise,
            })
        }
    }
}

impl From<&[u8; 3]> for LogVersion {
    fn from(bytes: &[u8; 3]) -> LogVersion {
        LogVersion {
            major: bytes[0],
            minor: bytes[1],
            revise: bytes[2],
        }
    }
}

impl PartialOrd for LogVersion {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.major == other.major && self.minor == other.minor && self.revise == other.revise {
            return Some(Ordering::Equal);
        } else if (self.major > other.major)
            || (self.major == other.major && self.minor > other.minor)
            || (self.major == other.major
                && self.minor == other.minor
                && self.revise > other.revise)
        {
            return Some(Ordering::Greater);
        } else {
            return Some(Ordering::Less);
        }
    }
}

impl PartialEq for LogVersion {
    fn eq(&self, other: &Self) -> bool {
        self.major == other.major && self.minor == other.minor && self.revise == other.revise
    }
}

impl std::fmt::Display for LogVersion {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.revise)
    }
}

#[derive(Debug, PartialEq)]
pub enum LogVersionError {
    LogVersionParsingError,
    InvalidString,
    ParseIntError(ParseIntError),
    UnexpectedLogVersion,
}

impl From<ParseIntError> for LogVersionError {
    fn from(error: ParseIntError) -> LogVersionError {
        LogVersionError::ParseIntError(error)
    }
}
