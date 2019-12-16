use {
    crate::{bucket::KBucket, error::Result, in_memory_hash_table::Table, key::Key},
    serde::{Deserialize, Serialize},
    std::net::SocketAddrV4,
};

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct NodeInfo {
    host: SocketAddrV4,
    id: Key,
}

impl NodeInfo {
    pub fn new(host: SocketAddrV4, id: Key) -> Self {
        Self { host, id }
    }

    pub fn get_id(&self) -> &Key {
        &self.id
    }
    pub fn get_host(&self) -> &SocketAddrV4 {
        &self.host
    }
}

impl From<SocketAddrV4> for NodeInfo {
    fn from(host: SocketAddrV4) -> NodeInfo {
        let id = Key::from(format!("{}", host));
        NodeInfo { host, id }
    }
}

pub struct Node {
    id: Key,
    host: SocketAddrV4,
    local_table: Table,
    k_bucket: KBucket,
}

impl Node {
    pub fn new(host: SocketAddrV4) -> Result<Self> {
        let id = Key::from(format!("{}", host));

        Ok(Self {
            host,
            id,
            local_table: Table::new(),
            k_bucket: KBucket::new(),
        })
    }

    pub fn find_value(&self, key: &Key) -> Option<Vec<u8>> {
        self.local_table.get(key).and_then(|v| Some(v.clone()))
    }

    pub fn store(&mut self, key: Key, value: Vec<u8>) {
        self.local_table.put(key, value);
    }

    pub fn update_bucket(&mut self, node_info: NodeInfo) {
        let distance = node_info.get_id().distance(&self.id);
        self.k_bucket.update_bucket(node_info, distance);
    }
}
