use std::{
    fmt::Write,
};

use crate::template as tpl;
use crate::template::Template;

pub struct Service<L> {
    lookup: L,
}

impl<L> Service<L> {
    pub fn new(lookup: L) -> Self {
        Self {
            lookup: lookup,
        }
    }
}

impl<L: tpl::Lookup> Service<L> {
    pub fn render_to(&mut self, key: &L::Key, write: impl Write) -> tpl::Result<()> {
        let tpl = self.lookup.get(key)?;

        tpl.render_to(write)?;

        Ok(())
    }
}
