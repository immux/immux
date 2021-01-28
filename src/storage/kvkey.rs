use std::fmt;

use crate::storage::executor::grouping_label::GroupingLabel;
use crate::storage::executor::unit_key::UnitKey;
use crate::utils::varint::{varint_decode, varint_encode, VarIntError};

#[derive(Debug, Clone, PartialEq)]
pub enum KVKeyError {
    VarIntError(VarIntError),
    ParseKVKeyErrorError,
}

enum KVKeyErrorPrefix {
    VarIntError = 0x01,
    ParseKVKeyErrorError = 0x02,
}

impl KVKeyError {
    pub fn marshal(&self) -> Vec<u8> {
        match self {
            KVKeyError::VarIntError(error) => {
                let mut result = vec![KVKeyErrorPrefix::VarIntError as u8];
                let error_bytes = error.marshal();
                result.extend_from_slice(&error_bytes);
                result
            }
            KVKeyError::ParseKVKeyErrorError => vec![KVKeyErrorPrefix::ParseKVKeyErrorError as u8],
        }
    }

    pub fn parse(data: &[u8]) -> Result<(KVKeyError, usize), KVKeyError> {
        let mut position = 0;
        let prefix = data[position];
        position += 1;

        if prefix == KVKeyErrorPrefix::VarIntError as u8 {
            let (error, offset) = VarIntError::parse(&data[position..])?;
            position += offset;
            Ok((KVKeyError::VarIntError(error), position))
        } else {
            Ok((KVKeyError::ParseKVKeyErrorError, position))
        }
    }
}

impl fmt::Display for KVKeyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KVKeyError::VarIntError(error) => {
                let error_string = format!("{}", error);
                write!(f, "{}::{}", "KVKeyError::VarIntError", error_string)
            }
            KVKeyError::ParseKVKeyErrorError => {
                write!(f, "{}", "KVKeyError::ParseKVKeyErrorError")
            }
        }
    }
}

impl From<VarIntError> for KVKeyError {
    fn from(err: VarIntError) -> KVKeyError {
        KVKeyError::VarIntError(err)
    }
}

/// The raw key of the storage engine.
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct KVKey(Vec<u8>);

impl KVKey {
    pub fn new(data: &[u8]) -> Self {
        Self(data.to_owned())
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut serialized = Vec::new();
        let bytes = self.as_bytes();
        serialized.extend_from_slice(&varint_encode(bytes.len() as u64));
        serialized.extend_from_slice(bytes);
        return serialized;
    }

    pub fn from_grouping_and_unit_key(grouping: &GroupingLabel, unit_key: &UnitKey) -> Self {
        let mut kvkey_bytes = vec![];
        let grouping_bytes = grouping.marshal();
        let unit_key_bytes = unit_key.as_bytes();

        kvkey_bytes.extend_from_slice(&grouping_bytes);
        kvkey_bytes.extend_from_slice(&unit_key_bytes);

        KVKey::new(&kvkey_bytes)
    }

    pub fn parse(data: &[u8]) -> Result<(GroupingLabel, UnitKey), KVKeyError> {
        let (data_length, offset) = varint_decode(&data)?;
        let grouping = GroupingLabel::new(&data[offset..offset + data_length as usize]);
        let remaining_bytes = &data[offset + data_length as usize..];
        let unit_key = UnitKey::from(remaining_bytes);
        return Ok((grouping, unit_key));
    }
}

impl From<Vec<u8>> for KVKey {
    fn from(data: Vec<u8>) -> KVKey {
        KVKey::new(&data)
    }
}

impl From<&[u8]> for KVKey {
    fn from(data: &[u8]) -> KVKey {
        KVKey::new(data)
    }
}

impl From<&str> for KVKey {
    fn from(data: &str) -> KVKey {
        KVKey::new(data.as_bytes())
    }
}

impl From<KVKey> for Vec<u8> {
    fn from(key: KVKey) -> Vec<u8> {
        key.0
    }
}

#[cfg(test)]
mod kvkey_tests {
    use super::KVKey;
    use immuxsys_dev_utils::dev_utils::{get_kvkey_errors, KVKeyError};

    #[test]
    fn kvkey_error_reversibility() {
        let kvkey_errors = get_kvkey_errors();

        for expected_error in kvkey_errors {
            let error_bytes = expected_error.marshal();
            let (actual_error, _) = KVKeyError::parse(&error_bytes).unwrap();
            assert_eq!(actual_error, expected_error);
        }
    }

    #[test]
    fn test_from_vec() {
        let input = vec![1, 2, 3];
        let key = KVKey::from(input.clone());
        assert_eq!(key.as_bytes(), input.as_slice())
    }

    #[test]
    fn test_from_slice() {
        let input = vec![3, 2, 1, 0];
        let key = KVKey::from(input.as_slice());
        assert_eq!(key.as_bytes(), input.as_slice())
    }

    #[test]
    fn test_from_str() {
        let thing = "abc";
        let key = KVKey::from(thing);
        assert_eq!(key.as_bytes(), &[97, 98, 99])
    }

    #[test]
    fn test_to_vec() {
        let key = KVKey::new(&[1, 2, 3]);
        let v: Vec<u8> = key.into();
        assert_eq!(v, vec![1, 2, 3])
    }
}
