use std::{
    ops::{Sub, Add},
};

mod common;
mod region;

pub trait AddSub<T> : 
    Add<Output = T> +
    Sub<Output = T> +
    Copy {}

pub use common::{XY, MinMax};
pub use region::{Region};
