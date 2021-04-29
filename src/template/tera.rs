use std::{
    io, fs,
    result::Result as StdResult,
    collections::HashMap,
    path::PathBuf,
    convert::TryInto,
};

use structopt as so;
use serde_json::Value;
use once_cell::sync::Lazy;
use anyhow as ah;

pub use tera as ttpl;

use crate::{
    template as tpl,
    service::{self, path},
    cli, Get
};


pub struct Lookup {
    engine: tera::Tera,
}

impl Lookup {
    pub fn new(engine: tera::Tera) -> Self {
        Self {
            engine: engine,
        }
    }
}

impl<'a> tpl::Lookup for &'a mut Lookup {
    type Key = String;
    type Error = tpl::Error;
    type Tpl = Template<'a>;

    fn get(self, name: Self::Key) -> tpl::Result<Self::Tpl>
    {
        Ok(Template{
            name: name, lookup: &self.engine
        })
    }
}

static EMPTY_CONTEXT: Lazy<tera::Context> = Lazy::new(|| {
    tera::Context::new()
});

pub fn empty_context() -> &'static tera::Context {
    Lazy::force(&EMPTY_CONTEXT)
}


pub struct Template<'a> {
    lookup: &'a tera::Tera,
    name: String,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("tera error: {0}")]
    Tera(#[from] tera::Error),
    #[error("while parsing string: {0}")]
    Utf8Parse(#[from] std::string::FromUtf8Error),
    #[error("service error")]
    Service(#[from] service::Error),
    #[error("while using serde")]
    Serde(#[from] serde_json::Error),
}

pub struct WithContext<'a> {
    tpl: Template<'a>,
}

impl tpl::Context for tera::Context {
    type Error = ttpl::Error;

    fn from_value(obj: Value) -> StdResult<Self, Self::Error> {
        tera::Context::from_value(obj)
    }
}

/*
impl<'a> tpl::WithContext for WithContext<'a> {
    type Error = tera::Error;
    type Ctx= tera::Context;

    fn render_to_impl(&self, ctx: Self::Ctx, write: impl io::Write) -> StdResult<(), Self::Error> {
        self.tpl.lookup.engine.render_to(self.tpl.name, ctx, write)
    }
}
*/

impl<'a> tpl::Template for Template<'a> {
    type Error = Error;
    type Ctx = ttpl::Context;

    fn render_to(&self, ctx: Option<Value>, write: impl io::Write) -> Result<(), Self::Error> {
        let lk = &self.lookup;

        let res = match ctx {
            Some(val) => {
                lk.render_to(self.name.as_str(), 
                             &tera::Context::from_value(val)?,  write)
            }
            None => {
                lk.render_to(self.name.as_str(), empty_context(), write)
            }
        };
        res.map_err(|e| Error::Tera(e))
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct Config {
    pattern: String,
    outputs: HashMap<String, PathBuf>,
    include: Vec<String>,
    context: Option<serde_json::Value>,
}

impl std::str::FromStr for Config {
    type Err = Error;

    fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
        let read: path::Reader = path::PathOrData::from_str(s)?.try_into()?;
        serde_json::from_reader(read).map_err(|e| e.into())
    }
}

#[derive(so::StructOpt, Debug)]
pub struct Main {
    #[structopt(short = "o", long="output")]
    output: PathBuf,
    #[structopt(short = "c", long="config")]
    config: Config,
}

impl Main {
    fn render_to(&self, t: &ttpl::Tera, name: &String, val: &Option<Value>) -> ah::Result<()> {
        let file = self.config.outputs.must_get(name)?;
        let write = io::BufWriter::new(fs::File::create(file)?);

        let res = match val {
            Some(v) => {
                t.render_to(name, 
                             &tera::Context::from_value(v.clone())?,  write)
            }
            None => {
                t.render_to(name, empty_context(), write)
            }
        };
        res.map_err(|e| e.into())
    }
}

impl cli::Run for Main {
    type Error = ah::Error;


    fn run(self) -> ah::Result<()> {
        let tpls = ttpl::Tera::new(self.config.pattern.as_str())?;

        for key in self.config.outputs.keys() {
            self.render_to(&tpls, key, &self.config.context)?
        }

        Ok(())
    }
}

