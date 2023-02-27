use crate::message::{
    EchoRequest, EchoResponse, InitMsg, Message, MessageBody, MessageBodyMetadata, MessageHeader,
};
use crate::message_handling::gen_msg_id;
use crate::message_handling::handler::MessageHandler;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct EchoHandler {
    init_data: InitMsg,
}

impl EchoHandler {
    pub fn new(init_data: InitMsg) -> Self {
        Self { init_data }
    }
}

impl MessageHandler<EchoRequest> for EchoHandler {
    type Response = Message;

    fn handle(
        &mut self,
        header: &MessageHeader,
        metadata: &MessageBodyMetadata,
        msg: &EchoRequest,
    ) -> Message {
        Message {
            header: MessageHeader {
                src: self.init_data.node_id.clone(),
                dest: header.src.clone(),
            },
            body: MessageBody::EchoResponse {
                metadata: MessageBodyMetadata {
                    msg_id: Some(gen_msg_id()),
                    in_reply_to: metadata.msg_id,
                },
                content: EchoResponse {
                    echo: msg.echo.clone(),
                },
            },
        }
    }
}
