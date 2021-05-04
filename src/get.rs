use std::{collections::HashMap, error, fmt, hash};

pub trait OkOrr<T>: Sized {
    fn ok_or<E>(self, err: E) -> Result<T, E>;

    fn must_get<S: Into<String>>(self, key: S) -> Result<T, KeyError> {
        self.ok_or(KeyError { key: key.into() })
    }
}

impl<T> OkOrr<T> for std::option::Option<T> {
    fn ok_or<E>(self, err: E) -> Result<T, E> {
        std::option::Option::ok_or(self, err)
    }
}

#[derive(Debug)]
pub struct KeyError {
    key: String,
}

impl KeyError {
    fn from<D: fmt::Debug>(d: &D) -> Self {
        Self {
            key: format!("{:?}", d),
        }
    }
}

impl fmt::Display for KeyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "failed to query key {}", self.key)
    }
}

impl error::Error for KeyError {}

pub trait Get {
    type Key: fmt::Debug;
    type Value;

    fn get(&self, key: &Self::Key) -> Option<&Self::Value>;

    fn must_get(&self, key: &Self::Key) -> Result<&Self::Value, KeyError> {
        self.get(key).ok_or(KeyError::from(key))
    }
}

impl<K, V, S> Get for HashMap<K, V, S>
where
    K: fmt::Debug + hash::Hash + std::cmp::Eq,
    S: hash::BuildHasher,
{
    type Key = K;
    type Value = V;

    fn get(&self, key: &Self::Key) -> Option<&Self::Value> {
        HashMap::get(self, key)
    }
}
