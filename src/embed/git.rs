use std::{
    process::Command,
    io,
    result
};

use serde::{Serialize};
use once_cell::sync::Lazy;
use thiserror::Error as TError;

#[derive(Debug, TError)]
pub enum Error {
    #[error("io error occurred")]
    IOError(#[from] io::Error),
    
    #[error("empty output recevied from command")]
    EmptyOutput,
}

pub type Result<T> = result::Result<T, Error>;

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
}

static GIT: Lazy<Binary> = Lazy::new(|| { Binary::new("git", " ")});

#[derive(Debug, Serialize)]
pub struct Info {
    branch: String,
    log: String
}

const AT: u8 = '@' as u8;

impl Info {
    pub fn in_current() -> Result<Self> {
        let url = GIT.args("config --get remote.origin.url").output()?;
        let url = url.stdout
                     .split(|c| *c == AT)
                     .next().ok_or(Error::EmptyOutput)?;
        
        
        Ok(Self {
            branch: format!("https://{:?}", &url),
            log: "".to_string(),
        })
    }
    
    /*
    pub fn in_folder(path: Path) -> Self {
        
    }
    */
}
