use crate::broadcast_workload::BroadcastStore;
use crate::maelstrom::{MessageReader, MessageWriter};
use color_eyre::eyre::eyre;
use color_eyre::Result;
use std::io;
use tracing::Level;

mod broadcast_workload;
mod echo_workload;
mod maelstrom;
mod unique_id_workload;

const LOG_LEVEL: Level = Level::DEBUG;

fn main() -> Result<()> {
    // misc setup
    color_eyre::install().expect("Could not install error formatter");
    tracing_subscriber::fmt()
        .with_max_level(LOG_LEVEL)
        .without_time()
        .with_writer(io::stderr)
        .init();

    let mut msg_reader = MessageReader::new();
    let mut msg_writer = MessageWriter::new();

    let init_data = maelstrom::init(&mut msg_reader, &mut msg_writer)?;
    let mut broadcast_store = BroadcastStore::new(init_data.clone());

    loop {
        let msg = msg_reader.read_anon()?;

        if msg.body.msg_type == echo_workload::REQ_MSG_TYPE {
            echo_workload::handle_echo(&mut msg_writer, msg.downparse()?)?
        } else if msg.body.msg_type == unique_id_workload::REQ_MSG_TYPE {
            unique_id_workload::handle_generate(&mut msg_writer, msg)?
        } else if broadcast_workload::MSG_TYPES.contains(&msg.body.msg_type.as_str()) {
            broadcast_store.handle_msg(&mut msg_writer, msg)?
        } else {
            Err(eyre!("Could not handle received message {msg:?}"))?
        }
    }
}
