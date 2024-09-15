use thiserror::Error;

use crate::commands::errors::CommandError;

pub type Result<T> = std::result::Result<T, BlockSimError>;

#[derive(Error, Debug)]
#[error(transparent)]
pub enum BlockSimError {
    CommandError(#[from] CommandError),
}
