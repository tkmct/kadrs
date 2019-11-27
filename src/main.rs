mod error;
mod hash_table;

use {
    async_std::{
        io::BufReader,
        net::{TcpListener, TcpStream},
        prelude::*,
        sync::Mutex,
        task,
    },
    bytes::Bytes,
    error::Error,
    hash_table::Table,
    std::sync::Arc,
};

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
enum Command {
    Get(String),
    Put(String, String),
    Succ(String),
}

fn parse_command(req_s: &str) -> Result<Command> {
    let req_v: Vec<&str> = req_s.split(' ').collect();
    match req_v[0] {
        "get" => {
            if req_v.len() == 2 {
                Ok(Command::Get(req_v[1].to_owned()))
            } else {
                Err(Error::InvalidRequest(format!(
                    "expected argument length for `get` 1, given {}",
                    req_v.len() - 1
                )))
            }
        }
        "put" => {
            if req_v.len() == 3 {
                Ok(Command::Put(req_v[1].to_owned(), req_v[2].to_owned()))
            } else {
                Err(Error::InvalidRequest(format!(
                    "expected argument length for `put` 2, given {}",
                    req_v.len() - 1
                )))
            }
        }
        "succ" => {
            if req_v.len() == 2 {
                Ok(Command::Succ(req_v[1].to_owned()))
            } else {
                Err(Error::InvalidRequest(format!(
                    "expected argument length for `succ` 1, given {}",
                    req_v.len() - 1
                )))
            }
        }
        _ => Err(Error::InvalidRequest(format!(
            "Command not found: {}",
            req_v[0]
        ))),
    }
}

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
        match parse_command(line.as_ref()) {
            Ok(command) => match command {
                Command::Get(k) => {
                    let table = table.lock().await;
                    if let Some(v) = table.get(k.into()) {
                        let mut stream = &*stream;
                        stream.write_all(v).await?;
                        stream.write(b"\n").await?;
                    }
                }
                Command::Put(k, v) => {
                    let mut table = table.lock().await;
                    let _ = table.put(k.into(), Bytes::from(v));
                }
                Command::Succ(_k) => {
                    println!("Successor");
                }
            },
            Err(e) => println!("Error: {}", e),
        }
    }
    Ok(())
}

fn main() {
    let result = task::block_on(main_loop());
    match result {
        Ok(..) => println!("Server exited"),
        Err(e) => println!("Server exited with unexpected error: {}", e),
    }
}
