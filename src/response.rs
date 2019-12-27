use {
    crate::{node::NodeInfo, request::Request, rpc::Rpc},
    arrayvec::ArrayVec,
    serde::{Deserialize, Serialize},
};

const K: usize = 10;

#[derive(Debug, Serialize, Deserialize)]
pub enum ResponseBody {
    PONG,
    VALUE(Vec<u8>),
    NODES(Vec<NodeInfo>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    request_rpc: Rpc,
    from: NodeInfo,
    to: Option<NodeInfo>,
    body: Option<ResponseBody>,
}

impl Response {
    pub fn new(from: NodeInfo, to: Option<NodeInfo>, rpc: Rpc, body: Option<ResponseBody>) -> Self {
        Self {
            from,
            to,
            request_rpc: rpc,
            body,
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

    pub fn get_body(&self) -> Option<&ResponseBody> {
        self.body.as_ref()
    }

    pub fn set_body(&mut self, body: Option<ResponseBody>) {
        self.body = body
    }

    pub fn from_request(req: Request) -> Self {
        Self {
            from: req.get_to().clone(),
            to: req.get_from().map(|f| f.clone()),
            request_rpc: req.get_rpc().clone(),
            body: None,
        }
    }
}
