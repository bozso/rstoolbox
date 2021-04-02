use std::{
    convert::AsRef,
    path::Path,
    process::Command,
};

#[derive(Clone, Copy)]
pub struct Binary<'a> {
    binary_name: &'a str,
    separator: &'a str,
}

impl<'a> Binary<'a> {
    pub fn new(binary_name: &'a str, separator: &'a str) -> Self {
        Self {
            binary_name: binary_name,
            separator: separator,
        }
    }
    
    pub fn args(&self, args: &str) -> Command {
        let mut c = Command::new(self.binary_name);
        c.args(args.split(self.separator));
        
        c
    }
    
    pub fn in_path<P: AsRef<Path>>(self, path: P) -> InPath<'a, P> {
        InPath {
            bin: self,
            path: path,
        }
    }
}

pub struct InPath<'a, P> {
    bin: Binary<'a>,
    path: P,
}

impl <'a, P: AsRef<Path>> InPath<'a, P> {
    pub fn args(&self, args: &str) -> Command {
        let mut c = self.bin.args(args);
        c.current_dir(&self.path);
        c
    }
}
