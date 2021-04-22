use std::{
    error::Error,
};

use error_chain::Iter;

pub mod log;
pub mod handle;

pub trait Run {
    type Error: Error;

    fn run(self) -> Result<(), Self::Error>;
}

pub fn run<R: Run>(runnable: R) {
    match runnable.run() {
        Ok(()) => {},
        Err(ref e) => {
            eprintln!("{}", e);
            Iter::new(e.source()).skip(1).last().map(
                |err| eprintln!("caused by: {}", err)
            );
        }
    };
}
