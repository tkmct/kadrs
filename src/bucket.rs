use {
    crate::{
        error::{Error, Result},
        key::Key,
        node::NodeInfo,
    },
    std::{collections::VecDeque, mem::MaybeUninit},
};

/// let 0 <= i < 160, store k nodes info whose distance is 2^i <= d < 2^(i+1) far.
/// bucket has at most k nodes
/// when node received any message from other nodes, bucket manages nodes in the following way
/// 1. if node is already in the bucket, move it to the tail of the bucket.
/// 2. if node is not in the bucket, and bucket is not full, append the node at the tail.
/// 3. if node is not in the bucket, and bucket is full, ping the least-recently seen node which is
///    at the head of the bucket, if it doesn't respond, evict the least-recently seen node and push
///    new node at the tail. if it does respond, discard new node.
pub struct Bucket {
    nodes: VecDeque<NodeInfo>,
    k: usize,
}

impl Bucket {
    pub fn new(k: usize) -> Self {
        Self {
            nodes: VecDeque::new(),
            k,
        }
    }

    /// append given node to the tail of the bucket
    pub fn push_back(&mut self, node_info: NodeInfo) -> Result<()> {
        if self.nodes.len() == self.k {
            return Err(Error::MaxCapacity);
        }
        self.nodes.push_back(node_info);
        Ok(())
    }

    pub fn remove(&mut self, i: usize) {
        self.nodes.remove(i);
    }

    /// move item at given index to tail of the bucket
    pub fn move_to_tail(&mut self, i: usize) -> Result<()> {
        if let Some(node) = self.nodes.remove(i) {
            self.nodes.push_back(node);
            Ok(())
        } else {
            Err(Error::IndexOutOfBounds(i, self.k))
        }
    }
}

/// kBucket implementation
/// store k nodes in single bucket
pub struct kBucket {
    k: usize,
    buckets: [Bucket; 160],
}

impl kBucket {
    pub fn new(k: usize) -> Self {
        let mut buckets: [Bucket; 160] = unsafe { MaybeUninit::uninit().assume_init() };
        for i in buckets.iter_mut() {
            *i = Bucket::new(k);
        }

        Self { k, buckets }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_node() {
        let mut bucket = Bucket::new(5);
        let _ = bucket.push_back(NodeInfo::new("127.0.0.1", 1999, "key1".into()).unwrap());
        let _ = bucket.push_back(NodeInfo::new("127.0.0.1", 2000, "key2".into()).unwrap());
        let _ = bucket.push_back(NodeInfo::new("127.0.0.1", 2001, "key2".into()).unwrap());
        assert_eq!(bucket.nodes.len(), 3);
    }

    #[test]
    fn test_push_node_not_exceed_k() {
        let mut bucket = Bucket::new(3);
        let _ = bucket.push_back(NodeInfo::new("127.0.0.1", 1999, "key1".into()).unwrap());
        let _ = bucket.push_back(NodeInfo::new("127.0.0.1", 2000, "key2".into()).unwrap());
        let _ = bucket.push_back(NodeInfo::new("127.0.0.1", 2001, "key2".into()).unwrap());
        let _ = bucket.push_back(NodeInfo::new("127.0.0.1", 2002, "key3".into()).unwrap());
        assert_eq!(bucket.nodes.len(), 3);
    }

    #[test]
    fn test_remove_node() {
        let mut bucket = Bucket::new(3);
        let _ = bucket.push_back(NodeInfo::new("127.0.0.1", 2001, "key1".into()).unwrap());
        let _ = bucket.push_back(NodeInfo::new("127.0.0.1", 2002, "key2".into()).unwrap());
        bucket.remove(1);
        assert_eq!(bucket.nodes.len(), 1);
    }

    #[test]
    fn test_node_move_to_tail() {
        let mut bucket = Bucket::new(5);
        let _ = bucket.push_back(NodeInfo::new("127.0.0.1", 2001, "key1".into()).unwrap());
        let _ = bucket.push_back(NodeInfo::new("127.0.0.1", 2002, "key2".into()).unwrap());
        let _ = bucket.push_back(NodeInfo::new("127.0.0.1", 2003, "key3".into()).unwrap());
        let _ = bucket.push_back(NodeInfo::new("127.0.0.1", 2004, "key4".into()).unwrap());
        let res = bucket.move_to_tail(0);
        assert!(res.is_ok(), "success move to tail");
        let back = bucket.nodes.back().unwrap();
        assert_eq!(back.get_id(), &Key::from("key1"));
    }

    #[test]
    fn test_node_move_to_tail_fail() {
        let mut bucket = Bucket::new(5);
        let _ = bucket.push_back(NodeInfo::new("127.0.0.1", 2001, "key1".into()).unwrap());
        let _ = bucket.push_back(NodeInfo::new("127.0.0.1", 2002, "key2".into()).unwrap());
        let res = bucket.move_to_tail(2);
        assert!(res.is_err(), "fail move to tail");
    }
}
