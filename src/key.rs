use ring::digest::{digest, SHA256};

/// Key struct represents Key of (Key, Value) pair and ID of nodes.
/// id and key are represented as 160-bit identifier.
/// distance between two keys are calcuated using XOR.
#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub struct Key([u8; 20]);

impl Key {
    pub fn new(k: [u8; 20]) -> Self {
        Self(k)
    }

    pub fn distance(&self, rhs: &Key) -> [u8; 20] {
        let xor: Vec<u8> = self
            .0
            .iter()
            .zip(rhs.0.iter())
            .map(|(l, r)| l ^ r)
            .collect();
        let mut arr = [0; 20];
        arr.copy_from_slice(&xor[0..20]);
        arr
    }
}

impl From<String> for Key {
    fn from(s: String) -> Self {
        let hashed = digest(&SHA256, s.as_ref());
        let mut arr = [0; 20];
        arr.copy_from_slice(&hashed.as_ref()[0..20]);
        Self(arr)
    }
}

impl From<&str> for Key {
    fn from(s: &str) -> Self {
        s.to_owned().into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_distance() {
        let key1 = Key::new([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        let key2 = Key::new([1, 0, 1, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        let d = key1.distance(&key2);

        assert_eq!(d, key2.0);
    }
}
