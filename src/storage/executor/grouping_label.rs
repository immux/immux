use crate::utils::varint::{varint_decode, varint_encode, VarIntError};
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum GroupingLabelError {
    VarIntError(VarIntError),
    ParseGroupingLabelErrorError,
}

enum GroupingLabelErrorPrefix {
    VarIntError = 0x01,
    ParseGroupingLabelErrorError = 0x02,
}

impl GroupingLabelError {
    pub fn marshal(&self) -> Vec<u8> {
        match self {
            GroupingLabelError::VarIntError(error) => {
                let mut result = vec![GroupingLabelErrorPrefix::VarIntError as u8];
                let error_bytes = error.marshal();
                result.extend_from_slice(&error_bytes);
                result
            }
            GroupingLabelError::ParseGroupingLabelErrorError => {
                vec![GroupingLabelErrorPrefix::ParseGroupingLabelErrorError as u8]
            }
        }
    }

    pub fn parse(data: &[u8]) -> Result<(GroupingLabelError, usize), GroupingLabelError> {
        let mut position = 0;
        let prefix = data[position];
        position += 1;

        if prefix == GroupingLabelErrorPrefix::VarIntError as u8 {
            let (error, offset) = VarIntError::parse(&data[position..])?;
            position += offset;
            Ok((GroupingLabelError::VarIntError(error), position))
        } else {
            Ok((GroupingLabelError::ParseGroupingLabelErrorError, position))
        }
    }
}

impl From<VarIntError> for GroupingLabelError {
    fn from(error: VarIntError) -> GroupingLabelError {
        GroupingLabelError::VarIntError(error)
    }
}

impl fmt::Display for GroupingLabelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GroupingLabelError::VarIntError(error) => {
                let error_string = format!("{}", error);
                write!(f, "{}::{}", "GroupingLabelError::VarInt", error_string)
            }
            GroupingLabelError::ParseGroupingLabelErrorError => {
                write!(f, "{}", "GroupingLabelError::ParseGroupingLabelErrorError",)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GroupingLabel(Vec<u8>);

impl GroupingLabel {
    pub fn new(data: &[u8]) -> Self {
        GroupingLabel(data.to_vec())
    }

    pub fn marshal(&self) -> Vec<u8> {
        let mut result = vec![];
        let data_length = self.0.len();
        let data_length_bytes = varint_encode(data_length as u64);
        result.extend_from_slice(&data_length_bytes);
        result.extend_from_slice(&self.0.to_vec());
        return result;
    }

    pub fn parse(data: &[u8]) -> Result<(Self, usize), GroupingLabelError> {
        let (data_length, offset) = varint_decode(&data)?;
        let grouping = GroupingLabel::new(&data[offset..offset + data_length as usize]);
        return Ok((grouping, offset + data_length as usize));
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl From<Vec<u8>> for GroupingLabel {
    fn from(data: Vec<u8>) -> Self {
        return GroupingLabel::new(&data);
    }
}

impl From<&[u8]> for GroupingLabel {
    fn from(data: &[u8]) -> Self {
        return GroupingLabel::new(data);
    }
}

impl Into<Vec<u8>> for GroupingLabel {
    fn into(self) -> Vec<u8> {
        return self.0;
    }
}

impl From<&str> for GroupingLabel {
    fn from(data: &str) -> GroupingLabel {
        GroupingLabel::new(data.as_bytes())
    }
}

impl From<&GroupingLabel> for Vec<u8> {
    fn from(data: &GroupingLabel) -> Vec<u8> {
        data.as_bytes().to_vec()
    }
}

impl fmt::Display for GroupingLabel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string = match String::from_utf8(self.0.to_vec()) {
            Err(_error) => String::from(""),
            Ok(s) => String::from(s),
        };
        write!(f, "{}", string)
    }
}

#[cfg(test)]
mod grouping_tests {
    use super::GroupingLabel;
    use immuxsys_dev_utils::dev_utils::{get_grouping_label_errors, GroupingLabelError};

    #[test]
    fn grouping_label_error_reversibility() {
        let grouping_label_errors = get_grouping_label_errors();

        for expected_error in grouping_label_errors {
            let error_bytes = expected_error.marshal();
            let (actual_error, _) = GroupingLabelError::parse(&error_bytes).unwrap();
            assert_eq!(expected_error, actual_error);
        }
    }

    #[test]
    fn test_marshal() {
        let data = [0x00, 0x01, 0x02, 0x03];
        let grouping = GroupingLabel::new(&data);

        let actual_output = grouping.marshal();
        let expected_output = [0x04, 0x00, 0x01, 0x02, 0x03].to_vec();

        assert_eq!(expected_output, actual_output);
    }

    #[test]
    fn test_parse() {
        let data = [0x06, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05];
        let (actual_output, offset) = GroupingLabel::parse(&data).unwrap();
        let expected_output = GroupingLabel::new(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05]);

        assert_eq!(actual_output, expected_output);
        assert_eq!(offset, data.len());
    }

    #[test]
    fn test_reversible() {
        let data = "any_grouping".as_bytes();

        let expected_output = GroupingLabel::new(&data);
        let marshal_data = expected_output.marshal();
        let (actual_output, offset) = GroupingLabel::parse(&marshal_data).unwrap();

        assert_eq!(actual_output, expected_output);
        assert_eq!(offset, 13);
    }
}
