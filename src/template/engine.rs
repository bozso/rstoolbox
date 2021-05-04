use std::io;

use serde_json::Value;
use structopt as so;
use tera;

use crate::template as tpl;

#[derive(so::StructOpt, serde::Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Mode {
    Tera,
}

impl std::str::FromStr for Mode {
    type Err = Unknown;

    fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
        match s {
            "tera" => Ok(Self::Tera),
            _ => Err(Unknown(s.into())),
        }
    }
}

#[derive(Debug)]
pub struct Unknown(String);

impl ToString for Unknown {
    fn to_string(&self) -> String {
        format!("unknown option: {}", self.0)
    }
}

impl Mode {
    pub fn to_engine(&self, cfg: &tpl::Config) -> tpl::Result<Engines> {
        Ok(match self {
            Self::Tera => {
                let mut t = tera::Tera::new(cfg.pattern.as_str())?;

                if let Some(ref inc) = cfg.include {
                    for path in inc.iter() {
                        t.extend(&tera::Tera::new(&path)?)?;
                    }
                }

                Engines::Tera(t)
            }
        })
    }
}

pub enum Engines {
    Tera(tera::Tera),
}

impl tpl::Engine for Engines {
    type Error = tpl::Error;
    type Key = String;

    fn render_to(
        &self,
        ctx: Option<Value>,
        key: &Self::Key,
        write: impl io::Write,
    ) -> std::result::Result<(), Self::Error> {
        match self {
            Self::Tera(ref engine) => tpl::Engine::render_to(engine, ctx, key, write),
        }
    }
}
