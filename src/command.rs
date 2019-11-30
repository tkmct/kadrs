use crate::{
    error::{Error, Result},
    key::Key,
};

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Ping,
    Store(Key, Vec<u8>),
    FindNode(Key),
    FindValue(Key),
}

const PING: &'static str = "PING";
const STORE: &'static str = "STORE";
const FIND_NODE: &'static str = "FIND_NODE";
const FIND_VALUE: &'static str = "FIND_VALUE";

pub fn parse_command(req_s: &str) -> Result<Command> {
    let req_v: Vec<&str> = req_s.split(' ').collect();
    match req_v[0] {
        PING => {
            if req_v.len() == 1 {
                Ok(Command::Ping)
            } else {
                Err(Error::InvalidRequest(
                    "method `PING` does not receive arguments".to_owned(),
                ))
            }
        }
        STORE => {
            if req_v.len() == 3 {
                Ok(Command::Store(req_v[1].to_owned().into(), req_v[2].into()))
            } else {
                Err(Error::InvalidRequest(format!(
                    "expected argument length for `STORE` 2, given {}",
                    req_v.len() - 1
                )))
            }
        }
        FIND_NODE => {
            if req_v.len() == 2 {
                Ok(Command::FindNode(req_v[1].into()))
            } else {
                Err(Error::InvalidRequest(format!(
                    "expected argument length for `FIND_NODE` 1, given {}",
                    req_v.len() - 1
                )))
            }
        }
        FIND_VALUE => {
            if req_v.len() == 2 {
                Ok(Command::FindValue(req_v[1].into()))
            } else {
                Err(Error::InvalidRequest(format!(
                    "expected argument length for `FIND_VALUE` 1, given {}",
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ping() {
        let command_str = "PING";
        let parsed = parse_command(command_str).unwrap();
        assert_eq!(Command::Ping, parsed);
    }

    #[test]
    fn test_parse_store() {
        let command_str = "STORE key val";
        let parsed = parse_command(command_str).unwrap();
        assert_eq!(Command::Store("key".into(), "val".into()), parsed);
    }

    #[test]
    fn test_parse_find_node() {
        let command_str = "FIND_NODE key";
        let parsed = parse_command(command_str).unwrap();
        assert_eq!(parsed, Command::FindNode("key".into()));
    }

    #[test]
    fn test_parse_find_value() {
        let command_str = "FIND_VALUE key";
        let parsed = parse_command(command_str).unwrap();
        assert_eq!(parsed, Command::FindValue("key".into()));
    }
}
