use std::{
    path::PathBuf,
    convert::TryInto,
    fs, io,
    collections::HashMap,
};

use structopt as so;
use anyhow::{
    self as ah,
    Result
};

use crate::{
    template::{
        self as tpl,
        Lookup, Template
    },
    service::{self, path},
    cli,
    thread,
    Get
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


#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("service error")]
    Service(#[from] service::Error),
    #[error("while using serde")]
    Serde(#[from] serde_json::Error),
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
    fn render<'a, L>(&self, lookup: &mut L, key: &'a String) -> Result<()>
    where
        L: tpl::Lookup<Key = &'a str>, 
           <L as Lookup>::Error: thread::safe::Error + 'static,
           <<L as Lookup>::Tpl as Template>::Error: thread::safe::Error + 'static,
    {
        let tpl = lookup.get(key)?;
        let file = self.config.outputs.must_get(key)?;
        let write = io::BufWriter::new(fs::File::create(file)?);

        tpl.render_to(self.config.context, write).map_err(|e| {
            ah::Error::new(e)
        })
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
