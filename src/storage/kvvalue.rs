use crate::database::unit_content::UnitContent;

#[derive(Clone, Debug, PartialEq)]
pub struct KVValue(Vec<u8>);

impl KVValue {
    pub fn new(data: &[u8]) -> Self {
        Self(data.to_owned())
    }
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl From<Vec<u8>> for KVValue {
    fn from(data: Vec<u8>) -> KVValue {
        KVValue::new(&data)
    }
}

impl From<&[u8]> for KVValue {
    fn from(data: &[u8]) -> KVValue {
        KVValue::new(data)
    }
}

impl From<&str> for KVValue {
    fn from(data: &str) -> KVValue {
        KVValue::new(data.as_bytes())
    }
}

impl From<UnitContent> for KVValue {
    fn from(unit_content: UnitContent) -> KVValue {
        KVValue::new(&unit_content.marshal())
    }
}

#[cfg(test)]
mod kvvalue_tests {
    use crate::storage::kvvalue::KVValue;

    #[test]
    fn test_creation() {
        let data = [1, 2, 3];
        let value = KVValue::new(&data);
        assert_eq!(value.as_bytes(), &data)
    }

    #[test]
    fn test_from_bytes() {
        let data = vec![1, 2, 3];
        let value_1 = KVValue::from(data.as_slice());
        let value_2 = KVValue::from(data);
        assert_eq!(value_1, value_2);
        assert_eq!(value_1.as_bytes(), &[1, 2, 3])
    }

    #[test]
    fn test_from_str() {
        let value = KVValue::from("aaa");
        assert_eq!(value.as_bytes(), &[97, 97, 97])
    }
}
