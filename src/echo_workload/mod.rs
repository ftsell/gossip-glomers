use crate::maelstrom::msg_protocol::MaelstromMessage;
use crate::maelstrom::MessageWriter;
use color_eyre::eyre::Result;
use serde::{Deserialize, Serialize};

pub const REQ_MSG_TYPE: &str = "echo";

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct EchoData {
    echo: String,
}

pub fn handle_echo(
    response_writer: &mut MessageWriter,
    msg: MaelstromMessage<EchoData>,
) -> Result<()> {
    response_writer.write(msg.make_reply("echo_ok".to_string(), msg.body.data.clone()))
}
