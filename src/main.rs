mod bucket;
mod error;
mod in_memory_hash_table;
mod key;
mod node;
mod rpc;

use {
    async_std::{
        io::BufReader,
        net::{TcpListener, TcpStream},
        prelude::*,
        sync::RwLock,
        task,
    },
    error::Result,
    node::Node,
    rpc::Rpc,
    std::sync::Arc,
};

async fn main_loop() -> Result<()> {
    let node = Arc::new(RwLock::new(Node::new("127.0.0.1", 8888)?));
    let listener = TcpListener::bind("127.0.0.1:8888").await?;
    let mut incoming = listener.incoming();
    while let Some(Ok(stream)) = incoming.next().await {
        let node = node.clone();
        task::spawn(async { connection_loop(stream, node).await });
    }
    Ok(())
}

async fn connection_loop(stream: TcpStream, node: Arc<RwLock<Node>>) -> Result<()> {
    println!("Incoming stream from '{:?}'", stream.peer_addr()?);
    let stream = Arc::new(stream);
    let reader = BufReader::new(&*stream);
    let mut lines = reader.lines();
    while let Some(Ok(line)) = lines.next().await {
        let req: Result<Rpc> = line.parse();
        match req {
            Ok(req) => match req {
                Rpc::Ping => unimplemented!("unimplemented PING"),
                Rpc::FindValue(k) => {
                    let node = node.read().await;
                    if let Some(v) = node.find_value(&k) {
                        let mut stream = &*stream;
                        stream.write_all(v.as_ref()).await?;
                        stream.write(b"\n").await?;
                    }
                }
                Rpc::FindNode(_k) => unimplemented!("unimplemented FIND_NODE"),
                Rpc::Store(k, v) => {
                    let mut node = node.write().await;
                    let _ = node.store(k.into(), v.into());
                }
            },
            Err(e) => println!("Error: {}", e),
        }
    }
    Ok(())
}

#[async_std::main]
async fn main() {
    let result = main_loop().await;
    match result {
        Ok(..) => println!("Server exited"),
        Err(e) => println!("Server exited with unexpected error: {}", e),
    }
}
