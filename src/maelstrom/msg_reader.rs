use crate::maelstrom::msg_protocol::{AnonMessage, MaelstromMessage};
use color_eyre::eyre::{Result, WrapErr};
use serde::Deserialize;
use std::io::{stdin, BufRead, StdinLock};

pub struct MessageReader<'a> {
    stdin: StdinLock<'a>,
}

impl MessageReader<'_> {
    pub fn new() -> Self {
        Self {
            stdin: stdin().lock(),
        }
    }

    pub fn read_anon(&mut self) -> Result<AnonMessage> {
        self.read()
    }

    pub fn read<M>(&mut self) -> Result<MaelstromMessage<M>>
    where
        M: for<'a> Deserialize<'a>,
    {
        let mut raw = String::with_capacity(512);
        self.stdin
            .read_line(&mut raw)
            .wrap_err("Could not read line from stdin")?;
        tracing::trace!("read line from stdin: {raw}");

        serde_json::from_str(&raw).wrap_err("Could not parse message")
    }
}
