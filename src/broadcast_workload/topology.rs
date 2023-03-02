use crate::broadcast_workload::BroadcastStore;
use crate::maelstrom::msg_protocol::{MaelstromMessage, NodeId};
use crate::maelstrom::MessageWriter;
use color_eyre::eyre::{eyre, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Information regarding node topology
///
/// The topology is stored as a Map of *node-id* to its neighbors.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub(super) struct Topology {
    topology: HashMap<NodeId, Vec<NodeId>>,
}

impl Topology {
    pub fn get_neighbors(&self, node: &NodeId) -> Option<&Vec<NodeId>> {
        self.topology.get(node)
    }
}

impl BroadcastStore {
    pub(super) fn handle_topology_msg(
        &mut self,
        msg_writer: &mut MessageWriter,
        msg: MaelstromMessage<Topology>,
    ) -> Result<()> {
        self.topology = Some(msg.body.data.clone());
        tracing::debug!(
            "Received broadcast topology. Own neighbors are {:?}",
            self.get_own_neighbors().unwrap()
        );
        msg_writer.write(msg.make_reply("topology_ok".to_string(), ()))
    }

    pub(super) fn get_own_neighbors(&self) -> Result<&Vec<NodeId>> {
        match &self.topology {
            None => Err(eyre!("Topology has not yet been setup by Maelstrom")),
            Some(topology) => Ok(topology
                .get_neighbors(&self.init_data.node_id)
                .expect("Topology information does not include own node id")),
        }
    }
}
