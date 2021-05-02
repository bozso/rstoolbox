use crate::{
    database as db,
};

pub struct Service<DB> {
    db: DB,
}

impl<DB> Service<DB> {
    pub fn new(db: DB) -> Self {
        Self {
            db: db,
        }
    }
}

impl<DB: db::generic::AutoKey> Service<DB> {
    pub fn error<E>(&mut self, err: E) -> Error<DB::Key> {
        Error {
            key: self.db.auto_insert(err),
        }
    }
}

pub struct Error<K> {
    key: K,
}
