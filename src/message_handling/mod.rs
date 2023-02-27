mod echo_handler;
mod handler;
mod unique_id_handler;

pub use echo_handler::EchoHandler;
pub use handler::MessageHandler;
use std::sync::atomic::{AtomicUsize, Ordering};
pub use unique_id_handler::UniqueIdHandler;

pub fn gen_msg_id() -> usize {
    static N: AtomicUsize = AtomicUsize::new(0);
    N.fetch_add(1, Ordering::SeqCst)
}
