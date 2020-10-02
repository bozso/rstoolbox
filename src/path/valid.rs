use std::{
    convert::AsRef,
    path::{Path},
};

use crate::{
    path::{Like},
};


pub struct Valid<T: Like> {
    like: T,
}

impl<T: Like> Valid<T> {
    pub(super) fn new(t: T) -> Self {
        Self {
            like: t,
        }
    }
}

impl <T: Like> AsRef<Path> for Valid<T> {
    fn as_ref(&self) -> &Path {
        self.like.as_ref()
    }
}


pub struct File<T: Like> {
    like: Valid<T>,
}

impl<T: Like> File<T> {
    pub(super) fn new(t: Valid<T>) -> Self {
        Self {
            like: t,
        }
    }
}

impl<T: Like> AsRef<Path> for File<T> {
    fn as_ref(&self) -> &Path {
        self.like.as_ref()
    }
}
