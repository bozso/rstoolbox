use std::{
    path::PathBuf,
    convert::TryInto,
};

use structopt as so;

use crate::{
    template as tpl,
    service::path,
};

#[derive(so::StructOpt, serde::Deserialize, Debug)]
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

#[derive(so::StructOpt)]
pub struct Main {
    #[structopt(short = "o", long="output")]
    output: PathBuf,
    #[structopt(short = "c", long="config")]
    config: path::PathOrData,
}

pub trait User {
    type Error;

    fn use_lookup<L: tpl::Lookup>(&self, lookup: L) -> Result<(), Self::Error>;
}

#[derive(serde::Deserialize, Debug)]
pub struct Config {
    mode: Mode,
    pattern: String,
    name: String,
    context: serde_json::Value,
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

impl<'a> User for Render<'a> {
    type Error = tpl::Error;

    fn use_lookup<L: tpl::Lookup>(&self, lookup: L) -> Result<(), Self::Error> {
        lookup.get(self.config.name)?
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("serde error: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("service error: {0}")]
    Service(#[from] crate::service::Error),

    #[error("template error: {0}")]
    Template(#[from] tpl::Error),
}


impl Main {
    pub fn run(self) -> Result<(), Error> {
        let read: path::Reader = self.config.try_into()?;

        let config: Config = serde_json::from_reader(read)?;

        config.use_lookup(
        )?;

        Ok(())
    }
}
