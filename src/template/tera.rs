use std::io;

use once_cell::sync::Lazy;
use serde_json::Value;

pub use tera as ttpl;

use crate::template as tpl;

impl tpl::Engine for ttpl::Tera {
    type Key = String;
    type Error = tpl::Error;

    fn render_to(
        &self,
        ctx: Option<Value>,
        key: &Self::Key,
        write: impl io::Write,
    ) -> Result<(), Self::Error> {
        let res = match ctx {
            Some(val) => self.render_to(key, &tera::Context::from_value(val)?, write),
            None => self.render_to(key, empty_context(), write),
        };
        res.map_err(|e| Error::Tera(e).into())
    }
}

static EMPTY_CONTEXT: Lazy<tera::Context> = Lazy::new(|| tera::Context::new());

pub fn empty_context() -> &'static tera::Context {
    Lazy::force(&EMPTY_CONTEXT)
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("tera error: {0}")]
    Tera(#[from] tera::Error),
    #[error("while parsing string: {0}")]
    Utf8Parse(#[from] std::string::FromUtf8Error),
    #[error("service error")]
    Service(#[from] crate::service::Error),
    #[error("while using serde")]
    Serde(#[from] serde_json::Error),
}
