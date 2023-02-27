use crate::message::Message;
use color_eyre::eyre::WrapErr;
use color_eyre::Result;
use std::io;
use std::io::{BufRead, BufReader, Cursor, Lines, Read, Write};
use std::pin::Pin;
use std::task::{Context, Poll};

#[derive(Debug)]
pub struct MessageInStream<R: BufRead> {
    stream: Lines<R>,
}

impl MessageInStream<io::StdinLock<'static>> {
    pub fn new_from_stdin() -> Self {
        Self {
            stream: io::stdin().lines(),
        }
    }
}

impl<R: BufRead> Iterator for MessageInStream<R> {
    type Item = Result<Message>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.stream.next() {
            None => None,
            Some(read_result) => match read_result {
                Err(e) => Some(Err(e.into())),
                Ok(raw_msg) => {
                    tracing::trace!("Read raw input: {raw_msg}");
                    Some(serde_json::from_str(&raw_msg).wrap_err("Could not parse json message"))
                }
            },
        }
    }
}

#[derive(Debug)]
pub struct MessageOutWriter<W: Write> {
    stream: W,
}

impl MessageOutWriter<io::Stdout> {
    pub fn new_from_stdout() -> Self {
        Self {
            stream: io::stdout(),
        }
    }
}

impl<'a, W> MessageOutWriter<W>
where
    W: Write + 'a,
    &'a W: Write,
{
    pub fn write(&'a self, message: Message) -> Result<()> {
        let serialized = serde_json::to_string(&message).wrap_err("Could not serialize message")?;
        tracing::trace!("Writing output message: {serialized}");
        writeln!(&self.stream, "{serialized}")
            .wrap_err("Could not write message into output stream")
    }
}
