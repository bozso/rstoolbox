use std::{
    cmp::PartialOrd,
};

use crate::geometry::Arithmetic;

#[derive(Debug)]
pub struct XY<T> {
    pub x: T,
    pub y: T
}

#[derive(Debug)]
pub struct MinMax<T> {
    pub min: T,
    pub max: T
}


impl<T: Arithmetic<T>> MinMax<T> {
    pub fn extend(&self, val: T) -> Self {
        Self {
            min: self.min - val,
            max: self.max + val
        }
    }

    pub fn shift(&self, val: T) -> Self {
        Self {
            min: self.min + val,
            max: self.max + val
        }
    }
}

impl<T: PartialOrd> MinMax<T> {
    pub fn in_range(&self, val: &T) -> bool {
        &self.min >= val && val <= &self.max 
    }
}
