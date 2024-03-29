use std::{
    cmp::PartialOrd,
};

use crate::geometry::AddSub;

#[derive(Debug)]
pub struct XY<T> {
    pub x: T,
    pub y: T
}

impl<T> Into<XY<T>> for (T, T) {
    fn into(self) -> XY<T> {
        XY {
            x: self.0,
            y: self.1,
        }
    }
}

#[derive(Debug)]
pub struct MinMax<T> {
    pub min: T,
    pub max: T
}


impl<T: AddSub<T>> MinMax<T> {
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

impl<T: PartialOrd + Copy> MinMax<T> {
    pub fn limit(&self, val: &T) -> T {
        if val <= &self.min {
            self.min
        } else if val >= &self.max {
            self.max
        } else {
            *val
        }
    }

    pub fn limit_mut(&self, val: &mut T) {
        *val = self.limit(val)
    }
}
