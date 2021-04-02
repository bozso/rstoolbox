use std::{
    path::PathBuf,
};

pub type Symlinks = Option<Vec<Symlink>>;

pub struct Downloaded<'a> {
    pub root: PathBuf,
    pub symlinks: &'a Symlinks,
}

pub struct Symlink {
    pub source: PathBuf,
    pub target: PathBuf,
}

pub struct Task {
    pub url: String,
    pub target: PathBuf,
    pub symlinks: Symlinks,
}
