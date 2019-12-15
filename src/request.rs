use {
    crate::{error::Result, node::NodeInfo, rpc::Rpc},
    async_std::{net::TcpStream, prelude::*},
    serde::{Deserialize, Serialize},
    std::net::Shutdown,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    from: Option<NodeInfo>,
    to: NodeInfo,
    rpc: Rpc,
}

impl Request {
    pub fn new(from: Option<NodeInfo>, rpc: Rpc, to: NodeInfo) -> Self {
        Self { from, rpc, to }
    }

    pub fn get_from(&self) -> Option<&NodeInfo> {
        self.from.as_ref()
    }

    pub fn get_rpc(&self) -> &Rpc {
        &self.rpc
    }

    pub fn get_to(&self) -> &NodeInfo {
        &self.to
    }

    pub async fn send(&self) -> Result<String> {
        let req_str = serde_json::to_string(&self)?;
        let mut stream = TcpStream::connect(self.to.get_host()).await?;
        stream.write_all(req_str.as_bytes()).await?;
        stream.write("\n".as_bytes()).await?;
        let mut buf = vec![0u8; 32];
        let count = stream.read(&mut buf).await?;
        let res = String::from_utf8(buf[..count].to_vec())?.trim().to_owned();
        stream.shutdown(Shutdown::Both)?;
        Ok(res)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {}
