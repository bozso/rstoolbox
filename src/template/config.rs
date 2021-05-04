use std::{collections::HashMap, convert::TryInto, path::PathBuf};

use crate::{
    service::path,
    template::{self as tpl, engine},
};

#[derive(serde::Deserialize, Debug)]
pub struct Config {
    mode: tpl::engine::Mode,
    pub pattern: String,
    pub include: Option<Vec<String>>,
    pub outputs: HashMap<String, PathBuf>,
    pub context: Option<serde_json::Value>,
}

impl Config {
    pub fn to_engine(&self) -> tpl::Result<engine::Engines> {
        self.mode.to_engine(self)
    }
}

impl std::str::FromStr for Config {
    type Err = tpl::Error;

    fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
        let read: path::Reader = path::PathOrData::from_str(s)?.try_into()?;
        serde_json::from_reader(read).map_err(|e| e.into())
    }
}
