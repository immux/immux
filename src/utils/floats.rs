use crate::utils::ints::{u64_to_u8_array, u8_array_to_u64};

pub fn f64_to_u8_array(f: f64) -> [u8; 8] {
    return u64_to_u8_array(f.to_bits());
}

pub fn u8_array_to_f64(data: &[u8; 8]) -> f64 {
    return f64::from_bits(u8_array_to_u64(data));
}

#[cfg(test)]
mod float_utils_test {
    use crate::utils::floats::{f64_to_u8_array, u8_array_to_f64};

    #[test]
    fn spot_check_f64_array_reversibility() {
        fn is_reversible(f: f64) -> bool {
            let bytes = f64_to_u8_array(f);
            let f_parsed = u8_array_to_f64(&[bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7]]);
            return f == f_parsed;
        }

        // large numbers
        {
            let mut f = 0.0;
            while f < 1e20 {
                assert!(is_reversible(f));
                f += 1.1e6;
                f *= -1.3;
            }
        }

        // small numbers
        {
            let mut f = 1e-10;
            while f < 1.0 {
                assert!(is_reversible(f));
                f *= -1.9;
            }
        }
    }
}
