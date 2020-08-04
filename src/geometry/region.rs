use std::cmp::PartialOrd;
use crate::geometry::{MinMax, XY};

#[derive(Debug)]
pub struct Region<T> {
    pub x: MinMax<T>,
    pub y: MinMax<T>
}

impl<T: PartialOrd> Region<T> {
    pub fn contains(&self, xy: &XY<T>) -> bool {
        return self.contains_xy(&xy.x, &xy.y)
    }

    pub fn contains_xy(&self, x: &T, y: &T) -> bool {
        self.x.in_range(&x) && self.y.in_range(&y)
    }
}
