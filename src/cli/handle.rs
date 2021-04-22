use std::{
    io,
    result::Result as StdResult,
};

pub enum Status {
    Continue,
    Finished
}

pub trait Result {
    fn handle<T, E>(&self, result: StdResult<T, E>) -> Status;

    fn exhaust<T, E, F>(&self, func: F) -> StdResult<T, E> 
    where
        F: Fn() -> StdResult<T, E>;
    {
        loop {
            let res = func()

            if self.try(res) == Finished {
                return
            }
        }
    }
}
