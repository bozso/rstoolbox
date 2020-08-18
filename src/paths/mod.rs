use std::{
    convert::AsRef,
    ffi::OsStr
};

pub trait Like : AsRef<OsStr> + ?Sized {}

mod path;
mod file;

pub use path::Path;
