use std::{
    sync::{Mutex, LockResult},
    process::Command,
    result::Result,
};

use serde::{Serialize};
use once_cell::sync::Lazy;

static GIT: Lazy<Mutex<Command>> = Lazy::new(|| {
    Mutex::new(Command::new("git"))
});


#[derive(Debug,Serialize)]
pub struct Info {
    branch: String,
    log: String
}

impl Info {
    pub fn in_current() -> LockResult<Self> {
        let g = GIT.lock()?;
        
        g.arg("");
        
        Ok(Self {
            branch: "".to_string(),
            log: "".to_string(),
        })
    }
    
    /*
    pub fn in_folder(path: Path) -> Self {
        
    }
    */
}
