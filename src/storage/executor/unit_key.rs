use std::fmt;

use crate::utils::varint::{varint_decode, varint_encode, VarIntError};

#[derive(Debug, Clone, PartialEq)]
pub enum UnitKeyError {
    VarIntError(VarIntError),
    ParseUnitKeyErrorError,
}

enum UnitKeyErrorPrefix {
    VarIntError = 0x01,
    ParseUnitKeyErrorError = 0x02,
}

impl UnitKeyError {
    pub fn marshal(&self) -> Vec<u8> {
        match self {
            UnitKeyError::VarIntError(error) => {
                let mut result = vec![UnitKeyErrorPrefix::VarIntError as u8];
                let error_bytes = error.marshal();
                result.extend_from_slice(&error_bytes);
                result
            }
            UnitKeyError::ParseUnitKeyErrorError => {
                vec![UnitKeyErrorPrefix::ParseUnitKeyErrorError as u8]
            }
        }
    }

    pub fn parse(data: &[u8]) -> Result<(UnitKeyError, usize), UnitKeyError> {
        let mut position = 0;
        let prefix = data[position];
        position += 1;

        if prefix == UnitKeyErrorPrefix::VarIntError as u8 {
            let (error, offset) = VarIntError::parse(&data[position..])?;
            position += offset;
            Ok((UnitKeyError::VarIntError(error), position))
        } else {
            Ok((UnitKeyError::ParseUnitKeyErrorError, position))
        }
    }
}

impl fmt::Display for UnitKeyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnitKeyError::VarIntError(error) => {
                let error_string = format!("{}", error);
                write!(f, "{}::{}", "UnitKeyError::VarIntError", error_string)
            }
            UnitKeyError::ParseUnitKeyErrorError => {
                write!(f, "{}", "UnitKeyError::ParseUnitKeyErrorError")
            }
        }
    }
}

impl From<VarIntError> for UnitKeyError {
    fn from(error: VarIntError) -> UnitKeyError {
        UnitKeyError::VarIntError(error)
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct UnitKey(Vec<u8>);

impl UnitKey {
    pub fn new(data: &[u8]) -> Self {
        Self(data.to_owned())
    }
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
    pub fn marshal(&self) -> Vec<u8> {
        let mut result = vec![];
        let data_length = self.as_bytes().len() as u64;
        let data_length_bytes = varint_encode(data_length);
        result.extend_from_slice(&data_length_bytes);
        result.extend_from_slice(&self.as_bytes());
        return result;
    }
    pub fn parse(data: &[u8]) -> Result<(Self, usize), UnitKeyError> {
        let mut position = 0;
        let (key_length, varint_size) = varint_decode(&data[position..])?;
        position += varint_size;

        let key = UnitKey::new(&data[position..position + key_length as usize]);
        position += key_length as usize;

        return Ok((key, position));
    }
}

impl From<Vec<u8>> for UnitKey {
    fn from(data: Vec<u8>) -> UnitKey {
        UnitKey::new(&data)
    }
}

impl From<&[u8]> for UnitKey {
    fn from(data: &[u8]) -> UnitKey {
        UnitKey::new(data)
    }
}

impl From<&str> for UnitKey {
    fn from(data: &str) -> UnitKey {
        UnitKey::new(data.as_bytes())
    }
}

impl From<UnitKey> for Vec<u8> {
    fn from(key: UnitKey) -> Vec<u8> {
        key.0
    }
}

impl fmt::Display for UnitKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(self.0.as_slice()))
    }
}

#[cfg(test)]
mod unit_key_tests {
    use crate::storage::executor::unit_key::UnitKey;
    use immuxsys_dev_utils::dev_utils::{get_unit_key_errors, UnitKeyError};

    #[test]
    fn unit_key_error_reversibility() {
        let unit_key_errors = get_unit_key_errors();

        for expected_error in unit_key_errors {
            let error_bytes = expected_error.marshal();
            let (actual_error, _) = UnitKeyError::parse(&error_bytes).unwrap();
            assert_eq!(actual_error, expected_error);
        }
    }

    #[test]
    fn test_unit_key_marshal_parse() {
        let expected_output = UnitKey::new(&vec![3, 2, 1, 0]);
        let key_bytes = expected_output.marshal();
        let (actual_output, offset) = UnitKey::parse(&key_bytes).unwrap();
        assert_eq!(expected_output, actual_output);
        assert_eq!(key_bytes.len(), offset);
    }

    #[test]
    fn test_from_vec() {
        let input = vec![1, 2, 3];
        let key = UnitKey::from(input.clone());
        assert_eq!(key.as_bytes(), input.as_slice())
    }

    #[test]
    fn test_from_slice() {
        let input = vec![3, 2, 1, 0];
        let key = UnitKey::from(input.as_slice());
        assert_eq!(key.as_bytes(), input.as_slice())
    }

    #[test]
    fn test_from_str() {
        let thing = "abc";
        let key = UnitKey::from(thing);
        assert_eq!(key.as_bytes(), &[97, 98, 99])
    }

    #[test]
    fn test_to_vec() {
        let key = UnitKey::new(&[1, 2, 3]);
        let v: Vec<u8> = key.into();
        assert_eq!(v, vec![1, 2, 3])
    }
}
