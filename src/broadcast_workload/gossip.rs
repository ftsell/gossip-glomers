use crate::broadcast_workload::BroadcastStore;
use crate::maelstrom::msg_protocol::{MaelstromMessage, MessageBody};
use crate::maelstrom::{gen_msg_id, MessageWriter};
use color_eyre::eyre::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct GossipData {
    pub id: Uuid,
    pub value: Value,
}

impl BroadcastStore {
    /// Gossip the given value to all neighbors
    pub(super) fn gossip_value(
        &mut self,
        msg_writer: &mut MessageWriter,
        data: GossipData,
    ) -> Result<()> {
        // ensure that the gossip message's id is saved
        self.known_gossip.insert(data.id);

        // gossip to all neighbors
        for i in self.get_own_neighbors()? {
            msg_writer.write(MaelstromMessage {
                src: self.init_data.node_id.clone(),
                dest: i.clone(),
                body: MessageBody {
                    msg_type: "broadcast_gossip".to_string(),
                    msg_id: Some(gen_msg_id()),
                    in_reply_to: None,
                    data: data.clone(),
                },
            })?;
        }
        Ok(())
    }

    /// Handle received gossip
    ///
    /// This works by storing new gossip in the internal data store and further propagating it to all neighbors.
    /// If a gossip has already been received it is instead ignored.
    pub(super) fn handle_gossip(
        &mut self,
        msg_writer: &mut MessageWriter,
        gossip_msg: MaelstromMessage<GossipData>,
    ) -> Result<()> {
        if self.known_gossip.contains(&gossip_msg.body.data.id) {
            Ok(())
        } else {
            self.values.push(gossip_msg.body.data.value.clone());
            self.gossip_value(msg_writer, gossip_msg.body.data)
        }
    }
}
