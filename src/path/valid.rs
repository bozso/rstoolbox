use std::{
    convert::AsRef,
};

use std::path::Path as StdPath;

use crate::{
    path::{Like},
};


pub struct Path<T: Like> {
    like: T,
}

impl<T: Like> Path<T> {
    pub(super) fn new(t: T) -> Self {
        Self {
            like: t,
        }
    }
    
    pub fn as_like(self) -> T {
        self.like
    }
}

impl <T: Like> AsRef<StdPath> for Path<T> {
    fn as_ref(&self) -> &StdPath {
        self.like.as_ref()
    }
}


impl <T: Like> Like for Path<T> {}

pub struct File<T: Like> {
    like: Path<T>,
}

impl<T: Like> File<T> {
    pub(super) fn new(t: Path<T>) -> Self {
        Self {
            like: t,
        }
    }
}

impl<T: Like> AsRef<StdPath> for File<T> {
    fn as_ref(&self) -> &StdPath {
        self.like.as_ref()
    }
}

impl<T: Like> Like for File<T> {}
