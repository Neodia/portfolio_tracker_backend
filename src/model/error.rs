use crate::repository::error::DBError;
use thiserror::Error;

// Basically startup errors
#[derive(Error, Debug)]
pub enum AppError {
    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),

    #[error("Database error: {0}")]
    DatabaseError(#[from] DBError),
}