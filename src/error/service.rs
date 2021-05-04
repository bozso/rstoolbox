use std::{
    error::Error as StdError,
    sync::{Arc, Mutex},
};

#[cfg(feature = "allocator_api")]
use std::alloc::AllocError;

pub type DynError = Box<dyn StdError>;

use crate::database as db;

#[derive(Debug)]
pub struct Service<DB> {
    db: DB,
}

impl<DB> Service<DB> {
    pub fn new(db: DB) -> Self {
        Self { db: db }
    }

    pub fn marced(self) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(self))
    }
}

impl<DB: db::AutoKey<Value = DynError>> Service<DB> {
    #[cfg(feature = "allocator_api")]
    pub fn error<E: StdError>(&mut self, err: E) -> Result<Ref<DB::Key>, AllocError> {
        Ok(Ref {
            key: self.db.auto_insert(Box::try_new(err)?),
        })
    }

    pub fn error<E: StdError + 'static>(&mut self, err: E) -> Ref<DB::Key> {
        Ref {
            key: self.db.auto_insert(Box::new(err)),
        }
    }

    pub fn dyn_error<'a>(&'a self, key: &DB::Key) -> Option<&'a (dyn StdError + 'a)> {
        self.db.get(key).map(|ok| ok.as_ref())
    }

    pub fn is<'a, E: StdError + 'static>(&'a self, key: &DB::Key) -> Option<bool> {
        match self.dyn_error(key) {
            Some(ref err) => Some(err.is::<E>()),
            None => None,
        }
        //.map(|err| err.is::<E>())
    }
}

#[derive(Debug)]
pub struct Ref<K> {
    key: K,
}

#[derive(Debug)]
pub struct Error<K, DB> {
    reference: Ref<K>,
    service: Arc<Mutex<Service<DB>>>,
}

/*
impl error::Error for Error<K, DB> {
}
*/
