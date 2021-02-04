use std::{
    ffi::OsStr,
};

use std::path::Path as StdPath;
use std::path::PathBuf as StdPathBuf;

use crate::{
    path::{Error, Result, valid},
};

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
    
    fn must_exist(self) -> Result<Self> {
        if !self.as_ref().exists() {
            Err(Error::InvalidPath(self.to_path_buf()))
        } else {
            Ok(self)
        }
    }

    fn to_valid(self) -> Result<valid::Path<Self>> {
        self.must_exist().map(valid::Path::new)
    }

    fn to_file(self) -> Result<valid::File<Self>> {
        self.to_valid()
            .map(|v| v.must_be_file())?
            .map(valid::File::new)
    }
    
    fn is_file(&self) -> bool {
        self.as_ref().is_file()
    }
    
    fn must_be_file(self) -> Result<Self> {
        if !self.is_file() {
            Err(Error::NotAFile(self.to_path_buf()))
        } else {
            Ok(self)
        }
    }
}

impl Like for &StdPath {}
impl Like for StdPathBuf {}
