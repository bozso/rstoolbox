/*
use std::{
    convert::AsRef,
    path::Path,
};

use crate::{
    path::Like,
};

pub struct File<T: Like> {
    wrapped: T,
}

impl<T: Like> File<T> {
    
}

impl<T: Like> AsRef<Path> for File<T> {
    fn as_ref(&self) -> &Path {
        self.wrapped.as_ref()
    }
}

impl<T: Like> Like for File<T> {}
*/
