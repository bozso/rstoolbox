pub trait Like {
    type Key;
    type Value;

    fn insert(&mut self, key: &Self::Key, val: Self::Value);
    fn get(&self, key: &Self::Key) -> Option<&Self::Value>;
    fn get_mut(&self, key: &Self::Key) -> Option<&mut Self::Value>;

    fn remove(&mut self, key: &Self::Key) -> Option<Self::Value>;
}

pub trait AutoKey: Like {
    fn get_key(&mut self) -> Self::Key;

    fn auto_insert(&mut self, val: Self::Value) -> Self::Key {
        let key = self.get_key();

        self.insert(&key, val);
        key
    }
}

pub mod generic {
    pub trait Value {
        type Key;

        fn insert<V>(&mut self, key: &Self::Key, val: V);
        fn get<V>(&self, key: &Self::Key) -> Option<&V>;
        fn get_mut<V>(&self, key: &Self::Key) -> Option<&mut V>;

        fn remove<V>(&mut self, key: &Self::Key) -> Option<V>;
    }

    pub trait AutoKey: Value {
        fn get_key(&mut self) -> Self::Key;

        fn auto_insert<V>(&mut self, val: V) -> Self::Key {
            let key = self.get_key();

            self.insert(&key, val);
            key
        }
    }
}
