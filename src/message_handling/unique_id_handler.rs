use uuid::Uuid;
use crate::message::{InitMsg, Message, MessageBody, MessageBodyMetadata, MessageHeader};
use crate::message_handling::{gen_msg_id, MessageHandler};

#[derive(Debug, Clone)]
pub struct UniqueIdHandler {
    init_data: InitMsg
}

impl UniqueIdHandler {
    pub fn new(init_data: InitMsg) -> Self {
        Self {
            init_data,
        }
    }
}

impl MessageHandler<()> for UniqueIdHandler {
    type Response = Message;

    fn handle(
        &mut self,
        header: &MessageHeader,
        metadata: &MessageBodyMetadata,
        _: &(),
    ) -> Self::Response {
        Message {
            header: MessageHeader {
                src: self.init_data.node_id.clone(),
                dest: header.src.clone()
            },
            body: MessageBody::UniqueIdResponse {
                metadata: MessageBodyMetadata {
                    msg_id: Some(gen_msg_id()),
                    in_reply_to: metadata.msg_id,
                },
                id: Uuid::new_v4(),
            }
        }
    }
}
