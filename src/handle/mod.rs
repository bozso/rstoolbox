pub mod attempt;
pub mod ops;
pub mod delay;

#[derive(PartialEq)]
pub enum Status {
    Continue,
    Finished
}

pub trait Error: Sized {
    fn handle<T, E>(&mut self, result: &Result<T, E>) -> Status;

    fn drain_result<T, E, F>(&mut self, func: F) -> Result<T, E>
    where
        F: Fn() -> Result<T, E>
    {
        loop {
            let res = func();

            if self.handle(&res) == Status::Finished {
                return res;
            }
        }
    }
}

pub trait Create {
    type Handler: Error;
    type Err: std::error::Error;

    fn create_handler(&self) -> Result<Self::Handler, Self::Err>;
}
