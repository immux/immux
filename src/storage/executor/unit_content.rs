use std::cmp::Ordering;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt;

use crate::utils::bools::{bool_to_u8, u8_to_bool};
use crate::utils::floats::{f64_to_u8_array, u8_array_to_f64};
use crate::utils::varint::{varint_decode, varint_encode};

pub enum ContentTypePrefix {
    Nil = 0x00,
    String = 0x10,
    Boolean = 0x11,
    Float64 = 0x12,
    Array = 0x20,
    Map = 0x21,
}

impl TryFrom<u8> for ContentTypePrefix {
    type Error = UnitContentError;
    fn try_from(byte: u8) -> Result<Self, UnitContentError> {
        if byte == ContentTypePrefix::Nil as u8 {
            return Ok(ContentTypePrefix::Nil);
        } else if byte == ContentTypePrefix::String as u8 {
            return Ok(ContentTypePrefix::String);
        } else if byte == ContentTypePrefix::Boolean as u8 {
            return Ok(ContentTypePrefix::Boolean);
        } else if byte == ContentTypePrefix::Float64 as u8 {
            return Ok(ContentTypePrefix::Float64);
        } else if byte == ContentTypePrefix::Array as u8 {
            return Ok(ContentTypePrefix::Array);
        } else if byte == ContentTypePrefix::Map as u8 {
            return Ok(ContentTypePrefix::Map);
        } else {
            return Err(UnitContentError::UnexpectedTypePrefix(byte));
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnitContent {
    Nil,
    String(String),
    Bool(bool),
    Float64(f64),
    Array(Vec<UnitContent>),
    Map(HashMap<String, UnitContent>),
}

#[derive(Debug)]
pub enum UnitContentError {
    UnexpectedTypePrefix(u8),
    EmptyInput,
    MissingDataBytes,
    UnexpectedLengthBytes,
}

impl PartialOrd for UnitContent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if let UnitContent::Float64(f1) = self {
            if let UnitContent::Float64(f2) = other {
                return f1.partial_cmp(&f2);
            }
        }
        return None;
    }
}

impl UnitContent {
    pub fn marshal(&self) -> Vec<u8> {
        let mut result = Vec::with_capacity(1);
        match &self {
            UnitContent::Nil => result.push(ContentTypePrefix::Nil as u8),
            UnitContent::Bool(boolean) => {
                result.push(ContentTypePrefix::Boolean as u8);
                result.push(bool_to_u8(*boolean));
            }
            UnitContent::Float64(number_f64) => {
                result.push(ContentTypePrefix::Float64 as u8);
                result.extend_from_slice(&f64_to_u8_array(*number_f64));
            }
            UnitContent::String(string) => {
                let bytes = string.as_bytes();
                result.push(ContentTypePrefix::String as u8);
                result.extend_from_slice(&varint_encode(bytes.len() as u64));
                result.extend_from_slice(bytes)
            }
            UnitContent::Array(array) => {
                result.push(ContentTypePrefix::Array as u8);
                let mut data = Vec::new();

                for content in array {
                    let bytes = content.marshal();
                    data.extend_from_slice(&bytes);
                }

                result.extend_from_slice(&varint_encode(data.len() as u64));
                result.extend_from_slice(&data);
            }
            UnitContent::Map(map) => {
                result.push(ContentTypePrefix::Map as u8);
                let mut data = Vec::new();

                for (key, value) in map.iter() {
                    let key_bytes = key.as_bytes();
                    let value_bytes = value.marshal();
                    data.extend_from_slice(&varint_encode(key_bytes.len() as u64));
                    data.extend_from_slice(key_bytes);
                    data.extend_from_slice(&value_bytes);
                }

                result.extend_from_slice(&varint_encode(data.len() as u64));
                result.extend_from_slice(&data);
            }
        }
        return result;
    }

