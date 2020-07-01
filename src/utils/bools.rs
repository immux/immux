pub fn bool_to_u8(b: bool) -> u8 {
    if b {
        1
    } else {
        0
    }
}

pub fn u8_to_bool(u: u8) -> bool {
    if u == 0 {
        false
    } else {
        true
    }
}

#[cfg(test)]
mod bool_utils_tests {
    use crate::utils::bools::{bool_to_u8, u8_to_bool};

    #[test]
    fn test_bool_to_u8() {
        assert_eq!(bool_to_u8(true), 1);
        assert_eq!(bool_to_u8(false), 0);
    }

    #[test]
    fn test_u8_to_bool() {
        assert_eq!(u8_to_bool(0), false);
        assert_eq!(u8_to_bool(1), true);
        assert_eq!(u8_to_bool(255), true);
    }
}
