use crate::maelstrom::{MessageReader, MessageWriter};
use color_eyre::eyre::eyre;
use color_eyre::Result;
use std::io;
use tracing::Level;

mod echo_workload;
mod maelstrom;
mod unique_id_workload;

const LOG_LEVEL: Level = Level::INFO;

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

    loop {
        let msg = msg_reader.read_anon()?;
        match msg.body.msg_type.as_str() {
            echo_workload::REQ_MSG_TYPE => {
                echo_workload::handle_echo(&mut msg_writer, &init_data, msg.downparse()?)
            }
            unique_id_workload::REQ_MSG_TYPE => {
                unique_id_workload::handle_generate(&mut msg_writer, &init_data, msg)
            }
            _ => Err(eyre!("Could not handle received message {msg:?}")),
        }?;
    }
}
