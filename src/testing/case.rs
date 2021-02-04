use std::{
    error::Error,
    result::Result,
    cmp::PartialEq,
    fmt::Debug,
    //marker::PhantomData,
};

pub trait Test<T, E: Error> {
    fn test(&self) -> Result<T, E>;
}

pub struct Case<T, K, E>
where
    T: PartialEq + Debug,
    E: Error + PartialEq,
    K: Test<T, E>
{
    pub test: K,
    pub expected: Result<T, E>,
}

use thiserror::Error as TError;

#[derive(TError, Debug, PartialEq)]
pub enum Error {
    
}

type Result = result::Result<(), Error>;

impl<T, K, E> Case<T, K, E>
where
    T: PartialEq + Debug,
    E: Error + PartialEq,
    K: Test<T, E>
{

    pub fn run(&self) {
        match self.test.test() {
            Err(res) => {
                match &self.expected {
                    Err(err) => {
                        if res != *err {
                            panic!("Expected error '{}', but got '{}'", res, err);
                        }
                    }
                    Ok(ok) => {
                        panic!("Expected error '{:?}' but got valid result '{:?}'", res, ok);
                    }
                }
            }
            Ok(res) => {
                match &self.expected {
                    Err(err) => {
                        panic!("Expected valid result '{:?}' but got error '{}'", res, err);
                    }
                    Ok(ok) => {
                        if *ok != res {
                            panic!("Expected '{:?}' but got '{:?}'", ok, res);
                        }
                    }
                }
            }
        }        
    } 
}

pub struct Cases<'a, T, K, E>
where
    T: PartialEq + Debug,
    E: Error + PartialEq,
    K: Test<T, E>
{
    cases: &'a [Case<T, K, E>],
}

impl<'a, T, K, E> Cases<'a, T, K, E>
where
    T: PartialEq + Debug,
    E: Error + PartialEq,
    K: Test<T, E>
{
    pub fn new(cases: &'a [Case<T, K, E>]) -> Self {
        Self{cases: cases}
    }
    
    pub fn run(&self) {
        let _ = self.cases.iter().map(|c| c.run()).collect::<_>();
    }
}
