use crate::geometry::{MinMax, XY};

#[derive(Debug)]
pub struct Region<T> {
    pub x: MinMax<T>,
    pub y: MinMax<T>
}

impl<T: PartialOrd> Region<T> {
    pub fn contains(&self, xy: &XY<T>) -> bool {
        self.x.in_range(&xy.x) && self.y.in_range(&xy.y)
    }

    pub fn contains_xy(&self, x: &T, y: &T) -> bool {
        self.x.in_range(&x) && self.y.in_range(&y)
    }
}
