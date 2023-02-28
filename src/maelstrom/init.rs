use crate::maelstrom::msg_protocol::{MaelstromMessage, MessageBody, NodeId};
use crate::maelstrom::{gen_msg_id, MessageReader, MessageWriter};
use color_eyre::eyre::{Result, WrapErr};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct InitData {
    /// The ID of the current node.
    pub node_id: NodeId,
    /// All nodes in the cluster, including the current node.
    /// All nodes receive an identical list.
    pub node_ids: Vec<NodeId>,
}

/// Wait for an init message and process it
pub fn init(reader: &mut MessageReader, writer: &mut MessageWriter) -> Result<InitData> {
    let init_msg: MaelstromMessage<InitData> =
        reader.read().wrap_err("Could not read init message")?;
    writer
        .write(MaelstromMessage {
            src: init_msg.dest.clone(),
            dest: init_msg.src.clone(),
            body: MessageBody {
                msg_id: Some(gen_msg_id()),
                in_reply_to: init_msg.body.msg_id,
                msg_type: "init_ok".to_string(),
                data: (),
            },
        })
        .wrap_err("Could not write init response")?;

    tracing::debug!(
        "Successfully initialized as node {}",
        init_msg.body.data.node_id
    );
    Ok(init_msg.body.data)
}
