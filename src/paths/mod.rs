use std::{
    convert::AsRef,
    ffi::OsStr
};

pub trait Like : AsRef<OsStr> + ?Sized {}

mod path;
mod file;
mod storage;

pub use path::Path;
pub use storage::Storage;
