use crate::database::unit_key::UnitKey;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct KVKey(Vec<u8>);

impl KVKey {
    pub fn new(data: &[u8]) -> Self {
        Self(data.to_owned())
    }
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl From<Vec<u8>> for KVKey {
    fn from(data: Vec<u8>) -> KVKey {
        KVKey::new(&data)
    }
}

impl From<&[u8]> for KVKey {
    fn from(data: &[u8]) -> KVKey {
        KVKey::new(data)
    }
}

impl From<&str> for KVKey {
    fn from(data: &str) -> KVKey {
        KVKey::new(data.as_bytes())
    }
}

impl From<KVKey> for Vec<u8> {
    fn from(key: KVKey) -> Vec<u8> {
        key.0
    }
}

impl From<&UnitKey> for KVKey {
    fn from(unit_key: &UnitKey) -> KVKey {
        KVKey::new(unit_key.as_bytes())
    }
}

#[cfg(test)]
mod kvkey_tests {
    use super::KVKey;

    #[test]
    fn test_from_vec() {
        let input = vec![1, 2, 3];
        let key = KVKey::from(input.clone());
        assert_eq!(key.as_bytes(), input.as_slice())
    }

    #[test]
    fn test_from_slice() {
        let input = vec![3, 2, 1, 0];
        let key = KVKey::from(input.as_slice());
        assert_eq!(key.as_bytes(), input.as_slice())
    }

    #[test]
    fn test_from_str() {
        let thing = "abc";
        let key = KVKey::from(thing);
        assert_eq!(key.as_bytes(), &[97, 98, 99])
    }

    #[test]
    fn test_to_vec() {
        let key = KVKey::new(&[1, 2, 3]);
        let v: Vec<u8> = key.into();
        assert_eq!(v, vec![1, 2, 3])
    }
}
