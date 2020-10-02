// u128
pub fn u128_to_u8_array(x: u128) -> [u8; 16] {
    let b15 = ((x >> 120) & 0xff) as u8;
    let b14 = ((x >> 112) & 0xff) as u8;
    let b13 = ((x >> 104) & 0xff) as u8;
    let b12 = ((x >> 96) & 0xff) as u8;
    let b11 = ((x >> 88) & 0xff) as u8;
    let b10 = ((x >> 80) & 0xff) as u8;
    let b9 = ((x >> 72) & 0xff) as u8;
    let b8 = ((x >> 64) & 0xff) as u8;
    let b7 = ((x >> 56) & 0xff) as u8;
    let b6 = ((x >> 48) & 0xff) as u8;
    let b5 = ((x >> 40) & 0xff) as u8;
    let b4 = ((x >> 32) & 0xff) as u8;
    let b3 = ((x >> 24) & 0xff) as u8;
    let b2 = ((x >> 16) & 0xff) as u8;
    let b1 = ((x >> 8) & 0xff) as u8;
    let b0 = ((x >> 0) & 0xff) as u8;

    [
        b0, b1, b2, b3, b4, b5, b6, b7, b8, b9, b10, b11, b12, b13, b14, b15,
    ]
}

pub fn u8_array_to_u128(data: &[u8; 16]) -> u128 {
    (((data[0] as u128) << 0)
        + ((data[1] as u128) << 8)
        + ((data[2] as u128) << 16)
        + ((data[3] as u128) << 24)
        + ((data[4] as u128) << 32)
        + ((data[5] as u128) << 40)
        + ((data[6] as u128) << 48)
        + ((data[7] as u128) << 56)
        + ((data[8] as u128) << 64)
        + ((data[9] as u128) << 72)
        + ((data[10] as u128) << 80)
        + ((data[11] as u128) << 88)
        + ((data[12] as u128) << 96)
        + ((data[13] as u128) << 104)
        + ((data[14] as u128) << 112)
        + ((data[15] as u128) << 120))
        .into()
}

// u64

pub fn u64_to_u8_array(x: u64) -> [u8; 8] {
    let b7 = ((x >> 56) & 0xff) as u8;
    let b6 = ((x >> 48) & 0xff) as u8;
    let b5 = ((x >> 40) & 0xff) as u8;
    let b4 = ((x >> 32) & 0xff) as u8;
    let b3 = ((x >> 24) & 0xff) as u8;
    let b2 = ((x >> 16) & 0xff) as u8;
    let b1 = ((x >> 8) & 0xff) as u8;
    let b0 = ((x >> 0) & 0xff) as u8;

    [b0, b1, b2, b3, b4, b5, b6, b7]
}

pub fn u8_array_to_u64(data: &[u8; 8]) -> u64 {
    (((data[0] as u64) << 0)
        + ((data[1] as u64) << 8)
        + ((data[2] as u64) << 16)
        + ((data[3] as u64) << 24)
        + ((data[4] as u64) << 32)
        + ((data[5] as u64) << 40)
        + ((data[6] as u64) << 48)
        + ((data[7] as u64) << 56))
        .into()
}

pub fn byte_slice_to_u64(data: &[u8]) -> u64 {
    if data.len() >= 8 {
        let p = data;
        u8_array_to_u64(&[p[0], p[1], p[2], p[3], p[4], p[5], p[6], p[7]])
    } else {
        let mut p = [0u8; 8];
        for (pos, i) in data.iter().enumerate() {
            p[pos] = *i;
        }
        u8_array_to_u64(&[p[0], p[1], p[2], p[3], p[4], p[5], p[6], p[7]])
    }
}

// u32

pub fn u8_array_to_u32(data: &[u8; 4]) -> u32 {
    (((data[0] as u32) << 0)
        + ((data[1] as u32) << 8)
        + ((data[2] as u32) << 16)
        + ((data[3] as u32) << 24))
        .into()
}

pub fn byte_slice_to_u32(data: &[u8]) -> u32 {
    if data.len() >= 4 {
        let p = data;
        u8_array_to_u32(&[p[0], p[1], p[2], p[3]])
    } else {
        let mut p = [0u8; 4];
        for (pos, i) in data.iter().enumerate() {
            p[pos] = *i;
        }
        u8_array_to_u32(&[p[0], p[1], p[2], p[3]])
    }
}

pub fn u32_to_u8_array(x: u32) -> [u8; 4] {
    let b3 = ((x >> 24) & 0xff) as u8;
    let b2 = ((x >> 16) & 0xff) as u8;
    let b1 = ((x >> 8) & 0xff) as u8;
    let b0 = ((x >> 0) & 0xff) as u8;

    [b0, b1, b2, b3]
}

pub fn get_bit_u32(input: u32, digit: u8) -> bool {
    if digit < 32 {
        input & (1u32 << (digit as u32)) != 0
    } else {
        false
    }
}

pub fn set_bit_u32(int: &mut u32, digit: u8, value: bool) {
    if value {
        *int |= 1u32 << (digit as u32);
    } else {
        *int &= !(1u32 << (digit as u32));
    }
}

