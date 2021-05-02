use std::path::{Path, PathBuf};

use crate::assets::Service;

pub struct Paths {
    cache: PathBuf,
    binaries: PathBuf,
}

impl Paths {
    pub fn get_root<P: AsRef<Path>>(&self, path: P) -> PathBuf {
        let r = path.as_ref();

        if r.is_relative() {
            self.cache.join(path)
        } else {
            r.to_path_buf()
        }
    }
}

pub struct Config {
    pub paths: Paths,
}

impl Config {
    pub fn to_service<D, E>(self, downloader: D, extractor: E) -> Service<D, E> {
        Service {
            down: downloader,
            extract: extractor,
            config: self,
        }
    }
}
