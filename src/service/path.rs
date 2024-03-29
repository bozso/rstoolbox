use std::{convert::TryFrom, fs, io, path::PathBuf};

use crate::service::{Error, Result};

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub enum PathOrData {
    Path(PathBuf),
    Data(String),
}

impl std::str::FromStr for PathOrData {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let meta = fs::metadata(s)?;
        if meta.is_file() {
            Ok(Self::Path(PathBuf::from_str(s)?))
        } else {
            Ok(Self::Path(s.into()))
        }
    }
}

pub enum Reader {
    File(fs::File),
    Data(String),
}

impl TryFrom<PathOrData> for Reader {
    type Error = Error;

    fn try_from(path_data: PathOrData) -> Result<Self> {
        match path_data {
            PathOrData::Path(path) => Ok(Self::File(fs::File::open(path)?)),
            PathOrData::Data(data) => Ok(Self::Data(data)),
        }
    }
}

impl std::io::Read for Reader {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self {
            Self::File(file) => file.read(buf),
            Self::Data(data) => data.as_bytes().read(buf),
        }
    }
}
