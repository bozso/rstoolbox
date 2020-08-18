use std::{
    path
};

use crate::{
    paths::Like
};

pub struct Path<S: Like> {
    wrapped: path::Path,
}

impl<S: Like> Path<S> {
    pub fn new(s &S) -> &Self {
        &Self {
            
        }
    }    
}
