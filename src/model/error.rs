use crate::repository::DBError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("Var parsing error: {0}")]
    VarError(#[from] std::env::VarError),

    #[error("Database error: {0}")]
    DatabaseError(#[from] DBError),
}
