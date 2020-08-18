use std::cmp::PartialOrd;
use crate::geometry::{MinMax, XY, Arithmetic};

#[derive(Debug)]
pub struct Region<T> {
    pub x: MinMax<T>,
    pub y: MinMax<T>
}

impl<T> Region<T> {
    pub fn new(min_x: T, max_x: T, min_y: T, max_y: T) -> Self {
        Self {
            x: MinMax{min: min_x, max: max_x},
            y: MinMax{min: min_y, max: max_y},
        }
    }
}

impl<T: Arithmetic<T>> Region<T> {
    pub fn moved_xy(&self, x: T, y: T) -> Self {
        Self {
            x: self.x.shift(x),
            y: self.y.shift(y),
        }
    }

    pub fn moved(&self, xy: &XY<T>) -> Self {
        self.moved_xy(xy.x, xy.y)
    }
}

impl<T: Arithmetic<T> + Default> Region<T> {
    pub fn move_x(&self, x: T) -> Self {
        self.moved_xy(x, T::default())
    }

    pub fn move_y(&self, y: T) -> Self {
        self.moved_xy(T::default(), y)
    }
}

impl<T: PartialOrd> Region<T> {
    pub fn contains(&self, xy: &XY<T>) -> bool {
        return self.contains_xy(&xy.x, &xy.y)
    }

    pub fn contains_xy(&self, x: &T, y: &T) -> bool {
        self.x.in_range(&x) && self.y.in_range(&y)
    }
}
