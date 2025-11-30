/// An error.
#[derive(Debug)]
pub enum Error {
    // TODO: implement error variants.
    Todo,
}

/// A result.
pub type Result<T> = std::result::Result<T, Error>;
