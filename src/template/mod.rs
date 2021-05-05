use std::{io, result::Result as StdResult};

use routerify as rf;
use serde_json::Value;

pub mod cli;
pub mod config;
pub mod engine;
pub mod tera;

pub use config::Config;

mod service;

use crate::{service as serv, KeyError};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("io error")]
    IO(#[from] io::Error),

    #[error("key error")]
    Key(#[from] KeyError),

    #[error("string parsing failed")]
    StringParse(#[from] std::string::FromUtf8Error),

    #[error("failed to query shared data")]
    Service(#[from] serv::Error),

    #[error("tera template error")]
    Tera(#[from] tera::Error),

    #[error("serde error")]
    Serde(#[from] serde_json::Error),

    #[error("routing error")]
    Router(#[from] rf::Error),
}

unsafe impl Send for Error {}
unsafe impl Sync for Error {}

impl From<tera::ttpl::Error> for Error {
    fn from(e: tera::ttpl::Error) -> Self {
        Self::Tera(tera::Error::Tera(e))
    }
}

// impl thread::safe::Safe for Error {}

pub type Result<T> = std::result::Result<T, Error>;

pub trait Engine {
    type Error: std::error::Error + From<std::string::FromUtf8Error>;
    type Key;

    fn render_to(
        &self,
        ctx: Option<Value>,
        key: &Self::Key,
        write: impl io::Write,
    ) -> StdResult<(), Self::Error>;

    fn render(&self, key: &Self::Key, ctx: Option<Value>) -> StdResult<String, Self::Error> {
        let mut s = Vec::<u8>::new();
        self.render_to(ctx, key, &mut s)?;
        Ok(String::from_utf8(s)?)
    }
}
