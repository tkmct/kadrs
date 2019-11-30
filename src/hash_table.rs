use {crate::key::Key, std::collections::HashMap};

pub struct Table {
    inner: HashMap<Key, Vec<u8>>,
}

impl Table {
    pub fn new() -> Self {
        Table {
            inner: HashMap::new(),
        }
    }

    pub fn get(&self, key: Key) -> Option<&Vec<u8>> {
        self.inner.get(&key)
    }

    pub fn put(&mut self, key: Key, value: Vec<u8>) -> Option<Vec<u8>> {
        self.inner.insert(key, value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_table() {
        let key: Key = "k1".into();
        let value: Vec<u8> = (&b"val"[..]).into();
        let mut table = Table::new();
        let put_result = table.put(key.clone(), value.clone());
        assert_eq!(put_result, None);
        let get_result = table.get(key).unwrap();
        assert_eq!(get_result, &value);
    }
}
