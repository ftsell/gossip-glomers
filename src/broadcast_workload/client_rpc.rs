use crate::broadcast_workload::gossip::GossipData;
use crate::broadcast_workload::BroadcastStore;
use crate::maelstrom::msg_protocol::{AnonMessage, MaelstromMessage};
use crate::maelstrom::MessageWriter;
use color_eyre::eyre::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub(super) struct BroadcastReq {
    message: Value,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub(super) struct ReadResponse {
    messages: Vec<Value>,
}

impl BroadcastStore {
    pub(super) fn handle_broadcast_msg(
        &mut self,
        msg_writer: &mut MessageWriter,
        msg: MaelstromMessage<BroadcastReq>,
    ) -> Result<()> {
        self.values.push(msg.body.data.message.clone());
        self.gossip_value(
            msg_writer,
            GossipData {
                value: msg.body.data.message.clone(),
                id: Uuid::new_v4(),
            },
        )?;
        msg_writer.write(msg.make_reply("broadcast_ok".to_string(), ()))
    }

    pub(super) fn handle_read_msg(
        &self,
        msg_writer: &mut MessageWriter,
        request: AnonMessage,
    ) -> Result<()> {
        msg_writer.write(request.make_reply(
            "read_ok".to_string(),
            ReadResponse {
                messages: self.values.clone(),
            },
        ))
    }
}
