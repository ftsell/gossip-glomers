mod echo_handler;
mod handler;

pub use echo_handler::EchoHandler;
pub use handler::MessageHandler;
use std::sync::atomic::{AtomicUsize, Ordering};

pub fn gen_msg_id() -> usize {
    static N: AtomicUsize = AtomicUsize::new(0);
    N.fetch_add(1, Ordering::SeqCst)
}
