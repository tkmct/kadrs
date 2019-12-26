use {
    crate::key::Key,
    serde::{Deserialize, Serialize},
};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
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
