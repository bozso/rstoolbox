use std::cmp::PartialOrd;

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

impl<T: PartialOrd> MinMax<T> {
    pub fn in_range(&self, val: &T) -> bool {
        &self.min >= val && val <= &self.max 
    }
}
