use crate::utils::{
    u16_to_u8_array, u32_to_u8_array, u64_to_u8_array, u8_array_to_u16, u8_array_to_u32,
    u8_array_to_u64,
};

/// Variable-length integer encoding, using simplistic Bitcoin's varint design:
/// https://bitcointalk.org/index.php?topic=32849.msg410480#msg410480

const VARINT_16BIT_PREFIX: u8 = 0xfd;
const VARINT_32BIT_PREFIX: u8 = 0xfe;
const VARINT_64BIT_PREFIX: u8 = 0xff;

#[derive(Debug)]
pub enum VarIntError {
    UnexpectedFormat,
}

pub fn varint_decode(data: &[u8]) -> Result<(u64, usize), VarIntError> {
    match data.get(0) {
        None => return Err(VarIntError::UnexpectedFormat),
        Some(&VARINT_16BIT_PREFIX) => {
            if data.len() < 3 {
                return Err(VarIntError::UnexpectedFormat);
            } else {
                let array = [data[1], data[2]];
                return Ok((u8_array_to_u16(&array) as u64, 3));
            }
        }
        Some(&VARINT_32BIT_PREFIX) => {
            if data.len() < 5 {
                return Err(VarIntError::UnexpectedFormat);
            } else {
                let array = [data[1], data[2], data[3], data[4]];
                return Ok((u8_array_to_u32(&array) as u64, 5));
            }
        }
        Some(&VARINT_64BIT_PREFIX) => {
            if data.len() < 9 {
                return Err(VarIntError::UnexpectedFormat);
            } else {
                let array = [
                    data[1], data[2], data[3], data[4], data[5], data[6], data[7], data[8],
                ];
                return Ok((u8_array_to_u64(&array), 9));
            }
        }
        Some(byte) => return Ok((*byte as u64, 1)),
    }
}

pub fn varint_encode(i: u64) -> Vec<u8> {
    if i < VARINT_16BIT_PREFIX as u64 {
        return vec![i as u8];
    } else if i <= std::u16::MAX as u64 {
        let mut result = vec![VARINT_16BIT_PREFIX];
        result.extend_from_slice(&u16_to_u8_array(i as u16));
        result
    } else if i <= std::u32::MAX as u64 {
        let mut result = vec![VARINT_32BIT_PREFIX];
        result.extend_from_slice(&u32_to_u8_array(i as u32));
        result
    } else {
        let mut result = vec![VARINT_64BIT_PREFIX];
        result.extend_from_slice(&u64_to_u8_array(i));
        result
    }
}

#[cfg(test)]
mod varint_utils_tests {
    use crate::varint::{varint_decode, varint_encode};

    #[test]
    fn test_varint_8bit_encode() {
        let u = 0xfc;
        let encoding = varint_encode(u);
        assert_eq!(encoding, [0xfc])
    }

    #[test]
    fn test_varint_8bit_decode() {
        let data = [0xfc];
        let (u, _) = varint_decode(&data).unwrap();
        assert_eq!(u, 0xfc)
    }

    #[test]
    fn test_varint_16bit_encode() {
        let u = 0xff00;
        let encoding = varint_encode(u);
        assert_eq!(encoding, [0xfd, 0x00, 0xff])
    }

    #[test]
    fn test_varint_16bit_decode() {
        let data = [0xfd, 0x11, 0x22];
        let (u, _) = varint_decode(&data).unwrap();
        assert_eq!(u, 0x2211)
    }

    #[test]
    #[should_panic]
    fn test_varint_16bit_decode_malformed() {
        varint_decode(&[0xfd, 0x00]).unwrap();
    }

    #[test]
    fn test_varint_32bit_encode() {
        let u = 0xaabbccdd;
        let encoding = varint_encode(u);
        assert_eq!(encoding, [0xfe, 0xdd, 0xcc, 0xbb, 0xaa])
    }

    #[test]
    fn test_varint_32bit_decode() {
        let data = [0xfe, 0x00, 0x11, 0x22, 0x33];
        let (u, _) = varint_decode(&data).unwrap();
        assert_eq!(u, 0x33221100)
    }

    #[test]
    #[should_panic]
    fn test_varint_32bit_decode_malformed() {
        varint_decode(&[0xfe, 0x00, 0x00, 0x00]).unwrap();
    }

    #[test]
    fn test_varint_64bit_encode() {
        let u = 0x1122334455667788;
        let encoding = varint_encode(u);
        assert_eq!(
            encoding,
            [0xff, 0x88, 0x77, 0x66, 0x55, 0x44, 0x33, 0x22, 0x11]
        )
    }

    #[test]
    fn test_varint_64bit_decode() {
        let data = [0xff, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07];
        let (u, _) = varint_decode(&data).unwrap();
        assert_eq!(u, 0x0706050403020100)
    }

    #[test]
    #[should_panic]
    fn test_varint_64bit_decode_malformed() {
        varint_decode(&[0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]).unwrap();
    }

    #[test]
    fn spot_check_varint_reversibility() {
        let mut i = std::u64::MAX;
        while i > 1 {
            let (u, _) = varint_decode(&varint_encode(i)).unwrap();
            assert_eq!(u, i);
            i /= 2;
        }
    }

    #[test]
    fn test_varint_boundary_cases() {
        let cases: Vec<(u64, Vec<u8>)> = vec![
            (0xfc, vec![0xfc]),
            (0xfd, vec![0xfd, 0xfd, 0x00]),
            (0xfe, vec![0xfd, 0xfe, 0x00]),
            (0xff, vec![0xfd, 0xff, 0x00]),
            (0xffff, vec![0xfd, 0xff, 0xff]),
            (0x00010000, vec![0xfe, 0x00, 0x00, 0x01, 0x00]),
            (0xffffffff, vec![0xfe, 0xff, 0xff, 0xff, 0xff]),
            (
                0x0100000000,
                vec![0xff, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00],
            ),
        ];

        for (u, v) in cases {
            assert_eq!(varint_encode(u), v)
        }
    }
}
