use crate::system_error::SystemError;
use std::cmp::Ordering;
use std::fmt;
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
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.revise)
    }
}

#[derive(Debug, PartialEq)]
pub enum LogVersionError {
    LogVersionParsingError,
    InvalidString,
    ParseIntError(SystemError),
    UnexpectedLogVersion,
    ParseLogVersionErrorError,
}

enum LogVersionErrorPrefix {
    LogVersionParsingError = 0x01,
    InvalidString = 0x02,
    ParseIntError = 0x03,
    UnexpectedLogVersion = 0x04,
    ParseLogVersionErrorError = 0x05,
}

impl LogVersionError {
    pub fn marshal(&self) -> Vec<u8> {
        match self {
            LogVersionError::LogVersionParsingError => {
                vec![LogVersionErrorPrefix::LogVersionParsingError as u8]
            }
            LogVersionError::InvalidString => vec![LogVersionErrorPrefix::InvalidString as u8],
            LogVersionError::ParseIntError(error) => {
                let mut result = vec![LogVersionErrorPrefix::ParseIntError as u8];
                let error_byte = error.marshal();
                result.push(error_byte);
                result
            }
            LogVersionError::UnexpectedLogVersion => {
                vec![LogVersionErrorPrefix::UnexpectedLogVersion as u8]
            }
            LogVersionError::ParseLogVersionErrorError => {
                vec![LogVersionErrorPrefix::ParseLogVersionErrorError as u8]
            }
        }
    }

    pub fn parse(data: &[u8]) -> Result<(LogVersionError, usize), LogVersionError> {
        let mut position = 0;
        let prefix = data[position];
        position += 1;

        if prefix == LogVersionErrorPrefix::LogVersionParsingError as u8 {
            Ok((LogVersionError::LogVersionParsingError, position))
        } else if prefix == LogVersionErrorPrefix::InvalidString as u8 {
            Ok((LogVersionError::InvalidString, position))
        } else if prefix == LogVersionErrorPrefix::ParseIntError as u8 {
            let system_error = SystemError::ParseIntError;
            Ok((LogVersionError::ParseIntError(system_error), position))
        } else if prefix == LogVersionErrorPrefix::UnexpectedLogVersion as u8 {
            Ok((LogVersionError::UnexpectedLogVersion, position))
        } else {
            Ok((LogVersionError::ParseLogVersionErrorError, position))
        }
    }
}

impl fmt::Display for LogVersionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogVersionError::LogVersionParsingError => {
                write!(f, "{}", "LogVersionError::LogVersionParsingError")
            }
            LogVersionError::InvalidString => {
                write!(f, "{}", "LogVersionError::InvalidString")
            }
            LogVersionError::ParseIntError(error) => {
                let error_string = format!("{}", error);
                write!(f, "{}::{}", "LogVersionError::ParseIntError", error_string)
            }
            LogVersionError::UnexpectedLogVersion => {
                write!(f, "{}", "LogVersionError::UnexpectedLogVersion")
            }
            LogVersionError::ParseLogVersionErrorError => {
                write!(f, "{}", "LogVersionError::ParseLogVersionErrorError")
            }
        }
    }
}

impl From<ParseIntError> for LogVersionError {
    fn from(_error: ParseIntError) -> LogVersionError {
        LogVersionError::ParseIntError(SystemError::ParseIntError)
    }
}
