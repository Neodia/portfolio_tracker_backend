use crate::client::error::ClientError;
use crate::model::Asset;
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

    #[error("Registration failed: User already exists")]
    UserAlreadyExistsError,

    #[error("User not found: Invalid email/password combination")]
    UserNotFoundError,

    #[error("TokenCreationError: {0}")]
    TokenCreationError(#[from] jsonwebtoken::errors::Error),

    #[error("PasswordError: {0}")]
    PasswordError(String),

    #[error("Missing CG Price for {0}")]
    MissingAssetPriceError(Asset),
}
