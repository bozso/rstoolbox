use std::{
    time::{Duration},
    marker::PhantomData,
};


use crate::{
    handle::{Create, Error, Status},
};

pub trait Scaler {
    type Type;

    fn scale(&self, var: Self::Type) -> Self::Type;

}

pub struct NoScale<T> {
    mark: PhantomData<T>
}

impl<T> NoScale<T> {
    pub fn new() -> Self {
        Self {
            mark: PhantomData,
        }
    }
}

impl<T> Scaler for NoScale<T> {
    type Type = T;

    fn scale(&self, var: Self::Type) -> Self::Type {
        var
    }
}

impl<T> Default for NoScale<T> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Delay<S> {
    delay: Duration,
    scaler: S,
}

impl<S> Delay<S> {
    pub fn new(delay: Duration, scaler: S) -> Self {
        Self {
            delay: delay,
            scaler: scaler,
        }
    }
}

impl Delay<NoScale<Duration>> {
    pub fn no_scale(delay: Duration) -> Self {
        Self::new(delay, NoScale::new())
    }
}

impl<S: Scaler<Type=Duration>> Error for Delay<S> {
    fn handle<T, E>(&mut self, _: &Result<T, E>) -> Status {
        std::thread::sleep(self.scaler.scale(self.delay));
        Status::Continue
    }
}

impl<S: Default + Scaler<Type=Duration>> Create for Delay<S> {
    type Handler = Self;
    type Err = std::convert::Infallible;

    fn create_handler(&self) -> Result<Self::Handler, Self::Err> {
        Ok(Self {
            delay: self.delay,
            scaler: S::default(),
        })
    }
}
