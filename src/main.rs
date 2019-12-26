#![feature(try_trait)]

mod bucket;
mod error;
mod in_memory_hash_table;
mod key;
mod node;
mod request;
mod response;
mod rpc;

use {
    async_std::{
        io::BufReader,
        net::{TcpListener, TcpStream},
        prelude::*,
        sync::RwLock,
        task,
    },
    clap::{App, Arg},
    error::Result,
    node::Node,
    request::Request,
    rpc::Rpc,
    std::{net::SocketAddrV4, sync::Arc},
};

async fn start(host: SocketAddrV4, neighbor: Option<SocketAddrV4>) -> Result<()> {
    let node = Arc::new(RwLock::new(Node::new(host)?));

    // Ping neighbor
    if let Some(n) = neighbor {
        let req = Request::new(Some(host.into()), Rpc::Ping, n.into());
        let res = req.send().await;
        println!("{:?}", res);
        let mut node = node.write().await;
        node.update_bucket(n.into());
    }

    let listener = TcpListener::bind(host).await?;
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
        let deserialized = serde_json::from_str::<Request>(&line);
        if deserialized.is_err() {
            println!("Request deserialize fail: {:?}", deserialized.err());
            continue;
        }
        let req = deserialized.unwrap();
        println!("{:?}", req);
        match req.get_rpc() {
            Rpc::Ping => {
                let mut stream = &*stream;
                stream.write("PONG".as_bytes()).await?;
                stream.write(b"\n").await?;
            }
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
                let _ = node.store(k.clone(), v.clone());
            }
        }
        let mut node = node.write().await;
        match req.get_from() {
            Some(n) => node.update_bucket(n.clone()),
            _ => {}
        }
    }
    Ok(())
}

#[async_std::main]
async fn main() {
    let app = App::new("kadrs")
        .version("0.1.0")
        .about("server app for kadrs")
        .arg(Arg::with_name("host").required(true))
        .arg(Arg::with_name("neighbor"));
    let matches = app.get_matches();
    let host: SocketAddrV4 = match matches.value_of("host").unwrap().parse() {
        Ok(s) => s,
        Err(_) => panic!("Invalid host string"),
    };
    let neighbor: Option<SocketAddrV4> = matches
        .value_of("neighbor")
        .map(|s| s.parse().expect("Invalid host string"));

    // start a server
    let server = start(host, neighbor).await;
    match server {
        Ok(..) => println!("Server exited"),
        Err(e) => println!("Server exited with unexpected error: {}", e),
    }
    //
    // request sample input
    // {"node_info":{"host":"127.0.0.1:2000","id":[44,112,225,43,122,6,70,249,34,121,244,39,199,179,142,115,52,216,229,56]},"rpc":{"FindValue":[129,116,9,150,135,162,102,33,244,226,205,215,204,3,179,218,206,219,63,185]}}
    // {"node_info":{"host":"127.0.0.1:2000","id":[44,112,225,43,122,6,70,249,34,121,244,39,199,179,142,115,52,216,229,56]},"rpc":{"Store":[[129,116,9,150,135,162,102,33,244,226,205,215,204,3,179,218,206,219,63,185],[104,101,108,108,111,32,119,111,114,108,100]]}}
    //
}
