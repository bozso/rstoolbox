use std::{
    error::Error as StdError,
};

use structopt::StructOpt;
use error_chain::Iter;

use toolbox::{
    template as tpl,
};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    Template(#[from] tpl::cli::Error),
}

#[derive(StructOpt, Debug)]
pub enum Main {
    Template(tpl::cli::Main),
}

impl Main {
    fn run(self) -> Result<(), Error> {
        match self {
            Self::Template(main) => main.run(),
        }.map_err(|e| Error::from(e))
    }
}

pub type OptErr<'a> = Option<&'a(dyn StdError + 'static)>;

pub fn run(m: Main) {
    match m.run() {
        Ok(()) => {},
        Err(ref e) => {
            eprintln!("{}", e);
            Iter::new(e.source()).skip(1).last().map(
                |err| eprintln!("caused by: {}", err)
            );
        }
    };
}


fn main() {
    run(Main::from_args());
}