    pub fn parse(data: &[u8]) -> Result<(Self, usize), UnitContentError> {
        match data.get(0) {
            None => return Err(UnitContentError::EmptyInput.into()),
            Some(first_byte) => {
                let type_prefix = ContentTypePrefix::try_from(*first_byte)?;
                let remaining_bytes = &data[1..];
                match type_prefix {
                    ContentTypePrefix::Nil => {
                        return Ok((UnitContent::Nil, 1));
                    }
                    ContentTypePrefix::Boolean => match remaining_bytes.get(0) {
                        None => return Err(UnitContentError::MissingDataBytes.into()),
                        Some(data_byte) => {
                            return Ok((UnitContent::Bool(u8_to_bool(*data_byte)), 2));
                        }
                    },
                    ContentTypePrefix::Float64 => {
                        if remaining_bytes.len() < 8 {
                            return Err(UnitContentError::MissingDataBytes.into());
                        } else {
                            let array: [u8; 8] = [
                                remaining_bytes[0],
                                remaining_bytes[1],
                                remaining_bytes[2],
                                remaining_bytes[3],
                                remaining_bytes[4],
                                remaining_bytes[5],
                                remaining_bytes[6],
                                remaining_bytes[7],
                            ];
                            return Ok((UnitContent::Float64(u8_array_to_f64(&array)), 9));
                        }
                    }
                    ContentTypePrefix::String => {
                        let (length, offset) = varint_decode(&remaining_bytes)
                            .map_err(|_| UnitContentError::UnexpectedLengthBytes)?;
                        let string_bytes = &remaining_bytes[offset..offset + length as usize];
                        let content_string = String::from_utf8_lossy(string_bytes).to_string();
                        return Ok((
                            UnitContent::String(content_string),
                            1 + offset + length as usize,
                        ));
                    }
                    ContentTypePrefix::Array => {
                        let mut result: Vec<UnitContent> = Vec::with_capacity(1);
                        let (total_length, offset) = varint_decode(&remaining_bytes)
                            .map_err(|_| UnitContentError::UnexpectedLengthBytes)?;
                        let contents_bytes =
                            &remaining_bytes[offset..offset + total_length as usize];

                        let mut position = 0;
                        while position < contents_bytes.len() {
                            let (content, offset) =
                                UnitContent::parse(&contents_bytes[position..])?;
                            result.push(content);
                            position += offset;
                        }

                        return Ok((
                            UnitContent::Array(result),
                            1 + offset + total_length as usize,
                        ));
                    }
                    ContentTypePrefix::Map => {
                        let mut result = HashMap::new();
                        let (total_length, offset) = varint_decode(&remaining_bytes)
                            .map_err(|_| UnitContentError::UnexpectedLengthBytes)?;
                        let map_bytes = &remaining_bytes[offset..offset + total_length as usize];

                        let mut position = 0;
                        while position < map_bytes.len() {
                            let (string_length, offset) = varint_decode(&map_bytes[position..])
                                .map_err(|_| UnitContentError::UnexpectedLengthBytes)?;
                            position += offset;
                            let string_bytes =
                                &map_bytes[position..position + string_length as usize];
                            position += string_length as usize;

                            let (content, offset) = UnitContent::parse(&map_bytes[position..])?;
                            position += offset;

                            let content_string = String::from_utf8_lossy(string_bytes).to_string();
                            result.insert(content_string, content);
                        }

                        return Ok((UnitContent::Map(result), 1 + offset + total_length as usize));
                    }
                }
            }
        }
    }
}

impl fmt::Display for UnitContent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnitContent::Nil => write!(f, "{}", "Nil".to_string()),
            UnitContent::String(string) => write!(f, "{}{}{}", "\"", string.clone(), "\""),
            UnitContent::Float64(float) => write!(f, "{}", float),
            UnitContent::Bool(b) => {
                write!(f, "{}", (if *b { "true" } else { "false" }).to_string())
            }
            UnitContent::Map(map) => {
                let kv_pairs: Vec<String> = map
                    .iter()
                    .map(|(key, content)| format!("{}:{}", key, content.to_string()))
                    .collect();
                write!(f, "{}{}{}", "{", kv_pairs.join(","), "}")
            }
            UnitContent::Array(array) => {
                let kv_pairs: Vec<String> =
                    array.iter().map(|content| content.to_string()).collect();
                write!(f, "{}{}{}", "[", kv_pairs.join(","), "]")
            }
        }
    }
}

impl From<&str> for UnitContent {
    fn from(incoming_str: &str) -> UnitContent {
        if incoming_str.starts_with("{") && incoming_str.ends_with("}") {
            let body_length = incoming_str.len();
            let sub_body = &incoming_str[1..body_length - 1];
            let kv_paris_str: Vec<&str> = sub_body.split(",").collect();

            let mut map = HashMap::new();

            for kv_pair_str in kv_paris_str.iter() {
                let kv_pair_str = kv_pair_str.trim();
                let segments: Vec<&str> = kv_pair_str.split(":").collect();
                let key = segments[0].trim();
                let content = UnitContent::from(segments[1].trim());
                map.insert(key.to_string(), content);
            }
            UnitContent::Map(map)
        } else if let Ok(num_f64) = incoming_str.parse::<f64>() {
            UnitContent::Float64(num_f64)
        } else if incoming_str == "true" {
            UnitContent::Bool(true)
        } else if incoming_str == "false" {
            UnitContent::Bool(false)
        } else if incoming_str == "Nil" {
            UnitContent::Nil
        } else if incoming_str.starts_with("[") && incoming_str.ends_with("]") {
            let body_length = incoming_str.len();
            let sub_body = &incoming_str[1..body_length - 1];
            let contents_str: Vec<&str> = sub_body.split(",").collect();

            let mut array = vec![];

            for content_str in contents_str.into_iter() {
                let content = UnitContent::from(content_str);
                array.push(content);
            }
            UnitContent::Array(array)
        } else if (incoming_str.starts_with("\"") && incoming_str.ends_with("\""))
            || (incoming_str.starts_with("'") && incoming_str.ends_with("'"))
        {
            let str_length = incoming_str.len();
            let sub_string = &incoming_str[1..str_length - 1];
            UnitContent::String(String::from(sub_string))
        } else {
            UnitContent::String(String::from(incoming_str))
        }
    }
}

