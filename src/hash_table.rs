use bytes::Bytes;
use std::collections::HashMap;

pub struct Table {
    inner: HashMap<Bytes, Bytes>,
}

impl Table {
    pub fn new() -> Self {
        Table {
            inner: HashMap::new(),
        }
    }

    pub fn get(&self, key: Bytes) -> Option<&Bytes> {
        self.inner.get(&key)
    }

    pub fn put(&mut self, key: Bytes, value: Bytes) -> Option<Bytes> {
        self.inner.insert(key, value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_table() {
        let key = Bytes::from(&b"k1"[..]);
        let value = Bytes::from(&b"val"[..]);
        let mut table = Table::new();
        let put_result = table.put(key.clone(), value.clone());
        assert_eq!(put_result, None);
        let get_result = table.get(key);
        assert_eq!(get_result, Some(&value));
    }
}
