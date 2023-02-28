use crate::maelstrom::msg_protocol::{MaelstromMessage, MessageBody};
use crate::maelstrom::{gen_msg_id, InitData, MessageWriter};
use color_eyre::eyre::Result;
use serde::{Deserialize, Serialize};

pub const REQ_MSG_TYPE: &str = "echo";

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct EchoData {
    echo: String,
}

pub fn handle_echo(
    response_writer: &mut MessageWriter,
    init_data: &InitData,
    msg: MaelstromMessage<EchoData>,
) -> Result<()> {
    response_writer.write(MaelstromMessage {
        src: init_data.node_id.clone(),
        dest: msg.src,
        body: MessageBody {
            msg_type: "echo_ok".to_string(),
            msg_id: Some(gen_msg_id()),
            in_reply_to: msg.body.msg_id,
            data: EchoData {
                echo: msg.body.data.echo,
            },
        },
    })
}
