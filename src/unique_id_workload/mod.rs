use crate::maelstrom::msg_protocol::AnonMessage;
use crate::maelstrom::MessageWriter;
use color_eyre::eyre::Result;
use serde::Serialize;
use uuid::Uuid;

pub const REQ_MSG_TYPE: &str = "generate";

#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct UniqueIdResponse {
    id: Uuid,
}

pub fn handle_generate(response_writer: &mut MessageWriter, msg: AnonMessage) -> Result<()> {
    response_writer.write(msg.make_reply(
        "generate_ok".to_string(),
        UniqueIdResponse { id: Uuid::new_v4() },
    ))
}
