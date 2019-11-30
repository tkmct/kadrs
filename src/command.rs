use super::error::{Error, Result};

#[derive(Debug)]
pub enum Command {
    Get(String),
    Put(String, String),
    Succ(String),
}

pub fn parse_command(req_s: &str) -> Result<Command> {
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
