use std::{
    path::PathBuf,
    convert::TryInto,
    fs, io
};

use structopt as so;

use crate::{
    template::{
        self as tpl,
        Lookup, Template
    },
    service::path,
};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("serde error: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("service error: {0}")]
    Service(#[from] crate::service::Error),

    #[error("template error: {0}")]
    Template(#[from] tpl::Error),

    #[error("io error: {0}")]
    IO(#[from] io::Error),
}

impl From<tera::Error> for Error {
    fn from(e: tera::Error) -> Self {
        Self::Template(tpl::Error::from(e))
    }
}

#[derive(so::StructOpt, serde::Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Mode {
    Tera,
}

#[derive(Debug)]
pub struct Unknown(String);

impl ToString for Unknown {
    fn to_string(&self) -> String {
        format!("unknown option: {}", self.0)
    }
}

impl std::str::FromStr for Mode {
    type Err = Unknown;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "tera" => Ok(Self::Tera),
            _ => Err(Unknown(s.into()))
        }
    }
}

#[derive(so::StructOpt, Debug)]
pub struct Main {
    #[structopt(short = "o", long="output")]
    output: PathBuf,
    #[structopt(short = "c", long="config")]
    config: path::PathOrData,
}

#[derive(serde::Deserialize, Debug)]
pub struct Config {
    mode: Mode,
    pattern: String,
    name: String,
    context: Option<serde_json::Value>,
}

/*
pub trait User {
    type Error;

    fn use_lookup<L: tpl::Lookup>(&self, lookup: L) -> Result<(), Self::Error>;
}


impl Config {
    pub fn use_lookup<'a, U>(&'a self, user: U) -> Result<(), U::Error>
    where
        U: User, <U as User>::Error: From<tera::Error>
    {
        user.use_lookup(
            tpl::tera::Lookup::new(&tera::Tera::new(&self.pattern)?)
        )
    }
}

struct Render<'a> {
    config: &'a Config,
}

impl User for Main {
    type Error = Error;

    fn use_lookup<L>(&self, lookup: L) -> Result<(), Self::Error>
    where
        L: tpl::Lookup<Key = std::string::String>,
    {
        let writer = fs::File::create(self.output)?;

        let read: path::Reader = self.config.try_into()?;
        let config: Config = serde_json::from_reader(read)?;

        let ctx = self.config.context.map(
            |ctx| <<L as tpl::Lookup>::Tpl as tpl::Template>::ctx_from_value(ctx)
        ).unwrap_or_else(
            tera::empty_context()
        )



        lookup.get(self.config.name)?.render_to(ctx, writer)
    }
}
*/

impl Main {
    pub fn run(self) -> Result<(), Error> {
        let read: path::Reader = self.config.try_into()?;
        let config: Config = serde_json::from_reader(read)?;

        let tera = &tera::Tera::new(&config.pattern).map_err(
            |e| Error::Template(e.into())
        )?;

        let mut lookup = tpl::tera::Lookup::new(tera);
        let write = fs::File::create(self.output)?;
        let tpl = lookup.get(config.name)?;

        let res = match config.context {
            Some(ctx) => {
                tpl.render_to(&tera::Context::from_value(ctx)?, write)                
            },
            None => {
                tpl.render_to(tpl::tera::empty_context(), write)                
            }
        };
        
        res.map_err(
            |e| Error::Template(tpl::Error::from(e))
        )
    }
}
