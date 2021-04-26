use std::{
    io,
};

use crate::{
    handle::{attempt, Create, Error, io as hio},
};

pub enum Level {
    Info,
    Debug,
    Warning,
    Error,
    Fatal
}

pub struct Logger<W, HC> {
    writer: W,
    handler_create: HC,
}

impl<W, HC> Logger<W, HC> {
    pub fn new(writer: W, handler_create: HC) -> Self {
        Self {
            writer: writer,
            handler_create: handler_create,
        }
    }
}

impl<W, HC: Create> Logger<W, HC> 
where
    HC: Create, HC::Err: Into<io::Error>,
{
    pub fn get_handler(&mut self) -> io::Result<HC::Handler> {
        self.handler_create.create().map_err(|e| e.into())
    }

    pub fn use_handler<T, E, F>(&mut self, func: F) -> io::Result<T>
    where
        F: Fn(&mut HC::Handler) -> Result<T, E>,
        E: Into<io::Error>
    {
        func(&mut self.get_handler()?).map_err(|e| e.into())
    }
}

impl<W, HC> io::Write for Logger<W, HC> 
where
    W: hio::UnitWrite,
    HC: Create, HC::Err: Into<io::Error>,
{
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let mut h = self.get_handler()?;
        h.drain_result(|| {
            io::Write::write(&mut self.writer, buf)
        })
    }

    fn flush(&mut self) -> io::Result<()> {
        self.get_handler()?
            .drain_result(|| {
                self.writer.flush().map_err(|e| e.into())
            })
    }
}

fn test_attempt() {
    let mut l = Logger::new(
        io::stdout(), attempt::NTimes::new(5).unwrap()
    );

    let data = ['a', 'b', 'c'].iter().map(|&c| c as u8).collect::<Vec<_>>();

    hio::UnitWrite::write(&mut l, &data).unwrap();
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        super::test_attempt();
    }
}
