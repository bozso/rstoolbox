use std::{
    convert::{TryFrom, TryInto},
    fs, io,
};

use structopt as so;

use crate::{
    cli,
    service::{self},
    template::{self as tpl, Engine},
    Get,
};

#[derive(so::StructOpt, Debug)]
pub struct Main {
    #[structopt(short = "c", long = "config")]
    config: tpl::Config,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("service error")]
    Service(#[from] service::Error),
    #[error("while using serde")]
    Serde(#[from] serde_json::Error),
}

pub struct Runner<'a> {
    main: &'a Main,
    engine: tpl::engine::Engines,
}

impl<'a> Runner<'a> {
    fn render(&self, key: &String) -> tpl::Result<()> {
        let file = self.main.config.outputs.must_get(key)?;
        let write = io::BufWriter::new(fs::File::create(file)?);

        self.engine
            .render_to(self.main.config.context.clone(), key, write)
    }
}

impl<'a> TryFrom<&'a Main> for Runner<'a> {
    type Error = tpl::Error;

    fn try_from(main: &'a Main) -> Result<Self, Self::Error> {
        Ok(Self {
            main: main,
            engine: main.config.to_engine()?,
        })
    }
}

impl cli::Run for Main {
    type Error = tpl::Error;

    fn run(&self) -> Result<(), Self::Error> {
        let r: Runner = self.try_into()?;

        for key in self.config.outputs.keys() {
            r.render(key)?;
        }

        Ok(())
    }
}
