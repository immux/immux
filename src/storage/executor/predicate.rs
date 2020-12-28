//! JavaScript-compatible "predicate" expressions for unit filtering.
//! A predicate specifies conditions for a UnitContent, checking whether the content satisfies
//! the conditions.
//! For example, a client may request data units that satisfy "this.price<20 && this.type=='fruit'",
//! for some inexpensive fruits. This string is in terms parsed into a Predicate struct, which can
//! be evaluated against UnitContents.

use std::fmt;

use crate::constants::FIELD_PATH_SELF_TOKEN;
use crate::storage::executor::predicate::Token::ContentString;
use crate::storage::executor::unit_content::{UnitContent, UnitContentError};
use crate::utils::serialize::{extract_data_with_varint_width, prepend_varint_width};
use crate::utils::varint::{varint_decode, VarIntError};
use std::string::FromUtf8Error;

#[derive(Debug, PartialOrd, PartialEq)]
enum Token {
    This,
    Dot,
    Identifier(String),
    ContentString(String),

    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,

    Or,
    And,
    Not,
}

#[derive(Debug, PartialEq)]
pub enum PredicateError {
    UnexpectedToken,
    MalformedTokens,

    InsufficientBytes,
    MalformedBytes(VarIntError),

    UnexpectedPrefix(u8),

    UnitContent(UnitContentError),

    ParsePredicateErrorError,
}

pub enum PredicateErrorPrefix {
    UnexpectedToken = 0x01,
    MalformedTokens = 0x02,
    InsufficientBytes = 0x03,
    MalformedBytes = 0x04,
    UnexpectedPrefix = 0x05,
    UnitContent = 0x06,
    ParsePredicateErrorError = 0x07,
}

impl PredicateError {
    pub fn marshal(&self) -> Vec<u8> {
        match self {
            PredicateError::UnexpectedToken => vec![PredicateErrorPrefix::UnexpectedToken as u8],
            PredicateError::MalformedTokens => vec![PredicateErrorPrefix::MalformedTokens as u8],
            PredicateError::InsufficientBytes => {
                vec![PredicateErrorPrefix::InsufficientBytes as u8]
            }
            PredicateError::MalformedBytes(error) => {
                let mut result = vec![PredicateErrorPrefix::MalformedBytes as u8];
                let error_bytes = error.marshal();
                result.extend_from_slice(&error_bytes);
                return result;
            }
            PredicateError::UnexpectedPrefix(byte) => {
                let mut result = vec![PredicateErrorPrefix::UnexpectedPrefix as u8];

                result.push(byte.clone());
                return result;
            }
            PredicateError::UnitContent(content_error) => {
                let mut result = vec![PredicateErrorPrefix::UnitContent as u8];
                let error_bytes = content_error.marshal();
                result.extend_from_slice(&error_bytes);
                return result;
            }
            PredicateError::ParsePredicateErrorError => {
                vec![PredicateErrorPrefix::ParsePredicateErrorError as u8]
            }
        }
    }

    pub fn parse(data: &[u8]) -> Result<(PredicateError, usize), PredicateError> {
        let mut position = 0;
        let prefix = data[position];
        position += 1;

        if prefix == PredicateErrorPrefix::UnexpectedToken as u8 {
            Ok((PredicateError::UnexpectedToken, position))
        } else if prefix == PredicateErrorPrefix::MalformedTokens as u8 {
            Ok((PredicateError::MalformedTokens, position))
        } else if prefix == PredicateErrorPrefix::InsufficientBytes as u8 {
            Ok((PredicateError::InsufficientBytes, position))
        } else if prefix == PredicateErrorPrefix::MalformedBytes as u8 {
            let (error, offset) = VarIntError::parse(&data[position..])?;
            position += offset;
            Ok((PredicateError::MalformedBytes(error), position))
        } else if prefix == PredicateErrorPrefix::UnexpectedPrefix as u8 {
            let byte = data[position];
            position += 1;

            return Ok((PredicateError::UnexpectedPrefix(byte), position));
        } else if prefix == PredicateErrorPrefix::UnitContent as u8 {
            let (error, offset) = UnitContentError::parse(&data[position..])?;
            position += offset;
            Ok((PredicateError::UnitContent(error), position))
        } else {
            return Ok((PredicateError::ParsePredicateErrorError, position));
        }
    }
}

