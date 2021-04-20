use std::{
    str::FromStr,
};

use routerify as rf;
use hyper as hp;

use crate::{
    thread,
};

mod error;
pub mod path;
pub mod remote_file;
pub use error::Error;

type Result<T> = std::result::Result<T, Error>;

pub trait RequestFns: rf::ext::RequestExt {
    fn must_data<T: thread::safe::Static>(&self) -> Result<&T> {
        self.data::<T>().ok_or(error::Missing::shared_data::<T>().into())
    }

    fn must_param<T: Into<String>>(&self, name: T) -> Result<&String> {
        let s = name.into();
        self.param(s.clone()).ok_or(error::Missing::Parameter(s).into())
    }

    fn must_get<S, T>(&self, name: S) -> Result<T>
    where
        S: Into<String>,
        T: FromStr, <T as FromStr>::Err: std::error::Error + 'static
    {
        let param = self.must_param(name)?;

        T::from_str(param.as_str()).map_err(|e| Error::Parse(Box::new(e)))
    }
}

impl RequestFns for hp::Request<hp::Body> {}
