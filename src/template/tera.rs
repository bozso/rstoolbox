use std::{
    io,
    result::Result as StdResult,
};

use serde_json::Value;
pub use tera as ttpl;
use once_cell::sync::Lazy;

use crate::{
    template as tpl,
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
