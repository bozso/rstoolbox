use routerify as rf;
use hyper as hp;

use crate::{
    ThreadStatic,
};

pub mod remote_file;

#[derive(Debug)]
pub struct MissingSharedData {
    pub name: &'static str,
}

impl MissingSharedData {
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
        }
    }

    pub fn from_type<T>() -> Self {
        Self::new(std::any::type_name::<T>())
    }
}

impl std::fmt::Display for MissingSharedData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "data type '{:?}' not found for current service",
               self.name)
    }
}

impl std::error::Error for MissingSharedData {
}

pub trait RequestFns: rf::ext::RequestExt {
    fn must_data<T: ThreadStatic>(&self) -> Result<&T, MissingSharedData> {
        self.data::<T>().ok_or(MissingSharedData::from_type::<T>())
    }
}

impl RequestFns for hp::Request<hp::Body> {}
