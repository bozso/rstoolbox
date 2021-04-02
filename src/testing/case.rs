use std::{
    error::Error as StdError,
    result::Result as StdResult,
    cmp::PartialEq,
    fmt::Debug,
};

pub trait Test<T, E: StdError> {
    fn test(&self) -> StdResult<T, E>;
}

pub struct Case<T, K, E>
where
    T: PartialEq + Debug,
    E: StdError + PartialEq,
    K: Test<T, E>
{
    pub test: K,
    pub expected: StdResult<T, E>,
}

use thiserror::Error as TError;

#[derive(TError, Debug, PartialEq)]
pub enum Error {
    
}

impl<T, K, E> Case<T, K, E>
where
    T: PartialEq + Debug,
    E: StdError + PartialEq,
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
    E: StdError + PartialEq,
    K: Test<T, E>
{
    cases: &'a [Case<T, K, E>],
}

impl<'a, T, K, E> Cases<'a, T, K, E>
where
    T: PartialEq + Debug,
    E: StdError + PartialEq,
    K: Test<T, E>
{
    pub fn new(cases: &'a [Case<T, K, E>]) -> Self {
        Self{cases: cases}
    }
    
    pub fn run(&self) {
        self.cases.iter().for_each(|c| c.run());
    }
}
