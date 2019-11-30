use {
    crate::{error::Result, in_memory_hash_table::Table, key::Key},
    std::net::{Ipv4Addr, SocketAddrV4},
};

pub struct NodeInfo {
    host: SocketAddrV4,
    id: Key,
}

impl NodeInfo {
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
        })
    }
}