impl fmt::Display for PredicateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PredicateError::UnexpectedToken => {
                write!(f, "{}", "PredicateError::UnexpectedToken")
            }
            PredicateError::MalformedTokens => {
                write!(f, "{}", "PredicateError::MalformedTokens")
            }
            PredicateError::InsufficientBytes => {
                write!(f, "{}", "PredicateError::InsufficientBytes")
            }
            PredicateError::MalformedBytes(error) => {
                write!(f, "{}::{}", "PredicateError::MalformedBytes", error)
            }
            PredicateError::UnexpectedPrefix(byte) => {
                write!(f, "{}::{}", "PredicateError::MalformedBytes", byte)
            }
            PredicateError::UnitContent(content_error) => {
                write!(f, "{}::{}", "PredicateError::MalformedBytes", content_error)
            }
            PredicateError::ParsePredicateErrorError => {
                write!(f, "{}", "PredicateError::ParsePredicateErrorToStringError")
            }
        }
    }
}

impl From<VarIntError> for PredicateError {
    fn from(err: VarIntError) -> Self {
        Self::MalformedBytes(err)
    }
}

impl From<UnitContentError> for PredicateError {
    fn from(err: UnitContentError) -> Self {
        Self::UnitContent(err)
    }
}

impl From<FromUtf8Error> for PredicateError {
    fn from(_err: FromUtf8Error) -> PredicateError {
        PredicateError::ParsePredicateErrorError
    }
}

pub type PredicateResult<T> = Result<T, PredicateError>;

#[derive(Debug, PartialEq, Clone)]
pub struct FieldPath(Vec<String>);

impl From<Vec<String>> for FieldPath {
    fn from(data: Vec<String>) -> Self {
        return Self(data);
    }
}

/// Represents segments of data path in a data, such as this.name.familyName
impl FieldPath {
    fn new() -> Self {
        Self(vec![])
    }
    fn inner(&self) -> &[String] {
        &self.0
    }
    fn push(&mut self, segment: &str) {
        self.0.push(segment.to_string())
    }
    fn len(&self) -> usize {
        self.0.len()
    }
    fn serialize(&self) -> Vec<u8> {
        self.inner()
            .iter()
            .map(|s| s.as_bytes())
            .map(|bytes| prepend_varint_width(bytes))
            .flatten()
            .collect()
    }
    fn parse(data: &[u8]) -> Result<Self, VarIntError> {
        let mut i = 0;
        let mut paths: Vec<String> = Vec::new();
        while i < data.len() {
            let (len, offset) = varint_decode(&data[i..])?;
            let string_data = &data[i + offset..i + offset + len as usize];
            let str = String::from_utf8_lossy(string_data);
            paths.push(str.to_string());
            i += offset + len as usize;
        }
        return Ok(Self::from(paths));
    }
    fn shift(&self) -> (Option<String>, Self) {
        if self.0.len() == 0 {
            return (None, Self::new());
        } else {
            return (
                Some(self.0[0].clone()),
                FieldPath::from(self.0[1..].to_vec()),
            );
        }
    }
}

impl fmt::Display for FieldPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.len() == 0 {
            write!(f, "{}", FIELD_PATH_SELF_TOKEN)
        } else {
            let subfields = self.inner().join(".");
            write!(f, "{}.{}", FIELD_PATH_SELF_TOKEN, subfields)
        }
    }
}

#[cfg(test)]
mod field_path_test {
    use crate::storage::executor::predicate::FieldPath;

    #[test]
    fn test_format() {
        let path = FieldPath::from(vec![String::from("hello"), String::from("world")]);
        let s = format!("{}", path);
        assert_eq!(s, String::from("this.hello.world"));

        let empty_path = FieldPath::from(vec![]);
        let s2 = format!("{}", empty_path);
        assert_eq!(s2, String::from("this"));
    }

