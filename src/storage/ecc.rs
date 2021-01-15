use std::fmt;

use crate::utils::ints::{u64_to_u8_array, u8_array_to_u64};

use crate::utils::ints::{get_bit_u8, set_bit_u8};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ECCMode {
    Identity = 0x00, // No ECC
    TMR = 0x01,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorCorrectionError {
    DataWidthNotDivisibleByModulus(usize),
    ParseErrorCorrectionErrorError,
}

pub enum ErrorCorrectionErrorPrefix {
    DataWidthNotDivisibleByModulus = 0x01,
    ParseErrorCorrectionErrorError = 0x02,
}

impl ErrorCorrectionError {
    pub fn marshal(&self) -> Vec<u8> {
        match self {
            ErrorCorrectionError::DataWidthNotDivisibleByModulus(size) => {
                let mut result =
                    vec![ErrorCorrectionErrorPrefix::DataWidthNotDivisibleByModulus as u8];
                result.extend_from_slice(&u64_to_u8_array(*size as u64));
                return result;
            }
            ErrorCorrectionError::ParseErrorCorrectionErrorError => {
                vec![ErrorCorrectionErrorPrefix::ParseErrorCorrectionErrorError as u8]
            }
        }
    }

    pub fn parse(data: &[u8]) -> Result<(ErrorCorrectionError, usize), ErrorCorrectionError> {
        let mut position = 0;
        let prefix = data[position];
        position += 1;

        if prefix == ErrorCorrectionErrorPrefix::DataWidthNotDivisibleByModulus as u8 {
            let size = u8_array_to_u64(&[
                data[position],
                data[position + 1],
                data[position + 2],
                data[position + 3],
                data[position + 4],
                data[position + 5],
                data[position + 6],
                data[position + 7],
            ]);
            position += 8;
            Ok((
                ErrorCorrectionError::DataWidthNotDivisibleByModulus(size as usize),
                position,
            ))
        } else {
            Ok((
                ErrorCorrectionError::ParseErrorCorrectionErrorError,
                position,
            ))
        }
    }
}

impl fmt::Display for ErrorCorrectionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorCorrectionError::DataWidthNotDivisibleByModulus(size) => write!(
                f,
                "{}::{}",
                "ErrorCorrectionError::DataWidthNotDivisibleByModulus", size
            ),
            ErrorCorrectionError::ParseErrorCorrectionErrorError => write!(
                f,
                "{}",
                "ErrorCorrectionError::ParseErrorCorrectionErrorError",
            ),
        }
    }
}

pub trait ErrorCorrectionCodec {
    fn encode(&self, data: &[u8]) -> Vec<u8>;
    fn decode(&self, data: &[u8]) -> Result<Vec<u8>, ErrorCorrectionError>;
}

// A code that passes data though without any processing
pub struct IdentityCode {}

impl IdentityCode {
    pub fn new() -> Self {
        Self {}
    }
}

impl ErrorCorrectionCodec for IdentityCode {
    fn encode(&self, data: &[u8]) -> Vec<u8> {
        return data.to_vec();
    }
    fn decode(&self, data: &[u8]) -> Result<Vec<u8>, ErrorCorrectionError> {
        return Ok(data.to_vec());
    }
}

// Triple modular redundancy, see https://en.wikipedia.org/wiki/Triple_modular_redundancy
pub struct TripleRedundancyCode {}

impl TripleRedundancyCode {
    pub fn new() -> Self {
        Self {}
    }
}

impl ErrorCorrectionCodec for TripleRedundancyCode {
    fn encode(&self, data: &[u8]) -> Vec<u8> {
        let mut result = Vec::with_capacity(data.len() * 3);
        for _ in 0..3 {
            result.extend(data);
        }
        return result;
    }
    fn decode(&self, data: &[u8]) -> Result<Vec<u8>, ErrorCorrectionError> {
        if data.len() % 3 != 0 {
            return Err(ErrorCorrectionError::DataWidthNotDivisibleByModulus(
                data.len(),
            ));
        }
        let mut i = 0;
        let original_width = data.len() / 3;
        let mut result = Vec::with_capacity(original_width);
        while i < original_width {
            let pos_1 = i;
            let pos_2 = i + original_width;
            let pos_3 = i + original_width * 2;
            let byte: u8 = if data[pos_1] == data[pos_2] && data[pos_2] == data[pos_3] {
                data[pos_1]
            } else {
                let mut output_byte = 0;
                for digit in 0..8 {
                    let bit1 = get_bit_u8(data[pos_1], digit);
                    let bit2 = get_bit_u8(data[pos_2], digit);
                    let bit3 = get_bit_u8(data[pos_3], digit);
                    let bit_sum: u8 = bit1 as u8 + bit2 as u8 + bit3 as u8;
                    let output_bit = if bit_sum >= 2 { true } else { false };
                    set_bit_u8(&mut output_byte, digit, output_bit);
                }
                output_byte
            };
            result.push(byte);
            i += 1;
        }
        return Ok(result);
    }
}

