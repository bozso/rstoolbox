pub mod cli;
pub mod handle;
pub mod geometry;
pub mod embed;
pub mod command;
pub mod server;
pub mod path;
pub mod testing;
pub mod service;
pub mod assets;
pub mod template;

mod get;

pub use get::{Get, KeyError};

pub mod thread {
    pub mod safe {
        pub trait Safe : Sync + Send {}
        pub trait Static : Safe + 'static {}
        pub trait Error : Safe + std::error::Error {}
        pub trait StaticError: Safe + std::error::Error {}

        impl<T> Static for T 
        where
            T: Safe + 'static
        {}

        impl<E> StaticError for E
        where
            E: Safe + std::error::Error
        {}
    }

    pub use safe::Safe;

    pub trait Static<T>
    where
        T: 'static
    {}

    pub trait StaticError: std::error::Error + 'static {}
}