    #[test]
    fn test_serialization() {
        let path = FieldPath::from(vec![String::from("hello"), String::from("world")]);
        let expected_serialization: Vec<u8> = vec![
            0x05, // width
            0x68, 0x65, 0x6c, 0x6c, 0x6f, // "hello"
            0x05, // width
            0x77, 0x6f, 0x72, 0x6c, 0x64, // "world"
        ];
        assert_eq!(
            FieldPath::parse(&expected_serialization).unwrap().inner(),
            path.inner()
        );
        assert_eq!(path.serialize(), expected_serialization);
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum PrimitivePredicate {
    Equal(FieldPath, UnitContent),
    GreaterThan(FieldPath, UnitContent),
    GreaterThanOrEqual(FieldPath, UnitContent),
    LessThan(FieldPath, UnitContent),
    LessThanOrEqual(FieldPath, UnitContent),
}

#[derive(Debug, PartialEq, Clone)]
pub enum CompoundPredicate {
    Not(Box<Predicate>),
    Or(Vec<Predicate>),
    And(Vec<Predicate>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Predicate {
    Primitive(PrimitivePredicate),
    Compound(CompoundPredicate),
}

fn combine_subpredicates(subpredicates: &[Predicate], separator: &str) -> String {
    subpredicates
        .iter()
        .map(|predicate| format!("{}", predicate))
        .collect::<Vec<String>>()
        .join(separator)
}

impl fmt::Display for Predicate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Predicate::Primitive(p) => match p {
                PrimitivePredicate::GreaterThan(path, content) => write!(f, "{}>{}", path, content),
                PrimitivePredicate::GreaterThanOrEqual(path, content) => {
                    write!(f, "{}>={}", path, content)
                }
                PrimitivePredicate::LessThan(path, content) => write!(f, "{}<{}", path, content),
                PrimitivePredicate::LessThanOrEqual(path, content) => {
                    write!(f, "{}<={}", path, content)
                }
                PrimitivePredicate::Equal(path, content) => write!(f, "{}=={}", path, content),
            },
            Predicate::Compound(p) => match p {
                CompoundPredicate::And(subpredicates) => {
                    write!(f, "{}", combine_subpredicates(subpredicates, "&&"))
                }
                CompoundPredicate::Or(subpredicates) => {
                    write!(f, "{}", combine_subpredicates(subpredicates, "||"))
                }
                CompoundPredicate::Not(predicate) => write!(f, "!{}", predicate),
            },
        }
    }
}

enum PredicatePrefix {
    PrimitiveEqual = 0x00,
    PrimitiveGreaterThan = 0x01,
    PrimitiveGreaterThanOrEqual = 0x02,
    PrimitiveLessThan = 0x03,
    PrimitiveLessThanOrEqual = 0x04,

    CompoundAnd = 0xA0,
    CompoundOr = 0xA1,
    CompoundNot = 0x0A2,
}

/// Represents a JavaScript-compatible expression that could be checked against UnitContent.
impl Predicate {
    /// Transform strings like "this.age==42&&this.answer=='universe'" to a predicate
    pub fn parse_str(str: &str) -> PredicateResult<Predicate> {
        let tokens = tokenize(str);
        Self::parse_tokens(&tokens)
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        match self {
            Predicate::Primitive(p) => match p {
                PrimitivePredicate::Equal(path, content) => {
                    result.push(PredicatePrefix::PrimitiveEqual as u8);
                    let path_bytes = path.serialize();
                    let content_bytes = content.marshal();
                    result.extend(prepend_varint_width(&path_bytes));
                    result.extend(prepend_varint_width(&content_bytes));
                }
                PrimitivePredicate::GreaterThan(path, content) => {
                    result.push(PredicatePrefix::PrimitiveGreaterThan as u8);
                    let path_bytes = path.serialize();
                    let content_bytes = content.marshal();
                    result.extend(prepend_varint_width(&path_bytes));
                    result.extend(prepend_varint_width(&content_bytes));
                }
                PrimitivePredicate::GreaterThanOrEqual(path, content) => {
                    result.push(PredicatePrefix::PrimitiveGreaterThanOrEqual as u8);
                    let path_bytes = path.serialize();
                    let content_bytes = content.marshal();
                    result.extend(prepend_varint_width(&path_bytes));
                    result.extend(prepend_varint_width(&content_bytes));
                }
                PrimitivePredicate::LessThan(path, content) => {
                    result.push(PredicatePrefix::PrimitiveLessThan as u8);
                    let path_bytes = path.serialize();
                    let content_bytes = content.marshal();
                    result.extend(prepend_varint_width(&path_bytes));
                    result.extend(prepend_varint_width(&content_bytes));
                }
                PrimitivePredicate::LessThanOrEqual(path, content) => {
                    result.push(PredicatePrefix::PrimitiveLessThanOrEqual as u8);
                    let path_bytes = path.serialize();
                    let content_bytes = content.marshal();
                    result.extend(prepend_varint_width(&path_bytes));
                    result.extend(prepend_varint_width(&content_bytes));
                }
            },
            Predicate::Compound(p) => match p {
                CompoundPredicate::And(predicates) => {
                    result.push(PredicatePrefix::CompoundAnd as u8);
                    for predicate in predicates {
                        result.extend(prepend_varint_width(&predicate.serialize()))
                    }
                }
                CompoundPredicate::Or(predicates) => {
                    result.push(PredicatePrefix::CompoundOr as u8);
                    for predicate in predicates {
                        result.extend(prepend_varint_width(&predicate.serialize()))
                    }
                }
                CompoundPredicate::Not(predicate) => {
                    result.push(PredicatePrefix::CompoundNot as u8);
                    result.extend(prepend_varint_width(&predicate.serialize()))
                }
            },
        }
        return result;
    }