#[cfg(test)]
mod unit_content_tests {
    use std::clone::Clone;
    use std::collections::HashMap;

    use crate::storage::executor::unit_content::UnitContent;
    use crate::utils::varint::varint_encode;

    fn permutation<T: Clone>(array: &[T]) -> Vec<Vec<T>> {
        if array.len() == 0 {
            return vec![];
        }

        if array.len() == 1 {
            return vec![array.to_vec()];
        }

        let mut result = vec![];

        for (index, pivot) in array.iter().enumerate() {
            let mut new_array = vec![];
            new_array.extend_from_slice(&array[0..index]);
            new_array.extend_from_slice(&array[index + 1..]);

            let temp_result = permutation(&new_array);

            for arr in temp_result.iter() {
                let mut temp_arr = vec![];
                temp_arr.push(pivot.clone());
                temp_arr.extend_from_slice(arr.as_slice());
                result.push(temp_arr);
            }
        }

        return result;
    }

    #[test]
    fn unit_content_marshal_parse_reversibility() {
        let mut map = HashMap::new();
        map.insert(
            String::from("brand"),
            UnitContent::String(String::from("apple")),
        );
        map.insert(String::from("price"), UnitContent::Float64(4000.0));
        let map_content = UnitContent::Map(map);

        let contents = vec![
            map_content,
            UnitContent::String(String::from("this is a string")),
            UnitContent::Bool(true),
            UnitContent::Bool(false),
            UnitContent::Nil,
            UnitContent::String(String::from("true")),
            UnitContent::String(String::from("false")),
            UnitContent::String(String::from("Nil")),
            UnitContent::Array(vec![
                UnitContent::Float64(1.0),
                UnitContent::String(String::from("Andy")),
            ]),
        ];

        for content in contents {
            let expected_output = &content;
            let content_bytes = content.marshal();
            let (actual_output, offset) = UnitContent::parse(&content_bytes).unwrap();
            assert_eq!(expected_output, &actual_output);
            assert_eq!(content_bytes.len(), offset);
        }
    }

    #[test]
    fn unit_content_from_string() {
        let mut map = HashMap::new();
        map.insert(
            String::from("name"),
            UnitContent::String(String::from("Tom")),
        );
        map.insert(String::from("age"), UnitContent::Float64(40.0));

        let content_pairs = [
            (
                "\"this is a string\"",
                UnitContent::String(String::from("this is a string")),
            ),
            ("true", UnitContent::Bool(true)),
            ("false", UnitContent::Bool(false)),
            ("Nil", UnitContent::Nil),
            ("\"true\"", UnitContent::String(String::from("true"))),
            ("\"false\"", UnitContent::String(String::from("false"))),
            ("\"Nil\"", UnitContent::String(String::from("Nil"))),
            ("{name:\"Tom\", age:40}", UnitContent::Map(map)),
            (
                "[1,2,3,\"Andy\",5]",
                UnitContent::Array(vec![
                    UnitContent::Float64(1.0),
                    UnitContent::Float64(2.0),
                    UnitContent::Float64(3.0),
                    UnitContent::String(String::from("Andy")),
                    UnitContent::Float64(5.0),
                ]),
            ),
            ("\"\"", UnitContent::String(String::from(""))),
            ("", UnitContent::String(String::from(""))),
        ];

        for (content_str, expected_output) in content_pairs.iter() {
            let actual_output = UnitContent::from(*content_str);
            assert_eq!(&actual_output, expected_output);
        }
    }

    #[test]
    fn weak_permutation_test() {
        let array = [1, 2, 3];
        let actual_result = permutation(&array);
        let expected_result = vec![
            [1, 2, 3],
            [1, 3, 2],
            [2, 1, 3],
            [2, 3, 1],
            [3, 1, 2],
            [3, 2, 1],
        ];

        assert_eq!(actual_result, expected_result);
    }

