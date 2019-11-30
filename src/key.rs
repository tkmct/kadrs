use byteorder::{BigEndian, ByteOrder};
use ring::digest::{digest, SHA256};

/// Key struct of hash table. `inner` field represents key hash.
/// given string would be hashed with SHA256 and truncated into 32 bits.
#[derive(Eq, PartialEq, Hash, Clone)]
pub struct Key {
    inner: u32,
}

impl Key {
    pub fn new(s: String) -> Self {
        let hashed = digest(&SHA256, s.as_ref());
        Self {
            inner: BigEndian::read_u32(&hashed.as_ref()[0..4]),
        }
    }

    pub fn distance(&self, lhs: &Key) -> u32 {
        self.inner ^ lhs.inner
    }
}

impl From<String> for Key {
    fn from(s: String) -> Self {
        Self::new(s)
    }
}

impl From<&str> for Key {
    fn from(s: &str) -> Self {
        Self::new(s.to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_distance() {
        let key1: Key = "k1".into();
        let key2: Key = "k2".into();
        let d = key1.distance(&key2);

        assert_eq!(d, 1810272128);
    }
}
