use std::{
    ops::{Sub, Add},
};

mod common;
mod region;

pub trait Arithmetic<T> : 
    Sub<Output = T> +
    Add<Output = T> +
    Copy {}

pub use common::{XY, MinMax};
pub use region::{Region};
