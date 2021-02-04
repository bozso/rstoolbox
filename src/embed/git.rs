use std::{
    io,
    result,
    str::{from_utf8, Utf8Error},
    string::FromUtf8Error,
    path::Path,
};

use serde::{Serialize};
use once_cell::sync::Lazy;
use thiserror::Error as TError;
use crate::command::binary::Binary;

#[derive(Debug, TError)]
pub enum Error {
    #[error("io error occurred")]
    IOError(#[from] io::Error),
    
    #[error("error occurred during conversion of bytes to string")]
    Utf8Error(#[from] Utf8Error),

    #[error("error occurred during conversion of bytes to string")]
    FromUtf8Error(#[from] FromUtf8Error),    
    
    #[error("empty output recevied from command")]
    EmptyOutput,
}

pub type Result<T> = result::Result<T, Error>;

static GIT: Lazy<Binary> = Lazy::new(|| { Binary::new("git", " ")});

#[derive(Debug, Serialize)]
pub struct Info {
    branch: String,
    log: String,
    url: String,
    revision: String,
}

const AT: u8 = '@' as u8;
const NEWLINE: u8 = '\n' as u8;

impl Info {
    pub fn in_path<P: AsRef<Path>>(path: P) -> Result<Self> {
        let g = GIT.in_path(path);
        
        let url = g.args("config --get remote.origin.url").output()?;
        let url = url.stdout
                     .split(|&c| c == AT)
                     .next().ok_or(Error::EmptyOutput)?;
        
        
        let branch = g.args("branch").output()?;
        let branch = branch.stdout
            .split(|&c| c == NEWLINE)
            .filter(|&line| line.starts_with(b"*"))
            .next().ok_or(Error::EmptyOutput)?;
        
        Ok(Self {
            url: format!("https://{:?}", &url),
            branch: from_utf8(&branch)?.to_owned(),
            
            revision: String::from_utf8(
                g.args("log -1 --pretty=\"%H\"").output()?.stdout)?,
            
            log: String::from_utf8(
                g.args("log -1 --pretty=\"log[%h]: %an (%ae) | %ad\" --date=iso")
                   .output()?.stdout)?,
        })
    }
    
    pub fn in_current() -> Result<Self> {
        Self::in_path(".")
    }
}
