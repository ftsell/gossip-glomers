use crate::message::{Message, MessageBodyMetadata, MessageHeader};

pub trait MessageHandler<M> {
    type Response;

    fn handle(
        &mut self,
        header: &MessageHeader,
        metadata: &MessageBodyMetadata,
        msg: &M,
    ) -> Self::Response;
}