// u16

pub fn u8_array_to_u16(data: &[u8; 2]) -> u16 {
    (((data[0] as u16) << 0) + ((data[1] as u16) << 8)).into()
}

pub fn byte_slice_to_u16(data: &[u8]) -> u16 {
    if data.len() >= 2 {
        let p = data;
        u8_array_to_u16(&[p[0], p[1]])
    } else {
        let mut p = [0u8; 2];
        for (pos, i) in data.iter().enumerate() {
            p[pos] = *i;
        }
        u8_array_to_u16(&[p[0], p[1]])
    }
}

pub fn get_bit_u16(input: u16, digit: u8) -> bool {
    if digit < 16 {
        input & (1u16 << (digit as u16)) != 0
    } else {
        false
    }
}

pub fn set_bit_u16(int: &mut u16, digit: u8, value: bool) {
    if value {
        *int |= 1u16 << (digit as u16);
    } else {
        *int &= !(1u16 << (digit as u16));
    }
}

pub fn u16_to_u8_array(x: u16) -> [u8; 2] {
    let b1 = ((x >> 8) & 0xff) as u8;
    let b0 = ((x >> 0) & 0xff) as u8;

    [b0, b1]
}

// u8

pub fn get_bit_u8(input: u8, digit: u8) -> bool {
    if digit < 8 {
        input & (1u8 << (digit as u8)) != 0
    } else {
        false
    }
}

pub fn set_bit_u8(int: &mut u8, digit: u8, value: bool) {
    if value {
        *int |= 1u8 << (digit as u8);
    } else {
        *int &= !(1u8 << (digit as u8));
    }
}

#[cfg(test)]
mod int_utils_test {
    use crate::utils::ints::{
        byte_slice_to_u16, byte_slice_to_u32, byte_slice_to_u64, get_bit_u16, get_bit_u32,
        get_bit_u8, set_bit_u16, set_bit_u32, set_bit_u8, u128_to_u8_array, u16_to_u8_array,
        u32_to_u8_array, u64_to_u8_array, u8_array_to_u128, u8_array_to_u16, u8_array_to_u32,
        u8_array_to_u64,
    };

    // u128

    #[test]
    fn test_u128_to_u8_array() {
        assert_eq!(
            u128_to_u8_array(0),
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        );
        assert_eq!(
            u128_to_u8_array(std::u64::MAX.into()),
            [255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0]
        );
        assert_eq!(
            u128_to_u8_array(std::u128::MAX),
            [255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,]
        );
    }

