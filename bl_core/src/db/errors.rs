use thiserror::Error;

pub type Result<T> = std::result::Result<T, DbError>;

#[derive(Debug, Error)]
pub enum DbError {
    #[error("Database error: {0}")]
    DbError(#[from] sea_orm::error::DbErr),
}
