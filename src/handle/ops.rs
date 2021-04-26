use crate::{
    handle::{UnitError, Error, Status, self},
};

pub struct And<'a, H, K> {
    first: &'a mut H,
    second: &'a mut K,
}

impl<'a, H, K> And<'a, H, K> {
    pub fn new(one: &'a mut H, two: &'a mut K) -> Self {
        Self {
            first: one,
            second: two,
        }
    }
}

impl<'a, H: Error, K: Error> Error for And<'a, H, K>
{
    fn handle<E>(&mut self, res: &E) -> Status {
        if self.first.handle(res) == Status::Finished &&
           self.second.handle(res) == Status::Finished {
               Status::Finished
       } else {
           Status::Continue
       }
    }
}

pub struct IgnoreLast<'a, H> {
    wrap: &'a mut H,
}

impl<'a, H> IgnoreLast<'a, H> {
    pub fn new(wrap: &'a mut H) -> Self {
        Self {
            wrap: wrap,
        }
    }
}

impl<'a, H: Error> Error for IgnoreLast<'a, H>
{
    fn handle<E>(&mut self, res: &E) -> Status {
        self.wrap.handle(res)
    }
}


impl<'a, H: Error> UnitError for IgnoreLast<'a, H>
{
    fn drain_result<F, E>(&mut self, mut func: F) -> Result<(), E>
    where
        F: FnMut() -> Result<(), E>
    {
        handle::ignore_err(Error::drain_result(self, func))
    }
}

