use crate::message::{Message, MessageBody, MessageBodyMetadata, MessageHeader};
use crate::message_handling::{gen_msg_id, EchoHandler, MessageHandler, UniqueIdHandler};
use crate::message_streams::{MessageInStream, MessageOutWriter};
use color_eyre::Result;
use std::io;
use tracing::Level;

mod message;
mod message_handling;
mod message_streams;

const LOG_LEVEL: Level = Level::INFO;

fn main() -> Result<()> {
    // misc setup
    color_eyre::install().expect("Could not install error formatter");
    tracing_subscriber::fmt()
        .with_max_level(LOG_LEVEL)
        .without_time()
        .with_writer(io::stderr)
        .init();

    let input = MessageInStream::new_from_stdin();
    let output = MessageOutWriter::new_from_stdout();

    let mut echo_handler = None;
    let mut unique_id_handler = None;

    tracing::info!("Starting message handling");
    for maybe_msg in input {
        match maybe_msg {
            Err(e) => tracing::error!("Could not handle input: {e}"),
            Ok(msg) => match msg.body {
                // init
                MessageBody::Init { content, metadata } => {
                    echo_handler = Some(EchoHandler::new(content.clone()));
                    unique_id_handler = Some(UniqueIdHandler::new(content.clone()));
                    output.write(Message {
                        header: MessageHeader {
                            src: content.node_id.clone(),
                            dest: msg.header.src.clone(),
                        },
                        body: MessageBody::InitResponse {
                            metadata: MessageBodyMetadata {
                                msg_id: Some(gen_msg_id()),
                                in_reply_to: metadata.msg_id,
                            },
                        },
                    })?;
                }

                // echo workload
                MessageBody::EchoRequest { metadata, content } => {
                    let response =
                        echo_handler
                            .clone()
                            .unwrap()
                            .handle(&msg.header, &metadata, &content);
                    output.write(response)?;
                }

                // unique id workload
                MessageBody::UniqueIdRequest { metadata } => {
                    let response =
                        unique_id_handler
                            .clone()
                            .unwrap()
                            .handle(&msg.header, &metadata, &());
                    output.write(response)?;
                }

                // handle messages which we should never receive
                MessageBody::EchoResponse { .. } => {
                    panic!("Maelstrom should never send an echo response to us")
                }
                MessageBody::InitResponse { .. } => {
                    panic!("Maelstrom should never send an init response to us")
                }
                MessageBody::UniqueIdResponse { .. } => {
                    panic!("Maelstrom should never send a unique id response to us")
                }
            },
        }
    }

    Ok(())
}
