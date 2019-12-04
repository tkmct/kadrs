use {
    crate::{bucket::KBucket, error::Result, in_memory_hash_table::Table, key::Key},
    serde::{Deserialize, Serialize},
    std::net::{Ipv4Addr, SocketAddrV4},
};

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct NodeInfo {
    host: SocketAddrV4,
    id: Key,
}

impl NodeInfo {
    pub fn get_id(&self) -> &Key {
        &self.id
    }

    pub fn new(addr: &str, port: u16, id: Key) -> Result<Self> {
        let addr: Ipv4Addr = addr.parse()?;
        Ok(Self {
            host: SocketAddrV4::new(addr, port),
            id,
        })
    }
}

pub struct Node {
    id: Key,
    host: SocketAddrV4,
    local_table: Table,
    k_bucket: KBucket,
}

impl Node {
    pub fn new(addr: &str, port: u16) -> Result<Self> {
        let addr: Ipv4Addr = addr.parse()?;
        let host = SocketAddrV4::new(addr, port);
        // generate id from ip address and port
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
        self.k_bucket.update_bucket(node_info)
    }
}
