use crate::maelstrom::msg_protocol::MaelstromMessage;
use color_eyre::eyre::{Result, WrapErr};
use serde::Serialize;
use std::io::{stdout, StdoutLock, Write};

#[derive(Debug)]
pub struct MessageWriter<'a> {
    stdout: StdoutLock<'a>,
}

impl MessageWriter<'_> {
    pub fn new() -> Self {
        Self {
            stdout: stdout().lock(),
        }
    }

    pub fn write<M>(&mut self, msg: MaelstromMessage<M>) -> Result<()>
    where
        M: Serialize,
    {
        let raw = serde_json::to_string(&msg).wrap_err("Could not serialize message")?;
        tracing::trace!("Sending message {raw}");

        writeln!(self.stdout, "{}", raw).wrap_err("Could not write message to stdout")
    }
}
