use std::{
    path::PathBuf,
    convert::TryInto,
    fs, io,
    collections::HashMap,
};

use structopt as so;
use anyhow::{
    self as ah,
    Result,
};

use crate::{
    template::{
        self as tpl,
        Lookup, Template
    },
    service::path,
    cli,
};

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

    fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
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
    config: Config,
}

#[derive(serde::Deserialize, Debug)]
pub struct Config {
    mode: Mode,
    pattern: String,
    outputs: HashMap<String, PathBuf>,
    include: Vec<String>,
    context: Option<serde_json::Value>,
}

impl std::str::FromStr for Config {
    type Err = ah::Error;

    fn from_str(s: &str) -> Result<Self> {
        let read: path::Reader = path::PathOrData::from_str(s)?.try_into()?;
        serde_json::from_reader(read).map_err(|e| e.into())
    }
}


/*
pub struct Runner<L> {
    lookup: L,
    config: Config,
    main: Main,
}


impl<L> Runner<L>
where
    L: tpl::Lookup<Key = &str>
{
    pub fn render(&mut self, key: &str, w: impl io::Write) -> Result<()> {
        let tpl = self.lookup.get(key)?;


        let res = match self.config.context {
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

    /*
    pub fn run(self) -> Result<()> {
        let write = fs::File::create(self.output)?;
        let tpl = lookup.get(config.name)?;

        
    }
    */
}
*/

impl Main {
    fn render<'a, L>(&self, lookup: &mut L, key: &'a str) -> Result<()>
    where
        L: tpl::Lookup<Key = &'a str>,
    {
        let tpl = lookup.get(key)?;
        let file = self.config.outputs.get(key)?;
        let mut write = io::BufWriter::new(fs::File::create(file)?);

        tpl.render_to(self.config.context, write)
    }
}

impl cli::Run for Main {
    type Error = ah::Error;

    fn run(self) -> Result<()> {

        Ok(())
    }
}

/*
impl std::convert::TryFrom<Main> for Runner<tpl::tera::Lookup> {
    type Error = Error;

    fn try_from(m: Main) -> ::std::result::Result<Self, Self::Error> {
        let read: path::Reader = m.config.try_into()?;
        let config: Config = serde_json::from_reader(read)?;

        let tera = tera::Tera::new(&config.pattern).map_err(
            |e| Error::Template(e.into())
        )?;

        Ok(Self {
            lookup: tpl::tera::Lookup::new(tera),
            config: config,
            main: m,
        })
    }
}
*/