    pub fn parse(data: &[u8]) -> PredicateResult<(Self, usize)> {
        fn get_path_and_content(data: &[u8]) -> PredicateResult<(FieldPath, UnitContent, usize)> {
            let (path_bytes, path_offset) = extract_data_with_varint_width(data)?;
            let path = FieldPath::parse(path_bytes)?;
            let (content_bytes, content_offset) =
                extract_data_with_varint_width(&data[path_offset..])?;
            let (content, _) = UnitContent::parse(content_bytes)?;
            let total_offset = path_offset + content_offset;
            return Ok((path, content, total_offset));
        }

        fn parse_predicates(data: &[u8]) -> PredicateResult<(Vec<Predicate>, usize)> {
            let mut i = 0;
            let mut predicates: Vec<Predicate> = Vec::new();
            while i < data.len() {
                let (predicate_bytes, predicate_offset) =
                    extract_data_with_varint_width(&data[i..])?;
                if predicate_bytes.len() == 0 {
                    break;
                }
                predicates.push(Predicate::parse(predicate_bytes)?.0);
                i += predicate_offset;
            }
            return Ok((predicates, data.len()));
        }

        if data.len() == 0 {
            return Err(PredicateError::InsufficientBytes);
        }
        let main_data = &data[1..];
        if data[0] == PredicatePrefix::PrimitiveEqual as u8 {
            let (path, content, offset) = get_path_and_content(main_data)?;
            return Ok((
                Predicate::Primitive(PrimitivePredicate::Equal(path, content)),
                1 + offset,
            ));
        } else if data[0] == PredicatePrefix::PrimitiveLessThan as u8 {
            let (path, content, offset) = get_path_and_content(main_data)?;
            return Ok((
                Predicate::Primitive(PrimitivePredicate::LessThan(path, content)),
                1 + offset,
            ));
        } else if data[0] == PredicatePrefix::PrimitiveLessThanOrEqual as u8 {
            let (path, content, offset) = get_path_and_content(main_data)?;
            return Ok((
                Predicate::Primitive(PrimitivePredicate::LessThanOrEqual(path, content)),
                1 + offset,
            ));
        } else if data[0] == PredicatePrefix::PrimitiveGreaterThan as u8 {
            let (path, content, offset) = get_path_and_content(main_data)?;
            return Ok((
                Predicate::Primitive(PrimitivePredicate::GreaterThan(path, content)),
                1 + offset,
            ));
        } else if data[0] == PredicatePrefix::PrimitiveGreaterThanOrEqual as u8 {
            let (path, content, offset) = get_path_and_content(main_data)?;
            return Ok((
                Predicate::Primitive(PrimitivePredicate::GreaterThanOrEqual(path, content)),
                1 + offset,
            ));
        } else if data[0] == PredicatePrefix::CompoundNot as u8 {
            let (predicate_bytes, offset) = extract_data_with_varint_width(main_data)?;
            return Ok((
                Predicate::Compound(CompoundPredicate::Not(Box::new(
                    Self::parse(predicate_bytes)?.0,
                ))),
                1 + offset,
            ));
        } else if data[0] == PredicatePrefix::CompoundAnd as u8 {
            let (predicates, offset) = parse_predicates(main_data)?;
            return Ok((
                Predicate::Compound(CompoundPredicate::And(predicates)),
                1 + offset,
            ));
        } else if data[0] == PredicatePrefix::CompoundOr as u8 {
            let (predicates, offset) = parse_predicates(main_data)?;
            return Ok((
                Predicate::Compound(CompoundPredicate::Or(predicates)),
                1 + offset,
            ));
        } else {
            return Err(PredicateError::UnexpectedPrefix(data[0]));
        }
    }

    /// Check whether content satisfies the predicate
    pub fn check(&self, content: &UnitContent) -> bool {
        fn check_primitive(predicate: &PrimitivePredicate, content: &UnitContent) -> bool {
            return match predicate {
                PrimitivePredicate::Equal(path, expected_content) => {
                    let target_content = get_value_at_path(content, path);
                    &target_content == expected_content
                }
                PrimitivePredicate::GreaterThan(path, expected_content) => {
                    let target_content = get_value_at_path(content, path);
                    &target_content > expected_content
                }
                PrimitivePredicate::GreaterThanOrEqual(path, expected_content) => {
                    let target_content = get_value_at_path(content, path);
                    &target_content >= expected_content
                }
                PrimitivePredicate::LessThan(path, expected_content) => {
                    let target_content = get_value_at_path(content, path);
                    &target_content < expected_content
                }
                PrimitivePredicate::LessThanOrEqual(path, expected_content) => {
                    let target_content = get_value_at_path(content, path);
                    &target_content <= expected_content
                }
            };
        }

        fn check_compound(predicate: &CompoundPredicate, content: &UnitContent) -> bool {
            match predicate {
                CompoundPredicate::Not(predicate) => !predicate.check(content),
                CompoundPredicate::Or(predicates) => {
                    predicates.iter().any(|predicate| predicate.check(content))
                }
                CompoundPredicate::And(predicates) => {
                    predicates.iter().all(|predicate| predicate.check(content))
                }
            }
        }

        match self {
            Predicate::Compound(predicate) => check_compound(predicate, content),
            Predicate::Primitive(predicate) => check_primitive(predicate, content),
        }
    }

