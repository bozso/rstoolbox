use structopt::StructOpt;

use toolbox::{cli, template as tpl};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("error while compiling templates")]
    Template(#[from] tpl::Error),
}

#[derive(StructOpt, Debug)]
pub enum Main {
    Template(tpl::cli::Main),
}

impl cli::Run for Main {
    type Error = Error;

    fn run(&self) -> Result<(), Error> {
        match self {
            Self::Template(main) => main.run(),
        }
        .map_err(|e| Error::from(e))
    }
}

fn main() {
    let a;
    let mut runner: cli::Runner = cli::ErrorPrintStrategy::TopAndLast.into();

    runner.run(Main::from_args());
}