    fn get_unit_content_map_key_value_pairs() -> Vec<(String, UnitContent)> {
        vec![
            (String::from("a"), UnitContent::Nil),
            (String::from("b"), UnitContent::String("hello".to_string())),
            (String::from("c"), UnitContent::Bool(true)),
            (String::from("d"), UnitContent::Float64(1.5)),
            (
                String::from("e"),
                UnitContent::Array(vec![
                    UnitContent::String("world".to_string()),
                    UnitContent::Float64(1.5),
                    UnitContent::Array(vec![UnitContent::Bool(true), UnitContent::Bool(false)]),
                ]),
            ),
        ]
    }

    fn get_permutation_content_map_bytes() -> Vec<Vec<u8>> {
        let key_value_pairs = get_unit_content_map_key_value_pairs();

        let permutation: Vec<Vec<(String, UnitContent)>> = permutation(&key_value_pairs);
        let mut result: Vec<Vec<u8>> = vec![];

        for pairs in permutation.iter() {
            let mut possible_output: Vec<u8> = vec![0x21, 0x35];

            for (key, value) in pairs {
                let key_length = varint_encode(key.len() as u64);
                let key_bytes = key.as_bytes();
                let content_bytes = value.marshal();

                possible_output.extend_from_slice(&key_length);
                possible_output.extend_from_slice(key_bytes);
                possible_output.extend_from_slice(&content_bytes);
            }

            result.push(possible_output);
        }

        return result;
    }

    fn get_fixture() -> Vec<(Option<UnitContent>, Vec<Vec<u8>>)> {
        let key_value_pairs = get_unit_content_map_key_value_pairs();
        let mut map: HashMap<String, UnitContent> = HashMap::new();
        for (key, value) in key_value_pairs.iter() {
            map.insert(key.clone(), value.clone());
        }
        let content_map = UnitContent::Map(map);
        let permutation_content_map_bytes = get_permutation_content_map_bytes();

        vec![
            (Some(UnitContent::Nil), vec![vec![0x00]]),
            (Some(UnitContent::Bool(true)), vec![vec![0x11, 0x01]]),
            (Some(UnitContent::Bool(false)), vec![vec![0x11, 0x00]]),
            (
                Some(UnitContent::Float64(1.5)),
                vec![vec![0x12, 0, 0, 0, 0, 0, 0, 0xf8, 0x3f]],
            ),
            (
                Some(UnitContent::String(String::from("hello"))),
                vec![vec![0x10, 0x05, 0x68, 0x65, 0x6c, 0x6c, 0x6f]],
            ),
            (
                Some(UnitContent::Array(vec![
                    UnitContent::Nil,
                    UnitContent::String(String::from("hello")),
                    UnitContent::Bool(true),
                    UnitContent::Float64(1.5),
                    UnitContent::Array(vec![UnitContent::Nil]),
                ])),
                vec![vec![
                    0x20, /*Array prefix*/
                    0x16, /*Length*/
                    0x00, /*Nil*/
                    0x10, 0x05, 0x68, 0x65, 0x6c, 0x6c, 0x6f, /*String*/
                    0x11, 0x01, /*bool true*/
                    0x12, 0, 0, 0, 0, 0, 0, 0xf8, 0x3f, /*float*/
                    0x20, /*Array prefix*/
                    0x01, /*Length*/
                    0x00, /*Nil*/
                ]],
            ),
            (Some(content_map), permutation_content_map_bytes),
            // Non-existing type
            (None, vec![vec![0xaa, 0x01, 0x02, 0x03]]),
            // Malformed boolean
            (None, vec![vec![0x11]]),
            // Malformed bytes with wrong varint length
            (None, vec![vec![0xff, 0xff, 0x10]]),
            // Empty input
            (None, vec![]),
        ]
    }

    #[test]
    fn test_serialize() {
        let table = get_fixture();
        assert!(table.len() > 0);
        for row in table {
            if let (Some(content), expected) = row {
                let serialized = content.marshal();
                assert!(expected.contains(&serialized));
            } else {
                // Malformed bytes, skip
            }
        }
    }

    #[test]
    fn test_deserialize() {
        let table = get_fixture();
        assert!(table.len() > 0);
        for row in table {
            let (expected, bytes_vec) = row;
            if let Some(expected_content) = expected {
                for bytes in bytes_vec {
                    let (parsed, actual_size) = UnitContent::parse(&bytes).unwrap();
                    let expected_size = expected_content.marshal().len();

                    assert_eq!(expected_content, parsed);
                    assert_eq!(expected_size, actual_size);
                }
            } else {
                for bytes in bytes_vec {
                    match UnitContent::parse(&bytes) {
                        Ok(_) => panic!("Should not be able to parse {:?}", bytes),
                        Err(_) => (),
                    }
                }
            }
        }
    }
}
