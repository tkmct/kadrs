use {
    crate::{node::NodeInfo, rpc::Rpc},
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    node_info: Option<NodeInfo>,
    rpc: Rpc,
}

impl Request {
    pub fn new(node_info: Option<NodeInfo>, rpc: Rpc) -> Self {
        Self { node_info, rpc }
    }

    pub fn get_node_info(&self) -> Option<&NodeInfo> {
        self.node_info.as_ref()
    }

    pub fn get_rpc(&self) -> &Rpc {
        &self.rpc
    }
}
