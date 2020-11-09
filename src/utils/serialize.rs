use crate::utils::varint::{varint_decode, varint_encode, VarIntError};

pub fn prepend_varint_width(data: &[u8]) -> Vec<u8> {
    let mut result = Vec::new();
    let data_width = data.len();
    let data_width_bytes = varint_encode(data_width as u64);
    result.extend(data_width_bytes);
    result.extend(data);
    result
}

pub fn extract_data_with_varint_width(data: &[u8]) -> Result<(&[u8], usize), VarIntError> {
    let (width, offset) = varint_decode(data)?;
    let data_end = width as usize + offset;
    return Ok((&data[offset..data_end], data_end));
}

#[cfg(test)]
mod varint_helper_tests {
    use crate::utils::serialize::{extract_data_with_varint_width, prepend_varint_width};

    #[test]
    fn test_varint_conversion() {
        let fixture = vec![
            (vec![], vec![0x00], 1, true),
            (
                vec![0xff, 0xff, 0xff],
                vec![0x03, 0xff, 0xff, 0xff],
                4,
                true,
            ),
            (
                vec![0xff, 0xff],
                vec![0x02, 0xff, 0xff, 0x00, 0x00, 0x00],
                3,
                false,
            ),
        ];

        for (data, data_with_header, offset, reversible) in fixture {
            let (extracted, data_offset) =
                extract_data_with_varint_width(&data_with_header).unwrap();
            assert_eq!(offset, data_offset);
            assert_eq!(extracted, data);
            if reversible {
                let prepended = prepend_varint_width(&data);
                assert_eq!(prepended, data_with_header);
            }
        }
    }
}
