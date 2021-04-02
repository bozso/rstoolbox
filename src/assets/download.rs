use std::{
    path::PathBuf,
};

use crate::assets as ast;

pub trait Downloader {
    fn download(&mut self, uri: &str, target: PathBuf) -> ast::Result<()>; 
}
