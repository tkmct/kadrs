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
        sync::Mutex,
        task,
    },
    error::Result,
    in_memory_hash_table::Table,
    rpc::Rpc,
    std::sync::Arc,
};

async fn main_loop() -> Result<()> {
    let table = Arc::new(Mutex::new(Table::new()));
    let listener = TcpListener::bind("127.0.0.1:8888").await?;
    let mut incoming = listener.incoming();
    while let Some(Ok(stream)) = incoming.next().await {
        let table = table.clone();
        task::spawn(async { connection_loop(stream, table).await });
    }
    Ok(())
}

async fn connection_loop(stream: TcpStream, table: Arc<Mutex<Table>>) -> Result<()> {
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
                    let table = table.lock().await;
                    if let Some(v) = table.get(k) {
                        let mut stream = &*stream;
                        stream.write_all(v).await?;
                        stream.write(b"\n").await?;
                    }
                }
                Rpc::FindNode(_k) => unimplemented!("unimplemented FIND_NODE"),
                Rpc::Store(k, v) => {
                    let mut table = table.lock().await;
                    let _ = table.put(k.into(), v.into());
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
