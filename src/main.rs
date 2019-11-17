use async_std::{io::BufReader, net::TcpListener, prelude::*, task};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

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
                panic!("Invalid inputs length, expected 2 got {}", req_v.len());
            }
        }
        "put" => {
            if req_v.len() == 3 {
                Ok(Command::Put(req_v[1].to_owned(), req_v[2].to_owned()))
            } else {
                panic!("Invalid inputs length, expected 2 got {}", req_v.len());
            }
        }
        "succ" => {
            if req_v.len() == 2 {
                Ok(Command::Succ(req_v[1].to_owned()))
            } else {
                panic!("Invalid inputs length, expected 2 got {}", req_v.len());
            }
        }
        _ => panic!("No matched command"),
    }
}

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
            let command = parse_command(line.as_ref())?;
            println!("Received command: {:?}", command);
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
