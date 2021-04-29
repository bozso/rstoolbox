use std::{
    io::Write,
    result::Result as StdResult,
};

use serde_json::Value;

//pub mod cli;

pub mod tera;
mod service;


#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("error occurred")]
    Any(Box<dyn std::error::Error + Sync + Send>),
    #[error("string parsing failed")]
    StringParse(#[from] std::string::FromUtf8Error),
    #[error("failed to query shared data")]
    Service(#[from] crate::service::Error),

    #[error("tera template error")]
    Tera(#[from] tera::Error)
}

impl From<tera::ttpl::Error> for Error {
    fn from(e: tera::ttpl::Error) -> Self {
        Self::Tera(tera::Error::Tera(e))
    }
}

// impl thread::safe::Safe for Error {}

pub type Result<T> = std::result::Result<T, Error>;

pub trait Context: Sized {
    type Error;
    fn from_value(obj: Value) -> StdResult<Self, Self::Error>;
}


pub trait Template {
    type Error: std::error::Error + From<std::string::FromUtf8Error>;
    type Ctx: Context;

    fn render_to(&self, ctx: Option<Value>, write: impl Write) -> StdResult<(), Self::Error>;

    fn render(&self, ctx: Option<Value>) -> StdResult<String, Self::Error> {
        let mut s = Vec::<u8>::new();
        self.render_to(ctx, &mut s)?;
        Ok(String::from_utf8(s)?)
    }
}

/*
pub trait WithContext {
    type Error;
    type Ctx: Context;

    fn render_to_impl(&self, ctx: Self::Ctx, write: impl Write) -> StdResult<(), Self::Error>;
    fn render_to(&self, obj: Value, write: impl Write) -> StdResult<(), Self::Error> {
        let ctx = Self::Ctx::from_value(obj)?;
        self.render_to_impl(ctx, write)
    }

}
*/

pub trait Lookup {
    type Key;
    type Error;
    type Tpl: Template;

    fn get(self, key: Self::Key) -> StdResult<Self::Tpl, Self::Error>;
    /*
    fn with_context<C: Context, T: Template>(&mut self, key: Self::Key, ctx: C) -> Result<T> {
        Ok(self.get(key)?.with_context(ctx))
    }
    */
}
