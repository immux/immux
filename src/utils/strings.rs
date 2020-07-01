pub fn utf8_to_string(bytes: &[u8]) -> String {
    let result = std::str::from_utf8(bytes);
    match result {
        Err(_error) => String::from(""),
        Ok(s) => String::from(s),
    }
}

#[cfg(test)]
mod string_utils_tests {
    use crate::utils::strings::utf8_to_string;

    #[test]
    fn test_utf8_to_string() {
        assert_eq!(utf8_to_string(&[255]), "");
        assert_eq!(utf8_to_string(&[104, 101, 108, 108, 111]), "hello");
    }
}
