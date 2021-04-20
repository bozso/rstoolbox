use crate::{
    thread,
};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("missing data: {0}")]
    Missing(#[from] Missing),
    #[error("failed parsing from string: {0}")]
    Parse(#[from] Box<dyn std::error::Error>),
    #[error("io error: {0}")]
    IO(#[from] std::io::Error),
    #[error("")]
    Infallible(#[from] std::convert::Infallible)
}

//impl thread::safe::Safe for Error {}

#[derive(thiserror::Error, Debug)]
pub enum Missing {
    #[error("missing shared data: {0}")]
    SharedData(&'static str),
    #[error("missing paramter: {0}")]
    Parameter(String),
}

impl thread::safe::Safe for Missing {}

impl Missing {
    pub fn shared_data<T>() -> Self {
        Self::SharedData(std::any::type_name::<T>())
    }
}
