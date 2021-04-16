use std::{
    io::Write,
};

use serde_json::Value;

mod tera;
mod service;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("error occurred {0}")]
    Any(Box<dyn std::error::Error>),
    #[error("string parsing failed: {0}")]
    StringParse(#[from] std::string::FromUtf8Error),
}

pub type Result<T> = std::result::Result<T, Error>;

pub trait Context: From<Value> {}

pub trait Template {
    type Error: std::error::Error;

    fn render_to(&self, write: impl Write) -> Result<()>;
    fn with_context<C: Context>(self, ctx: C) -> Self;

    fn render(&self) -> Result<String> {
        let mut s = Vec::<u8>::new();
        self.render_to(&mut s)?;
        Ok(String::from_utf8(s)?)
    }
}

pub trait Lookup {
    type Key;
    type Tpl: Template;

    fn get(&mut self, key: &Self::Key) -> Result<Self::Tpl>;
    fn with_context<C: Context>(&mut self, key: &Self::Key, ctx: C) -> Result<Self::Tpl> {
        Ok(self.get(key)?.with_context(ctx))
    }
}
