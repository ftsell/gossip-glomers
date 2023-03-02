//! Implementation of the [broadcast workflow](https://github.com/jepsen-io/maelstrom/blob/main/doc/workloads.md#workload-broadcast)

use crate::broadcast_workload::topology::Topology;
use crate::maelstrom::msg_protocol::AnonMessage;
use crate::maelstrom::{InitData, MessageWriter};
use color_eyre::eyre::{eyre, Result};
use std::collections::HashSet;
use uuid::Uuid;

mod client_rpc;
mod gossip;
mod topology;

pub const MSG_TYPES: &[&str] = &["topology", "broadcast", "read", "broadcast_gossip"];

/// A data structure used to store information and handle messages
#[derive(Debug)]
pub struct BroadcastStore {
    init_data: InitData,
    values: Vec<serde_json::Value>,
    known_gossip: HashSet<Uuid>,
    topology: Option<Topology>,
}

impl BroadcastStore {
    pub fn new(init_data: InitData) -> Self {
        Self {
            init_data,
            values: Vec::with_capacity(500),
            known_gossip: HashSet::with_capacity(500),
            topology: None,
        }
    }

    pub fn handle_msg(&mut self, msg_writer: &mut MessageWriter, msg: AnonMessage) -> Result<()> {
        match msg.body.msg_type.as_str() {
            "topology" => self.handle_topology_msg(msg_writer, msg.downparse()?),
            "broadcast" => self.handle_broadcast_msg(msg_writer, msg.downparse()?),
            "read" => self.handle_read_msg(msg_writer, msg),
            "broadcast_gossip" => self.handle_gossip(msg_writer, msg.downparse()?),
            _ => Err(eyre!(
                "Broadcast store got a message type that it couldn't handle"
            )),
        }
    }
}