    #[test]
    fn test_u8_array_to_u128() {
        assert_eq!(
            u8_array_to_u128(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
            0,
        );
        assert_eq!(
            u8_array_to_u128(&[255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0]),
            std::u64::MAX.into(),
        );
        assert_eq!(
            u8_array_to_u128(&[
                255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
            ]),
            std::u128::MAX,
        );
    }

    // test u64 conversions

    #[test]
    fn test_u64_to_u8_array() {
        assert_eq!(u64_to_u8_array(0), [0, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(
            u64_to_u8_array(std::u32::MAX.into()),
            [255, 255, 255, 255, 0, 0, 0, 0]
        );
        assert_eq!(
            u64_to_u8_array(std::u64::MAX),
            [255, 255, 255, 255, 255, 255, 255, 255]
        );
    }

    #[test]
    fn test_u8_array_to_u64() {
        assert_eq!(0, u8_array_to_u64(&[0, 0, 0, 0, 0, 0, 0, 0]));
        assert_eq!(
            std::u32::MAX as u64,
            u8_array_to_u64(&[255, 255, 255, 255, 0, 0, 0, 0])
        );
        assert_eq!(
            std::u64::MAX,
            u8_array_to_u64(&[255, 255, 255, 255, 255, 255, 255, 255])
        );
    }

    #[test]
    fn test_byte_slices_to_u64() {
        // Normal
        assert_eq!(0, byte_slice_to_u64(&[0, 0, 0, 0, 0, 0, 0, 0]));
        assert_eq!(
            std::u32::MAX as u64,
            byte_slice_to_u64(&[255, 255, 255, 255, 0, 0, 0, 0])
        );
        assert_eq!(
            std::u64::MAX,
            byte_slice_to_u64(&[255, 255, 255, 255, 255, 255, 255, 255])
        );

        // Abnormal
        assert_eq!(0, byte_slice_to_u64(&[]));
        assert_eq!(42, byte_slice_to_u64(&[42]));
        assert_eq!(0x100, byte_slice_to_u64(&[0, 1]));
        assert_eq!(
            std::u64::MAX,
            byte_slice_to_u64(&[255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255])
        );
    }

    #[test]
    fn spot_check_u64_array_reversibility() {
        let large_prime = 67280421310721;
        for i in (0..std::u64::MAX).step_by(large_prime) {
            assert_eq!(u8_array_to_u64(&u64_to_u8_array(i)), i);
            assert_eq!(byte_slice_to_u64(&u64_to_u8_array(i)), i);
        }
    }

    // test u32 conversions

    #[test]
    fn test_u32_to_u8_array() {
        assert_eq!(u32_to_u8_array(0), [0, 0, 0, 0]);
        assert_eq!(u32_to_u8_array(std::u16::MAX.into()), [255, 255, 0, 0]);
        assert_eq!(u32_to_u8_array(std::u32::MAX), [255, 255, 255, 255]);
    }

    #[test]
    fn test_u8_array_to_u32() {
        assert_eq!(0, u8_array_to_u32(&[0, 0, 0, 0]));
        assert_eq!(std::u16::MAX as u32, u8_array_to_u32(&[255, 255, 0, 0]));
        assert_eq!(std::u32::MAX, u8_array_to_u32(&[255, 255, 255, 255]));
    }

    #[test]
    #[test]
    fn test_byte_slices_to_u32() {
        // Normal
        assert_eq!(0, u8_array_to_u32(&[0, 0, 0, 0]));
        assert_eq!(std::u16::MAX as u32, byte_slice_to_u32(&[255, 255, 0, 0]));
        assert_eq!(std::u32::MAX, byte_slice_to_u32(&[255, 255, 255, 255]));

        // Abnormal
        assert_eq!(0, byte_slice_to_u32(&[]));
        assert_eq!(42, byte_slice_to_u32(&[42]));
        assert_eq!(0x100, byte_slice_to_u32(&[0, 1]));
        assert_eq!(
            std::u32::MAX,
            byte_slice_to_u32(&[255, 255, 255, 255, 255, 255, 255])
        );
    }

    #[test]
    fn spot_check_u32_array_reversibility() {
        let large_prime = 6700417;
        for i in (0..std::u32::MAX).step_by(large_prime) {
            assert_eq!(u8_array_to_u32(&u32_to_u8_array(i)), i)
        }
    }

    // test u16 conversions
    #[test]
    fn test_u16_to_u8_array() {
        assert_eq!(u32_to_u8_array(0), [0, 0, 0, 0]);
        assert_eq!(u32_to_u8_array(std::u16::MAX.into()), [255, 255, 0, 0]);
        assert_eq!(u32_to_u8_array(std::u32::MAX), [255, 255, 255, 255]);
    }

    #[test]
    fn test_u8_array_to_u16() {
        assert_eq!(0, u8_array_to_u16(&[0, 0]));
        assert_eq!(std::u8::MAX as u16, u8_array_to_u16(&[255, 0]));
        assert_eq!(std::u16::MAX, u8_array_to_u16(&[255, 255]));
    }

    #[test]
    fn test_byte_slice_to_u16() {
        // Normal
        assert_eq!(0, byte_slice_to_u16(&[0, 0]));
        assert_eq!(std::u16::MAX as u16, byte_slice_to_u16(&[255, 255]));

        // Malformed
        assert_eq!(0, byte_slice_to_u16(&[]));
        assert_eq!(42, byte_slice_to_u16(&[42]));
        assert_eq!(std::u16::MAX, byte_slice_to_u16(&[255, 255, 1]));
    }

    #[test]
    fn spot_check_u16_array_reversibility() {
        let prime = 8191;
        for i in (0..std::u16::MAX).step_by(prime) {
            assert_eq!(u8_array_to_u16(&u16_to_u8_array(i)), i)
        }
    }

    #[test]
    fn test_get_bit_u32() {
        for digit in 0..32 {
            assert_eq!(get_bit_u32(0, digit), false);
            assert_eq!(get_bit_u32(std::u32::MAX, digit), true);
        }
        assert_eq!(get_bit_u32(std::u32::MAX, 100), false);
    }

    #[test]
    fn test_get_bit_u16() {
        for digit in 0..16 {
            assert_eq!(get_bit_u16(0, digit), false);
            assert_eq!(get_bit_u16(std::u16::MAX, digit), true);
        }
        assert_eq!(get_bit_u16(std::u16::MAX, 100), false);
    }

    #[test]
    fn test_get_bit_u8() {
        for digit in 0..8 {
            assert_eq!(get_bit_u8(0, digit), false);
            assert_eq!(get_bit_u8(std::u8::MAX, digit), true);
        }
        assert_eq!(get_bit_u8(std::u8::MAX, 100), false);
    }

    #[test]
    fn test_set_bit_u32() {
        let mut data = 0u32;

        for i in 0..32 {
            set_bit_u32(&mut data, i, true);
        }
        assert_eq!(data, std::u32::MAX);

        for i in 0..32 {
            set_bit_u32(&mut data, i, false);
        }
        assert_eq!(data, 0);
    }

    #[test]
    fn test_set_bit_u16() {
        let mut data = 0u16;

        for i in 0..16 {
            set_bit_u16(&mut data, i, true);
        }
        assert_eq!(data, std::u16::MAX);

        for i in 0..16 {
            set_bit_u16(&mut data, i, false);
        }
        assert_eq!(data, 0);
    }

    #[test]
    fn test_set_bit_u8() {
        let mut data = 0u8;

        for i in 0..8 {
            set_bit_u8(&mut data, i, true);
        }
        assert_eq!(data, std::u8::MAX);

        for i in 0..8 {
            set_bit_u8(&mut data, i, false);
        }
        assert_eq!(data, 0);
    }
}