    fn parse_tokens(tokens: &[Token]) -> PredicateResult<Predicate> {
        /// Separate a binary predicate like "this EQUAL 1 OR this EQUAL 2" into two predicates to
        /// be joined into a compound predicate.
        /// Parameter i is the location of the logical operator.
        fn parse_binary_predicates(
            tokens: &[Token],
            i: usize,
        ) -> PredicateResult<(Predicate, Predicate)> {
            let left = &tokens[0..i];
            let right = &tokens[i + 1..];
            let left_predicate = Predicate::parse_tokens(left)?;
            let right_predicate = Predicate::parse_tokens(right)?;
            return Ok((left_predicate, right_predicate));
        }

        fn parse_primitive_predicate(tokens: &[Token]) -> PredicateResult<Predicate> {
            let mut field_path = FieldPath::new();
            let mut content: Option<UnitContent> = None;
            enum Relations {
                GreaterThan,
                GreaterThanOrEqual,
                LessThan,
                LessThanOrEqual,
                Equal,
            }
            let mut relations: Option<Relations> = None;
            for token in tokens {
                match token {
                    Token::This => {}
                    Token::Dot => {}
                    Token::Identifier(s) => field_path.push(s),
                    Token::ContentString(s) => {
                        content = Some(UnitContent::from(s.as_str()));
                    }
                    Token::GreaterThan => {
                        relations = Some(Relations::GreaterThan);
                    }
                    Token::GreaterThanOrEqual => {
                        relations = Some(Relations::GreaterThanOrEqual);
                    }
                    Token::LessThan => {
                        relations = Some(Relations::LessThan);
                    }
                    Token::LessThanOrEqual => {
                        relations = Some(Relations::LessThanOrEqual);
                    }
                    Token::Equal => {
                        relations = Some(Relations::Equal);
                    }
                    _ => {
                        return Err(PredicateError::UnexpectedToken);
                    }
                }
            }
            if let Some(content) = content {
                if let Some(relations) = relations {
                    let primitive = match relations {
                        Relations::LessThan => PrimitivePredicate::LessThan(field_path, content),
                        Relations::GreaterThan => {
                            PrimitivePredicate::GreaterThan(field_path, content)
                        }
                        Relations::GreaterThanOrEqual => {
                            PrimitivePredicate::GreaterThanOrEqual(field_path, content)
                        }
                        Relations::LessThanOrEqual => {
                            PrimitivePredicate::LessThanOrEqual(field_path, content)
                        }
                        Relations::Equal => PrimitivePredicate::Equal(field_path, content),
                    };
                    return Ok(Predicate::Primitive(primitive));
                }
            }
            return Err(PredicateError::MalformedTokens);
        }

        let or_index = tokens.iter().position(|token| token == &Token::Or);
        if let Some(i) = or_index {
            let (left, right) = parse_binary_predicates(tokens, i)?;
            let compound = Predicate::Compound(CompoundPredicate::Or(vec![left, right]));
            return Ok(compound);
        }
        let and_index = tokens.iter().position(|token| token == &Token::And);
        if let Some(i) = and_index {
            let (left, right) = parse_binary_predicates(tokens, i)?;
            let compound = Predicate::Compound(CompoundPredicate::And(vec![left, right]));
            return Ok(compound);
        } else {
            parse_primitive_predicate(tokens)
        }
    }
}

#[derive(Debug)]
enum TokenizerState {
    ParsingIdentifier,
    ParsingValue,
}

/// Check whether chars starts with target at position start
/// e.g. chars = [a b c d], start = 2, target = [c, d], then it would return true.
fn starts_with_at(chars: &[char], start: usize, target: &str) -> bool {
    let size = target.len();
    if start + size < chars.len() {
        for (i, char) in target.chars().into_iter().enumerate() {
            if chars[start + i] != char {
                return false;
            }
        }
        return true;
    } else {
        return false;
    }
}

/// Group relevant characters together.
/// For example "this.a==1" is tokenized as ["this", ".", "a", "==", 1] (forming "this" and "==").
/// No semantic checks are done at this stage and the tokens could make no sense.
fn tokenize(str: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut state = TokenizerState::ParsingIdentifier;

    let mut identifier = String::new();
    let mut content_str = String::new();

    let mut index = 0;
    let chars: Vec<_> = str.chars().collect();

    let mut active_quote_mark: Option<&str> = None;

    while index < chars.len() {
        let char = chars[index];
        match state {
            TokenizerState::ParsingIdentifier => {
                let s = char.to_string();
                match s.as_str() {
                    "." => {
                        state = TokenizerState::ParsingIdentifier;
                        if identifier.len() > 0 {
                            tokens.push(Token::Identifier(identifier.clone()));
                            identifier.clear();
                        }
                        tokens.push(Token::Dot);
                        index += 1;
                    }
                    ">" => {
                        state = TokenizerState::ParsingValue;
                        tokens.push(Token::Identifier(identifier.clone()));
                        identifier.clear();
                        if starts_with_at(&chars, index, ">=") {
                            tokens.push(Token::GreaterThanOrEqual);
                            index += 2;
                        } else {
                            tokens.push(Token::GreaterThan);
                            index += 1;
                        };
                    }
                    "<" => {
                        state = TokenizerState::ParsingValue;
                        tokens.push(Token::Identifier(identifier.clone()));
                        identifier.clear();
                        if starts_with_at(&chars, index, "<=") {
                            tokens.push(Token::LessThanOrEqual);
                            index += 2;
                        } else {
                            tokens.push(Token::LessThan);
                            index += 1;
                        };
                    }
                    "=" => {
                        state = TokenizerState::ParsingValue;
                        tokens.push(Token::Identifier(identifier.clone()));
                        identifier.clear();
                        tokens.push(Token::Equal);
                        if starts_with_at(&chars, index, "==") {
                            index += 2;
                        } else {
                            index += 1;
                        };
                    }
                    "|" => {
                        if identifier.len() > 0 {
                            tokens.push(Token::Identifier(identifier.clone()));
                            identifier.clear();
                        }
                        tokens.push(Token::Or);
                        if starts_with_at(&chars, index, "||") {
                            index += 2;
                        } else {
                            index += 1;
                        };
                    }
                    "&" => {
                        if identifier.len() > 0 {
                            tokens.push(Token::Identifier(identifier.clone()));
                            identifier.clear();
                        }
                        tokens.push(Token::And);
                        if starts_with_at(&chars, index, "&&") {
                            index += 2;
                        } else {
                            index += 1;
                        };
                    }
                    " " => {
                        index += 1;
                    }
                    "!" => {
                        if identifier.len() > 0 {
                            tokens.push(Token::Identifier(identifier.clone()));
                            identifier.clear();
                        }
                        if starts_with_at(&chars, index, "!=") {
                            tokens.push(Token::NotEqual);
                            index += 2;
                            state = TokenizerState::ParsingValue;
                        } else {
                            tokens.push(Token::Not);
                            index += 1;
                        };
                    }
                    _ => {
                        let offset = FIELD_PATH_SELF_TOKEN.len();
                        if starts_with_at(&chars, index, FIELD_PATH_SELF_TOKEN) {
                            tokens.push(Token::This);
                            index += offset;
                        } else {
                            identifier.push(char);
                            index += 1;
                        }
                    }
                }
            }
            TokenizerState::ParsingValue => {
                let s = char.to_string();
                match s.as_str() {
                    "|" | "&" | "!" => {
                        if content_str.len() > 0 {
                            tokens.push(ContentString(content_str.clone()));
                            content_str.clear();
                        }
                        state = TokenizerState::ParsingIdentifier
                    }
                    "\"" => {
                        content_str.push(char);
                        index += 1;
                        if let Some(quote) = active_quote_mark {
                            if quote == "\"" {
                                active_quote_mark = None;
                            }
                        } else {
                            active_quote_mark = Some("\"");
                        }
                    }
                    "'" => {
                        content_str.push(char);
                        index += 1;
                        if let Some(quote) = active_quote_mark {
                            if quote == "\'" {
                                active_quote_mark = None;
                            }
                        } else {
                            active_quote_mark = Some("\'");
                        }
                    }
                    " " => {
                        if active_quote_mark.is_some() {
                            content_str.push(char);
                        }
                        index += 1;
                    }
                    _ => {
                        content_str.push(char);
                        index += 1;
                    }
                }
            }
        }
    }
    tokens.push(Token::ContentString(content_str));
    return tokens;
}

/// Reach into a Map-based UnitContent and get data at a path, for instance, this.data.subfield
fn get_value_at_path(content: &UnitContent, path: &FieldPath) -> UnitContent {
    let (first, remainder) = path.shift();
    if let Some(first) = first {
        if let UnitContent::Map(map) = content {
            if let Some(value) = map.get(&first) {
                return get_value_at_path(value, &remainder);
            } else {
                return UnitContent::Nil;
            }
        } else {
            return UnitContent::Nil;
        }
    } else {
        return content.clone();
    }
}

#[cfg(test)]
mod predicate_tests {
    use crate::storage::executor::predicate::{
        tokenize, CompoundPredicate, FieldPath, Predicate, PrimitivePredicate, Token,
    };
    use crate::storage::executor::unit_content::UnitContent;
    use immuxsys_dev_utils::dev_utils::{get_phone_mode_test_predicates, get_phone_model_fixture};
    use std::collections::HashMap;

