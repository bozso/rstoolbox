mod config;
mod download;
mod service;
mod extract;
pub mod task;

pub enum Error {
}

pub type Result<T> = ::std::result::Result<T, Error>;

pub use config::Config;
pub use task::Task;
pub use download::Downloader;
pub use extract::Extractor;
pub use service::Service;
