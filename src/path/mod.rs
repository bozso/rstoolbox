use std::{
    result,
    path::PathBuf,
    cmp::PartialEq
};

use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum Error {
    #[error("'{0}' is an invalid path")]
    InvalidPath(PathBuf),
    #[error("'{0}' is not a file")]
    NotAFile(PathBuf),
}

type Result<T> = result::Result<T, Error>;

pub mod valid;
mod like;

pub use like::Like;

#[cfg(test)]
mod tests {
    use std::path::Path;
    use crate::testing::{Case, Cases, Test};
    
    use super::*;
    
    struct MustExist<P: Like> {
        path: P,
    }
    
    impl<P: Like + Copy> Test<P, Error> for MustExist<P> {
        fn test(&self) -> Result<P> {
            self.path.must_exist()
        }
    }
    
    #[test]
    fn test_invalid() {
        let p = &[
            Path::new("does_not_exists.rs"),
            Path::new("mod.rs"),
        ];
        
        Cases::new(
        &[
            Case{
                test: MustExist{path: p[0]},
                //expected: Err(Error::InvalidPath(p[0].to_path_buf())),
                expected: Ok(&p[1]),
            },
            
            Case{
                test: MustExist{path: p[1]},
                //expected: Err(Error::InvalidPath(p[1].to_path_buf())),
                expected: Ok(&p[1]),
            },
        ],
        ).run();
    }
}
