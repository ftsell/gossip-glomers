//! Handling of all relevant interactions with [Maelstrom](https://github.com/jepsen-io/maelstrom/)

mod init;
pub mod msg_protocol;
mod msg_reader;
mod msg_writer;

pub use init::{init, InitData};
pub use msg_reader::MessageReader;
pub use msg_writer::MessageWriter;
use std::sync::atomic::{AtomicUsize, Ordering};

pub fn gen_msg_id() -> usize {
    static N: AtomicUsize = AtomicUsize::new(0);
    N.fetch_add(1, Ordering::SeqCst)
}
