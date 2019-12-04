use {
    crate::{
        error::{Error, Result},
        node::NodeInfo,
    },
    arrayvec::ArrayVec,
    std::mem::MaybeUninit,
};

// TODO: use const generics if ready
const K: usize = 20;

/// let 0 <= i < 160, store k nodes info whose distance is 2^i <= d < 2^(i+1) far.
/// bucket has at most k nodes
/// when node received any message from other nodes, bucket manages nodes in the following rule
/// 1. if node is already in the bucket, move it to the tail of the bucket.
/// 2. if node is not in the bucket, and bucket is not full, append the node at the tail.
/// 3. if node is not in the bucket, and bucket is full, ping the least-recently seen node which is
///    at the head of the bucket, if it doesn't respond, evict the least-recently seen node and push
///    new node at the tail. if it does respond, discard new node.
pub struct Bucket {
    nodes: ArrayVec<[NodeInfo; K]>,
}

impl Bucket {
    pub fn new() -> Self {
        Self {
            nodes: ArrayVec::new(),
        }
    }

    /// append given node to the tail of the bucket
    pub fn push_back(&mut self, node_info: NodeInfo) -> Result<()> {
        self.nodes.try_push(node_info).map_err(Into::into)
    }

    /// remove item at given index
    /// panics if index is out of bounds
    pub fn remove(&mut self, index: usize) -> NodeInfo {
        self.nodes.remove(index)
    }

    /// move item at given index to tail of the bucket
    /// panics if given index is out of bounds
    pub fn move_to_tail(&mut self, index: usize) -> Result<()> {
        if index >= self.nodes.len() {
            return Err(Error::IndexOutOfBounds(index, self.nodes.len() - 1));
        }

        let node = self.nodes.remove(index);
        self.nodes.push(node);
        Ok(())
    }

    /// update bucket with given node_info in rule specified above.
    //TODO: PING is not implemented yet.
    pub fn update(&mut self, node_info: NodeInfo) {}
}

/// kBucket implementation
/// store k nodes in single bucket
pub struct kBucket {
    buckets: [Bucket; 160],
}

impl kBucket {
    pub fn new() -> Self {
        let mut buckets: [Bucket; 160] = unsafe { MaybeUninit::uninit().assume_init() };
        for i in buckets.iter_mut() {
            *i = Bucket::new();
        }

        Self { buckets }
    }

    pub fn update_bucket(&mut self, node_info: NodeInfo) {
        let i = node_info.get_id().most_significant_bit();
        self.buckets[i as usize].update(node_info);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::key::Key;

    #[test]
    fn test_push_node() {
        let mut bucket = Bucket::new();
        let _ = bucket.push_back(NodeInfo::new("127.0.0.1", 1999, "key1".into()).unwrap());
        let _ = bucket.push_back(NodeInfo::new("127.0.0.1", 2000, "key2".into()).unwrap());
        let _ = bucket.push_back(NodeInfo::new("127.0.0.1", 2001, "key2".into()).unwrap());
        assert_eq!(bucket.nodes.len(), 3);
    }

    #[test]
    fn test_remove_node() {
        let mut bucket = Bucket::new();
        let _ = bucket.push_back(NodeInfo::new("127.0.0.1", 2001, "key1".into()).unwrap());
        let _ = bucket.push_back(NodeInfo::new("127.0.0.1", 2002, "key2".into()).unwrap());
        let node_info = bucket.remove(1);
        assert_eq!(bucket.nodes.len(), 1);
        assert_eq!(
            node_info,
            NodeInfo::new("127.0.0.1", 2002, "key2".into()).unwrap()
        );
    }

    #[test]
    fn test_node_move_to_tail() {
        let mut bucket = Bucket::new();
        let _ = bucket.push_back(NodeInfo::new("127.0.0.1", 2001, "key1".into()).unwrap());
        let _ = bucket.push_back(NodeInfo::new("127.0.0.1", 2002, "key2".into()).unwrap());
        let _ = bucket.push_back(NodeInfo::new("127.0.0.1", 2003, "key3".into()).unwrap());
        let _ = bucket.push_back(NodeInfo::new("127.0.0.1", 2004, "key4".into()).unwrap());
        let res = bucket.move_to_tail(0);
        assert!(res.is_ok(), "success move to tail");
        let back = bucket.nodes.last().unwrap();
        assert_eq!(back.get_id(), &Key::from("key1"));
    }

    #[test]
    fn test_node_move_to_tail_fail() {
        let mut bucket = Bucket::new();
        let _ = bucket.push_back(NodeInfo::new("127.0.0.1", 2001, "key1".into()).unwrap());
        let _ = bucket.push_back(NodeInfo::new("127.0.0.1", 2002, "key2".into()).unwrap());
        let res = bucket.move_to_tail(2);
        assert!(res.is_err(), "fail move to tail");
    }
}
