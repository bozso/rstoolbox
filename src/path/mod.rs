use std::{
    ffi::OsStr,
    result,
};

use thiserror::Error;
use std::path::Path as StdPath;
use std::path::PathBuf as StdPathBuf;

#[derive(Error, Debug)]
pub enum Error {
    #[error("'{0}' is an invalid path")]
    InvalidPath(StdPathBuf),
}

/*
impl<T: Like> From<&T> for Error {
    fn from(p: &T) -> Self {
        Self::InvalidPath(p.to_path_buf())
    }
}

impl<T: Like> From<T> for Error {
    fn from(p: T) -> Self {
        Self::InvalidPath(p.to_path_buf())
    }
}
*/

type Result<T> = result::Result<T, Error>;

pub trait Like : AsRef<StdPath> + Sized {
    fn extension(&self) -> Option<&OsStr> {
        self.as_ref().extension()
    }
    
    fn to_path_buf(&self) -> StdPathBuf {
        self.as_ref().to_path_buf()
    }
    
    fn join<P: AsRef<StdPath>>(&self, path: P) -> StdPathBuf {
        self.as_ref().join(path)
    }
    
    fn exists(&self) -> bool {
        self.as_ref().exists()
    }
    
    fn to_valid(self) -> Result<Valid<Self>> {
        //self.must_exist().map(Valid::new(self))?
        
        if !self.exists() {
            Err(Error::InvalidPath(self.to_path_buf()))
        } else {
            Ok(Valid::new(self))
        }
        
    }

    fn to_file(self) -> Result<valid::File<Self>> {
        Ok(valid::File::new(self.to_valid()?))
    }
    
    fn must_exist(&self) -> Result<()> {
        if !self.as_ref().exists() {
            Err(Error::InvalidPath(self.to_path_buf()))
        } else {
            Ok(())
        }
    }
    
    fn is_file(&self) -> bool {
        self.as_ref().is_file()
    }
    
    /*
    fn to_file(&self) -> Result<File, dyn 
    */
}

mod valid;

pub use valid::Valid;
