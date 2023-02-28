//! Basic message types as defined by the [Maelstrom Protocol](https://github.com/jepsen-io/maelstrom/blob/main/doc/protocol.md)

use color_eyre::eyre::Result;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

pub type NodeId = String;

pub type AnonMessage = MaelstromMessage<serde_json::Value>;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct MaelstromMessage<T> {
    pub src: NodeId,
    pub dest: NodeId,
    pub body: MessageBody<T>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct MessageBody<T> {
    pub msg_id: Option<usize>,
    pub in_reply_to: Option<usize>,
    #[serde(rename = "type")]
    pub msg_type: String,
    #[serde(flatten)]
    pub data: T,
}

impl AnonMessage {
    /// Try to parse the contained data further into a certain type
    pub fn downparse<T>(self) -> Result<MaelstromMessage<T>>
    where
        T: DeserializeOwned,
    {
        Ok(MaelstromMessage {
            src: self.src,
            dest: self.dest,
            body: MessageBody {
                msg_type: self.body.msg_type,
                msg_id: self.body.msg_id,
                in_reply_to: self.body.in_reply_to,
                data: serde_json::from_value(self.body.data)?,
            },
        })
    }
}
