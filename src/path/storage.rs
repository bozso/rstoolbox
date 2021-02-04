use std::{
    convert::From,
    //ops::Index,
};

pub enum Storage<'a, T> {
    SliceRef(&'a[T]),
    Vec(Vec<T>),
}

impl<'a, T> From<Vec<T>> for Storage<'a, T> {
    fn from(v: Vec<T>) -> Self {
        Self::Vec(v)
    }
}

impl<'a, T> From<&'a[T]> for Storage<'a, T> {
    fn from(s: &'a [T]) -> Self {
        Self::SliceRef(s)
    }
}

/*
impl<'a, T> Index<> for Storage<'a, T> {
    type Output = T;
    
    fn index(&self, index: ) -> &Self::Output {
        match self {
            SliceRef(ref slice) => slice[index],
            Vec(ref vec) => vec[index],
        }
    }
}
*/
