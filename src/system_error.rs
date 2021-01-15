use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum SystemError {
    FromUtf8Error,
    IOError,
    ParseIntError,
    ReceiverError,
    ParseSystemErrorError,
}

enum SystemErrorPrefix {
    FromUtf8Error = 0x01,
    IOError = 0x02,
    ParseIntError = 0x03,
    ReceiverError = 0x04,
    ParseSystemErrorError = 0x05,
}

impl SystemError {
    pub fn marshal(&self) -> Vec<u8> {
        match self {
            SystemError::FromUtf8Error => vec![SystemErrorPrefix::FromUtf8Error as u8],
            SystemError::ParseSystemErrorError => {
                vec![SystemErrorPrefix::ParseSystemErrorError as u8]
            }
            SystemError::IOError => vec![SystemErrorPrefix::IOError as u8],
            SystemError::ParseIntError => vec![SystemErrorPrefix::ParseIntError as u8],
            SystemError::ReceiverError => vec![SystemErrorPrefix::ReceiverError as u8],
        }
    }
    pub fn parse(data: &[u8]) -> Result<(SystemError, usize), SystemError> {
        let mut position = 0;
        let error_byte = data[position];
        position += 1;

        if error_byte == SystemErrorPrefix::FromUtf8Error as u8 {
            Ok((SystemError::FromUtf8Error, position))
        } else if error_byte == SystemErrorPrefix::IOError as u8 {
            Ok((SystemError::IOError, position))
        } else if error_byte == SystemErrorPrefix::ParseIntError as u8 {
            Ok((SystemError::ParseIntError, position))
        } else if error_byte == SystemErrorPrefix::ReceiverError as u8 {
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

#[cfg(test)]
mod system_error_test {
    use super::SystemError;

    #[test]
    fn test_error_reversibility() {
        let errors = vec![
            SystemError::ReceiverError,
            SystemError::IOError,
            SystemError::FromUtf8Error,
            SystemError::ParseIntError,
            SystemError::ParseSystemErrorError,
        ];

        for expected_error in errors {
            let error_bytes = expected_error.marshal();
            let (acutal_error, _) = SystemError::parse(&error_bytes).unwrap();
            assert_eq!(expected_error, acutal_error);
        }
    }
}
