pub mod attempt;
pub mod ops;
pub mod delay;
pub mod io;

#[derive(PartialEq)]
pub enum Status {
    Continue,
    Finished
}

pub fn ignore_err<E>(res: Result<(), E>) -> Result<(), E> {
    match res {
        Ok(ok) => res,
        Err(ref err) => Ok(())
    }
}

pub fn to_unit_err<T, E>(res: Result<T, E>) -> Result<(), E> {
    match res {
        Ok(ok) => Ok(()),
        Err(err) => Err(err),
    }
}

pub trait Error {
    fn handle<E>(&mut self, result: &E) -> Status;

    fn drain_result<T, E, F>(&mut self, mut func: F) -> Result<T, E>
    where
        F: FnMut() -> Result<T, E>
    {
        loop {
            let res = func();

            match res {
                Ok(ok) => { return Ok(ok); }
                Err(ref err) => {
                    if self.handle(err) == Status::Finished {
                        return res;
                    }
                }
            }
        }
    }

    fn drain_ignore<F, E>(&mut self, mut func: F) -> Result<(), E>
    where
        F: FnMut() -> Result<(), E>
    {
        ignore_err(self.drain_result(func))
    }
}

pub trait UnitError: Error {
    fn drain_result<F, E>(&mut self, mut func: F) -> Result<(), E>
    where
        F: FnMut() -> Result<(), E>
    {
        Error::drain_result(self, func)
    }
}

pub trait Create {
    type Handler: Error;
    type Err: std::error::Error;

    fn create(&self) -> Result<Self::Handler, Self::Err>;
}
