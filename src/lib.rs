#![warn(clippy::all)]

mod database;
mod error;
mod node;
mod page;

pub use database::Database;
pub use error::{Error, Result};
