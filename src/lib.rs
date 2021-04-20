pub mod geometry;
pub mod embed;
pub mod command;
pub mod server;
pub mod path;
pub mod testing;
pub mod service;
pub mod assets;
pub mod template;

pub trait ThreadSafe : Sync + Send {}

pub trait Static<T>
where
    T: 'static
{}

pub trait ThreadStatic : ThreadSafe + 'static {}
