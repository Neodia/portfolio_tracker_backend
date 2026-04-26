use crate::client::ClientError;
use crate::repository::DBError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Client error: {0}")]
    ClientError(#[from] ClientError),

    #[error("Var parsing error: {0}")]
    VarError(#[from] std::env::VarError),

    #[error("Database error: {0}")]
    DatabaseError(#[from] DBError),
    
    #[error("Network parsing error: unknown {0} network")]
    NetworkParsingError(String),
}
