#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct UnitKey(Vec<u8>);

impl UnitKey {
    pub fn new(data: &[u8]) -> Self {
        Self(data.to_owned())
    }
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl From<Vec<u8>> for UnitKey {
    fn from(data: Vec<u8>) -> UnitKey {
        UnitKey::new(&data)
    }
}

impl From<&[u8]> for UnitKey {
    fn from(data: &[u8]) -> UnitKey {
        UnitKey::new(data)
    }
}

impl From<&str> for UnitKey {
    fn from(data: &str) -> UnitKey {
        UnitKey::new(data.as_bytes())
    }
}

impl From<UnitKey> for Vec<u8> {
    fn from(key: UnitKey) -> Vec<u8> {
        key.0
    }
}

impl ToString for UnitKey {
    fn to_string(&self) -> String {
        String::from_utf8_lossy(self.0.as_slice()).to_string()
    }
}

#[cfg(test)]
mod unit_key_tests {
    use super::UnitKey;

    #[test]
    fn test_from_vec() {
        let input = vec![1, 2, 3];
        let key = UnitKey::from(input.clone());
        assert_eq!(key.as_bytes(), input.as_slice())
    }

    #[test]
    fn test_from_slice() {
        let input = vec![3, 2, 1, 0];
        let key = UnitKey::from(input.as_slice());
        assert_eq!(key.as_bytes(), input.as_slice())
    }

    #[test]
    fn test_from_str() {
        let thing = "abc";
        let key = UnitKey::from(thing);
        assert_eq!(key.as_bytes(), &[97, 98, 99])
    }

    #[test]
    fn test_to_vec() {
        let key = UnitKey::new(&[1, 2, 3]);
        let v: Vec<u8> = key.into();
        assert_eq!(v, vec![1, 2, 3])
    }
}
