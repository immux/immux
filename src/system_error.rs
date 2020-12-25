use std::fmt;
#[derive(Debug, PartialEq)]
pub enum SystemError {
    FromUtf8Error,
    IOError,
    ParseIntError,
    ReceiverError,
    ParseSystemErrorError,
}

impl SystemError {
    pub fn marshal(&self) -> u8 {
        match self {
            SystemError::FromUtf8Error => 0x01,
            SystemError::ParseSystemErrorError => 0x02,
            SystemError::IOError => 0x03,
            SystemError::ParseIntError => 0x04,
            SystemError::ReceiverError => 0x05,
        }
    }
    pub fn parse(data: &[u8]) -> Result<(SystemError, usize), SystemError> {
        let mut position = 0;
        let error_byte = data[position];
        position += 1;

        if error_byte == SystemError::FromUtf8Error as u8 {
            Ok((SystemError::FromUtf8Error, position))
        } else if error_byte == SystemError::IOError as u8 {
            Ok((SystemError::IOError, position))
        } else if error_byte == SystemError::ParseIntError as u8 {
            Ok((SystemError::ParseIntError, position))
        } else if error_byte == SystemError::ReceiverError as u8 {
            Ok((SystemError::ReceiverError, position))
        } else {
            Ok((SystemError::ParseSystemErrorError, position))
        }
    }
}

impl fmt::Display for SystemError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SystemError::FromUtf8Error => write!(f, "{}", "SystemError::FromUtf8Error",),
            SystemError::ParseSystemErrorError => {
                write!(f, "{}", "SystemError::ParseSystemErrorError",)
            }
            SystemError::IOError => write!(f, "{}", "SystemError::IOError",),
            SystemError::ParseIntError => write!(f, "{}", "SystemError::ParseIntError",),
            SystemError::ReceiverError => write!(f, "{}", "SystemError::ReceiverError",),
        }
    }
}