    #[test]
    fn test_tokenizer() {
        let fixture = vec![
            // Simple
            (
                "this.hello>1",
                vec![
                    Token::This,
                    Token::Dot,
                    Token::Identifier(String::from("hello")),
                    Token::GreaterThan,
                    Token::ContentString(String::from("1")),
                ],
            ),
            // Simple with spaces
            (
                "this. hello > 1",
                vec![
                    Token::This,
                    Token::Dot,
                    Token::Identifier(String::from("hello")),
                    Token::GreaterThan,
                    Token::ContentString(String::from("1")),
                ],
            ),
            // String parsing
            (
                "this.str != \"hello\" && this.c == '世界' && this.spaces = '   '",
                vec![
                    Token::This,
                    Token::Dot,
                    Token::Identifier(String::from("str")),
                    Token::NotEqual,
                    Token::ContentString(String::from("\"hello\"")),
                    Token::And,
                    Token::This,
                    Token::Dot,
                    Token::Identifier(String::from("c")),
                    Token::Equal,
                    Token::ContentString(String::from("'世界'")),
                    Token::And,
                    Token::This,
                    Token::Dot,
                    Token::Identifier(String::from("spaces")),
                    Token::Equal,
                    Token::ContentString(String::from("\'   \'")),
                ],
            ),
            // Complex
            (
                "this.A=='wow'||this.B>=1&&this.C<1",
                vec![
                    Token::This,
                    Token::Dot,
                    Token::Identifier(String::from("A")),
                    Token::Equal,
                    Token::ContentString(String::from("'wow'")),
                    Token::Or,
                    Token::This,
                    Token::Dot,
                    Token::Identifier(String::from("B")),
                    Token::GreaterThanOrEqual,
                    Token::ContentString(String::from("1")),
                    Token::And,
                    Token::This,
                    Token::Dot,
                    Token::Identifier(String::from("C")),
                    Token::LessThan,
                    Token::ContentString(String::from("1")),
                ],
            ),
        ];
        assert!(fixture.len() > 0);
        for (str, expected) in fixture {
            let tokenized = tokenize(str);
            assert_eq!(tokenized, expected)
        }
    }

