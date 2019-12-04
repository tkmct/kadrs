use {
    crate::{node::NodeInfo, rpc::Rpc},
    serde::{Deserialize, Serialize},
};

#[derive(Serialize, Deserialize)]
pub struct Request {
    node_info: NodeInfo,
    rpc: Rpc,
}

impl Request {
    pub fn new(node_info: NodeInfo, rpc: Rpc) -> Self {
        Self { node_info, rpc }
    }

    pub fn get_node_info(&self) -> &NodeInfo {
        &self.node_info
    }

    pub fn get_rpc(&self) -> &Rpc {
        &self.rpc
    }
}
