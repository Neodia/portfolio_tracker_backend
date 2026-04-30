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

    #[error("TokenCreationError: {0}")]
    TokenCreationError(#[from] jsonwebtoken::errors::Error),

    #[error("PasswordError: {0}")]
    PasswordError(String),

    #[error("BusinessError: {0}")]
    BusinessError(#[from] BusinessError),

    #[error("BadRequest: {0}")]
    BadRequestError(String),
}

#[derive(Error, Debug)]
pub enum BusinessError {
    #[error("Registration failed: User already exists")]
    UserAlreadyExistsError,

    #[error("User not found: Invalid email/password combination")]
    UserNotFoundError,

    #[error("Unauthorized")]
    Unauthorized,
}
