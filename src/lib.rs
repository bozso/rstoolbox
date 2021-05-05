pub mod assets;
pub mod cli;
pub mod command;
pub mod database;
pub mod embed;
//pub mod error;
pub mod geometry;
pub mod handle;
pub mod path;
pub mod server;
pub mod service;
pub mod template;
pub mod testing;

mod get;

pub use get::{Get, KeyError, OkOrr};

pub mod thread {
    pub mod safe {
        pub trait Safe: Sync + Send {}
        pub trait Static: Safe + 'static {}
        pub trait Error: Safe + std::error::Error {}
        pub trait StaticError: Safe + std::error::Error {}

        impl<T> Static for T where T: Safe + 'static {}

        impl<E> StaticError for E where E: Safe + std::error::Error {}
    }

    pub use safe::Safe;

    pub trait Static<T>
    where
        T: 'static,
    {
    }

    pub trait StaticError: std::error::Error + 'static {}
}
