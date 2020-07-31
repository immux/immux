use crate::utils::varint::{varint_decode, varint_encode, VarIntError};
use std::fmt;

#[derive(Debug)]
pub enum GroupingLabelError {
    VarInt(VarIntError),
}

impl From<VarIntError> for GroupingLabelError {
    fn from(error: VarIntError) -> GroupingLabelError {
        GroupingLabelError::VarInt(error)
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
    use crate::storage::executor::grouping_label::GroupingLabel;

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
