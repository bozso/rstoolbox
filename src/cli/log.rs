use std::{
    io
};

pub enum Level {
    Info,
    Debug,
    Warning,
    Error,
    Fatal
}

pub struct Logger<W> {
    writer: W,
}

impl<W> Logger<W> {
    pub fn new(writer: W) -> Self {
        Self {
            writer: writer,
        }
    }
}

impl<W: io::Write> Logger<W> {

}
