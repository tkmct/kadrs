#![feature(async_closure)]
use async_std::{
    io::BufReader,
    net::{TcpListener, TcpStream},
    prelude::*,
    task,
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

async fn main_loop() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8888").await?;

    let mut incoming = listener.incoming();
    while let Some(stream) = incoming.next().await {
        let stream = stream?;
        println!("Incoming stream from '{:?}'", stream.peer_addr()?);
        let reader = BufReader::new(stream);
        let mut lines = reader.lines();
        while let Some(line) = lines.next().await {
            let line = line?;
            println!("Received line: {}", line);
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

    println!("Hello, world!");
}
