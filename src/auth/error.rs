use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Missing auth token")]
    MissingAuthToken,

    #[error("Claims encoding failed: {0}")]
    ClaimsEncodeError(String),
    #[error("Claims token is invalid")]
    ClaimsDecodeError,

    #[error("Password task failed: {0}")]
    PasswordTaskFailed(String),
    #[error("Password hashing failed: {0}")]
    PasswordHashingFailed(String),
    #[error("Password parsing failed: {0}")]
    PasswordParsingFailed(String),
    #[error("Password verification failed")]
    PasswordVerificationFailed,
}
