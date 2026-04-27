use crate::client::error::ClientError;
use crate::repository::error::DBError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Client error: {0}")]
    ClientError(#[from] ClientError),

    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),

    #[error("Var parsing error: {0}")]
    VarError(#[from] std::env::VarError),

    #[error("Database error: {0}")]
    DatabaseError(#[from] DBError),
}