use std::{
    ffi::OsStr,
    process::Command,
};

pub mod binary;

pub trait Spawner {
    type Argument: AsRef<OsStr>;
    type Iter: IntoIterator<Item = Self::Argument>;

    fn spawn_command(&self, args: &Self::Iter) -> Command;
}
