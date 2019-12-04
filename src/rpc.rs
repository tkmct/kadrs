use {
    crate::{error::Error, key::Key},
    serde::{Deserialize, Serialize},
    std::str::FromStr,
};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Rpc {
    /// PING is used to check if node is online
    /// request must be exact string with `PING`
    Ping,

    /// STORE is used to store given key value pair
    /// request string must be a shape of following
    /// `STORE <key> <value>`
    /// <key> must be 160-bit data represented as bytes of length 20.
    /// <value> can be represented as any kind of data but parsed into Vec<u8>
    Store(Key, Vec<u8>),

    /// FIND_NODE is used to find closest nodes with given 160-bit id.
    /// request string must be a shape of following
    /// `FIND_NODE <id>`
    /// <id> must be 160-bit data represented as bytes of length 20.
    FindNode(Key),

    /// FIND_VALUE is used to find value for given 160-bit key.
    /// request string must be a shape of following
    /// `FIND_VALUE <key>`
    /// <key> must be 160-bit data represented as bytes of length 20.
    FindValue(Key),
}

const PING: &'static str = "PING";
const STORE: &'static str = "STORE";
const FIND_NODE: &'static str = "FIND_NODE";
const FIND_VALUE: &'static str = "FIND_VALUE";

impl FromStr for Rpc {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let req_v: Vec<&str> = s.split(' ').collect();
        match req_v[0] {
            PING => {
                if req_v.len() == 1 {
                    Ok(Self::Ping)
                } else {
                    Err(Error::InvalidRequest(
                        "method `PING` does not receive arguments".to_owned(),
                    ))
                }
            }
            STORE => {
                if req_v.len() == 3 {
                    Ok(Self::Store(req_v[1].to_owned().into(), req_v[2].into()))
                } else {
                    Err(Error::InvalidRequest(format!(
                        "expected argument length for `STORE` 2, given {}",
                        req_v.len() - 1
                    )))
                }
            }
            FIND_NODE => {
                if req_v.len() == 2 {
                    Ok(Self::FindNode(req_v[1].into()))
                } else {
                    Err(Error::InvalidRequest(format!(
                        "expected argument length for `FIND_NODE` 1, given {}",
                        req_v.len() - 1
                    )))
                }
            }
            FIND_VALUE => {
                if req_v.len() == 2 {
                    Ok(Self::FindValue(req_v[1].into()))
                } else {
                    Err(Error::InvalidRequest(format!(
                        "expected argument length for `FIND_VALUE` 1, given {}",
                        req_v.len() - 1
                    )))
                }
            }
            _ => Err(Error::RequestParse(s.to_owned())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ping() {
        let req = "PING".parse::<Rpc>().unwrap();
        assert_eq!(req, Rpc::Ping);
    }

    #[test]
    fn test_parse_store() {
        let req = "STORE key val".parse::<Rpc>().unwrap();
        assert_eq!(req, Rpc::Store("key".into(), "val".into()));
    }

    #[test]
    fn test_parse_find_node() {
        let req = "FIND_NODE key".parse::<Rpc>().unwrap();
        assert_eq!(req, Rpc::FindNode("key".into()));
    }

    #[test]
    fn test_parse_find_value() {
        let req = "FIND_VALUE key".parse::<Rpc>().unwrap();
        assert_eq!(req, Rpc::FindValue("key".into()));
    }

    #[test]
    fn test_parse_error() {
        let result = Rpc::from_str("INVALID_COMMAND kkkk");
        assert!(result.is_err(), "parse should return error")
    }
}
