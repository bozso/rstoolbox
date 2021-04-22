use std::{
    time::{Duration},
    marker::PhantomData,
};

use crate::{
    handle::{Error, Status, Create},
};

pub struct And<'a, H, K> {
    first: &'a mut H,
    second: &'a mut K,
}

impl<'a, H: Error, K: Error> Error for And<'a, H, K> {
    fn handle<T, E>(&mut self, res: &Result<T, E>) -> Status {
        if self.first.handle(res) == Status::Finished &&
           self.second.handle(res) == Status::Finished {
               Status::Finished
       } else {
           Status::Continue
       }
    }
}