#[cfg(test)]
mod ecc_error_tests {
    use immuxsys_dev_utils::dev_utils::{get_error_correction_errors, ErrorCorrectionError};

    #[test]
    fn test_error_reversibility() {
        let expected_errors = get_error_correction_errors();

        for error in expected_errors {
            let error_bytes = error.marshal();
            let (actual_error, _) = ErrorCorrectionError::parse(&error_bytes).unwrap();
            assert_eq!(error, actual_error);
        }
    }
}

#[cfg(test)]
mod ecc_identity_tests {
    use crate::storage::ecc::{ErrorCorrectionCodec, IdentityCode};

    #[test]
    fn test_identity_encode() {
        let input = [1, 20, 200];
        let codec = IdentityCode::new();
        let encoded = codec.encode(&input);
        assert_eq!(encoded, input)
    }

    #[test]
    fn test_tmr_decode() {
        let input = [1, 20, 200];
        let codec = IdentityCode::new();
        let decoded = codec.decode(&input).unwrap();
        assert_eq!(decoded, input)
    }
}

#[cfg(test)]
mod ecc_tmr_tests {
    use crate::storage::ecc::{ErrorCorrectionCodec, TripleRedundancyCode};

    #[test]
    fn test_tmr_encode() {
        let data = [1, 20, 200];
        let codec = TripleRedundancyCode::new();
        let encoded = codec.encode(&data);
        assert_eq!(encoded, [1, 20, 200, 1, 20, 200, 1, 20, 200])
    }

    #[test]
    #[rustfmt::skip]
    fn test_tmr_decode() {
        let table: Vec<(Vec<u8>, Vec<u8>)> = vec![
            (
                vec![
                    0x11, 0x55, 0xff, 0x42,
                    0x11, 0x55, 0xff, 0x42,
                    0x11, 0x55, 0xff, 0x42,
                ],
                vec![0x11, 0x55, 0xff, 0x42],
            ),
            (
                vec![
                    0x00, 0x55, 0xff, 0x42,
                    0x11, 0x55, 0x00, 0x42,
                    0x11, 0x00, 0xff, 0x42,
                ],
                vec![0x11, 0x55, 0xff, 0x42],
            ),
        ];
        let codec = TripleRedundancyCode::new();
        for row in table {
            let (input, expected_original) = row;
            let decoded = codec.decode(&input).unwrap();
            assert_eq!(decoded, expected_original)
        }
    }

    #[test]
    #[should_panic]
    // TMR encoded data should has a width divisible by 3
    fn test_tmr_incorrect_width() {
        let codec = TripleRedundancyCode::new();
        codec.decode(&[1, 2, 3, 4]).unwrap();
    }

    #[test]
    fn test_tmr_codec_reversibility() {
        for i in 0..10 {
            let input: Vec<u8> = [1, 2, 3].repeat(i).iter().map(|n| n * i as u8).collect();
            let codec = TripleRedundancyCode::new();
            let output = codec.encode(&input);
            let input_recovered = codec.decode(&output).unwrap();
            assert_eq!(input, input_recovered);
        }
    }

    #[test]
    fn test_tar_resistance_to_single_corruption() {
        let input = vec![0, 1, 2, 3, 255];
        let codec = TripleRedundancyCode::new();
        let output = codec.encode(&input);

        // Corrupt every bit one by one and verify correct output
        for corrupt_position in 0..output.len() {
            for corrupt_value in 0..255 {
                let mut corrupted_output = output.clone();
                corrupted_output[corrupt_position] = corrupt_value; // Corruption
                let input_recovered = codec.decode(&corrupted_output).unwrap();
                assert_eq!(input, input_recovered);
            }
        }
    }
}
