use std::io;

use crate::handle;

pub trait UnitWrite: io::Write {
    fn write(&mut self, data: &[u8]) -> io::Result<()> {
        handle::to_unit_err(io::Write::write(self, data))
    }
}

impl<W: io::Write> UnitWrite for W {}
