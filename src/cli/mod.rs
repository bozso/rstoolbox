use std::{
    error::Error,
};

use error_chain::Iter;

pub mod log;

pub trait Run {
    type Error;

    fn run(self) -> Result<(), Self::Error>;
}

pub trait ErrorPrint {
    fn print_error<'a>(&mut self, error: &'a dyn Error);
}

pub enum ErrorPrintStrategy {
    Top,
    Last,
    TopAndLast,
    Chain,
    Other(&'static mut dyn ErrorPrint),
}


pub type OptErr<'a> = Option<&'a(dyn Error + 'static)>;

impl ErrorPrintStrategy {
    pub fn take<'a, E: Error>(&self, error: &'a E) -> Option<&'a dyn Error>
    {
        match self {
            Self::Top | Self::Chain | Self::Other(_) => None,
            Self::Last | Self::TopAndLast => {
                Iter::new(error.source()).last()
            }
        }
    }

    pub fn print_error<E: Error>(&mut self, error: &E) {
        match self {
            Self::Top | Self::TopAndLast | Self::Last => {
                eprintln!("\n{}", error);
                if let Some(ref err) = self.take(error) {
                    eprintln!("\n\nCaused by:\t{}", err);
                }
            },
            Self::Chain => {
                eprintln!("\n{}", error);
                if let Some(cause) = error.source() {
                    eprint!("\n\nCaused by:");
                    Iter::new(Some(cause)).for_each(
                        |e| { let _ = eprintln!("\t{}", e); }
                    )
                }
            }
            Self::Other(other) => other.print_error(error),
        }
    }
}

pub struct Runner {
    strategy: ErrorPrintStrategy,
}

impl From<ErrorPrintStrategy> for Runner {
    fn from(e: ErrorPrintStrategy) -> Self {
        Self {
            strategy: e,
        }
    }
}

impl Runner {
    pub fn run<R>(&mut self, runnable: R)
    where
        R: Run,
        R::Error: Error,
    {
        match runnable.run() {
            Ok(()) => {},
            Err(e) => self.strategy.print_error(&e)
        }
    }
}
