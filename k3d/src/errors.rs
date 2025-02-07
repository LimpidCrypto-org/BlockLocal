use thiserror::Error;

pub(crate) type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("IO Error: {0}")]
    IO(#[from] std::io::Error),
    #[error("UTF-8 Error: {0}")]
    FromUtf8Error(#[from] std::string::FromUtf8Error),
    #[error("Command failed: {0}")]
    CommandFailed(String),
}
