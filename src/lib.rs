pub mod cli;
pub mod commands;
pub mod config;
pub mod core;
pub mod error;
pub mod i18n;
pub mod utils;

pub use error::Error;
pub type Result<T> = std::result::Result<T, Error>; 