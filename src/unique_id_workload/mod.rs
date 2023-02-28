use crate::maelstrom::msg_protocol::{AnonMessage, MaelstromMessage, MessageBody};
use crate::maelstrom::{gen_msg_id, InitData, MessageWriter};
use color_eyre::eyre::Result;
use serde::Serialize;
use uuid::Uuid;

pub const REQ_MSG_TYPE: &str = "generate";

#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct UniqueIdResponse {
    id: Uuid,
}

pub fn handle_generate(
    response_writer: &mut MessageWriter,
    init_data: &InitData,
    msg: AnonMessage,
) -> Result<()> {
    response_writer.write(MaelstromMessage {
        src: init_data.node_id.clone(),
        dest: msg.src,
        body: MessageBody {
            msg_type: "generate_ok".to_string(),
            msg_id: Some(gen_msg_id()),
            in_reply_to: msg.body.msg_id,
            data: UniqueIdResponse { id: Uuid::new_v4() },
        },
    })
}