    #[test]
    fn test_parse_str() {
        let fixture = vec![
            (
                "this.hello>1",
                Predicate::Primitive(PrimitivePredicate::GreaterThan(
                    FieldPath::from(vec![String::from("hello")]),
                    UnitContent::Float64(1.0),
                )),
                true,
            ),
            (
                "this.hello.world>1||this.name==\"world\"",
                Predicate::Compound(CompoundPredicate::Or(vec![
                    Predicate::Primitive(PrimitivePredicate::GreaterThan(
                        FieldPath::from(vec![String::from("hello"), String::from("world")]),
                        UnitContent::Float64(1.0),
                    )),
                    Predicate::Primitive(PrimitivePredicate::Equal(
                        FieldPath::from(vec![String::from("name")]),
                        UnitContent::String(String::from("world")),
                    )),
                ])),
                true,
            ),
        ];

        assert!(fixture.len() > 0);

        for (str, predicate, reversible) in fixture {
            let parsed = Predicate::parse_str(str).unwrap();
            assert_eq!(parsed, predicate);
            if reversible {
                let stringified = format!("{}", predicate);
                assert_eq!(str, stringified);
            }
        }
    }

    #[test]
    fn test_byte_serialization() {
        let fixture: Vec<(Predicate, Vec<u8>)> = vec![
            (
                Predicate::Primitive(PrimitivePredicate::Equal(
                    FieldPath::from(vec![String::from("hello")]),
                    UnitContent::String(String::from("data")),
                )),
                vec![
                    0x00u8, // Prefix(Equal)
                    // FieldPath
                    0x06, // path width
                    0x05, // width of "hello"
                    0x68, 0x65, 0x6c, 0x6c, 0x6f, // "hello"
                    // UnitContent
                    0x06, // content width
                    0x10, // Prefix(String)
                    0x04, // width of "data"
                    0x64, 0x61, 0x74, 0x61, // "data"
                ],
            ),
            // Complex predicate
            // this.name == "han" && !(this.alive == true)
            (
                Predicate::Compound(CompoundPredicate::And(vec![
                    Predicate::Primitive(PrimitivePredicate::Equal(
                        FieldPath::from(vec![String::from("name")]),
                        UnitContent::String(String::from("han")),
                    )),
                    Predicate::Compound(CompoundPredicate::Not(Box::new(Predicate::Primitive(
                        PrimitivePredicate::Equal(
                            FieldPath::from(vec![String::from("alive")]),
                            UnitContent::Bool(true),
                        ),
                    )))),
                ])),
                vec![
                    //
                    // Left branch
                    //
                    0xA0, // And
                    // this.name == "han"
                    0x0d, // predicate width
                    0x00, // Prefix(Equal)
                    // FieldPath
                    0x05, // path width
                    0x04, // width of "name"
                    0x6e, 0x61, 0x6d, 0x65, // "name"
                    // UnitContent
                    0x05, // content width
                    0x10, // Prefix(String)
                    0x03, // width of "han"
                    0x68, 0x61, 0x6e, // "han"
                    //
                    // Right branch
                    //
                    // Not
                    0x0d, // predicate width
                    0xA2, // Prefix(Not)
                    // this.alive == true
                    0x0b, // predicate width
                    0x00, // Prefix(Equal)
                    // FieldPath
                    0x06, // path width
                    0x05, // width of "alive"
                    0x61, 0x6c, 0x69, 0x76, 0x65, // "alive"
                    // UnitContent
                    0x02, // content width
                    0x11, // Prefix(Bool)
                    0x01, // true
                ],
            ),
        ];

        assert!(fixture.len() > 0);

        for (predicate, bytes) in fixture {
            let serialized = predicate.serialize();
            assert_eq!(serialized, bytes);

            let (parsed, _) = Predicate::parse(&bytes).unwrap();
            assert_eq!(parsed, predicate);
        }
    }

