use {
    crate::{node::NodeInfo, request::Request, rpc::Rpc},
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    request_rpc: Rpc,
    from: NodeInfo,
    to: Option<NodeInfo>,
    //TODO: add body
}

impl Response {
    pub fn new(from: NodeInfo, to: Option<NodeInfo>, rpc: Rpc) -> Self {
        Self {
            from,
            to,
            request_rpc: rpc,
        }
    }

    pub fn get_request_rpc(&self) -> &Rpc {
        &self.request_rpc
    }

    pub fn get_from(&self) -> &NodeInfo {
        &self.from
    }

    pub fn get_to(&self) -> Option<&NodeInfo> {
        self.to.as_ref()
    }

    pub fn from_request(req: Request) -> Self {
        Self {
            from: req.get_to().clone(),
            to: req.get_from().map(|f| f.clone()),
            request_rpc: req.get_rpc().clone(),
        }
    }
}