    #[test]
    fn test_satisfy_simple_predicate() {
        // predict this.x=1.0
        let predicate = Predicate::Primitive(PrimitivePredicate::Equal(
            FieldPath::from(vec![String::from("x")]),
            UnitContent::Float64(1.0),
        ));
        let mut data = HashMap::new();
        data.insert(String::from("x"), UnitContent::Float64(1.0));
        let content = UnitContent::Map(data);
        assert!(predicate.check(&content));
    }

    #[test]
    fn test_satisfy_simple_predicate_reject() {
        // predict this.x=2.0, when this.x==1.0
        let predicate = Predicate::Primitive(PrimitivePredicate::Equal(
            FieldPath::from(vec![String::from("x")]),
            UnitContent::Float64(2.0),
        ));
        let mut data = HashMap::new();
        data.insert(String::from("x"), UnitContent::Float64(1.0));
        let content = UnitContent::Map(data);
        assert!(!predicate.check(&content));
    }

    #[test]
    fn test_whole_workflow() {
        let str = "this.x==1&&this.y==2";
        let predicate = Predicate::parse_str(str).unwrap();

        let mut data_correct = HashMap::new();
        data_correct.insert(String::from("x"), UnitContent::Float64(1.0));
        data_correct.insert(String::from("y"), UnitContent::Float64(2.0));
        let content_correct = UnitContent::Map(data_correct);
        assert!(predicate.check(&content_correct));

        let mut data_wrong = HashMap::new();
        data_wrong.insert(String::from("x"), UnitContent::String(String::from("1")));
        data_wrong.insert(String::from("y"), UnitContent::Float64(2.0));
        let content_wrong = UnitContent::Map(data_wrong);
        assert!(!predicate.check(&content_wrong));
    }

    #[test]
    fn test_predicate_checking() {
        let (expected_satisfied_contents, unsatisfied_contents) = get_phone_model_fixture();

        let mut contents = vec![];
        contents.extend_from_slice(&expected_satisfied_contents);
        contents.extend_from_slice(&unsatisfied_contents);

        let predicate = get_phone_mode_test_predicates();

        for content in expected_satisfied_contents {
            assert!(predicate.check(&content))
        }
        for content in unsatisfied_contents {
            assert!(!predicate.check(&content))
        }
    }
}
