use crate::auth::error::AuthError;
use crate::client::error::ClientError;
use crate::repository::error::DBError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServiceError {
    // Internal, to log
    #[error("Internal server error: {0}")]
    InternalServerError(String),

    // User facing
    #[error("Unauthorized")]
    Unauthorized,
    #[error("User with email {0} not found")]
    UserNotFound(String),
    #[error("Email/password doesn't match")]
    PasswordDoesntMatch,
    #[error("User email {0} already exists")]
    UserEmailAlreadyExistsError(String),
}
impl From<AuthError> for ServiceError {
    fn from(auth_error: AuthError) -> Self {
        match auth_error {
            // User facing
            AuthError::MissingAuthToken => ServiceError::Unauthorized,
            AuthError::ClaimsDecodeError => ServiceError::Unauthorized,
            AuthError::PasswordVerificationFailed => ServiceError::PasswordDoesntMatch,

            // Internal, to log
            AuthError::ClaimsEncodeError(err) => ServiceError::InternalServerError(err.to_string()),
            AuthError::PasswordHashingFailed(err) => {
                ServiceError::InternalServerError(err.to_string())
            }
            AuthError::PasswordParsingFailed(err) => {
                ServiceError::InternalServerError(err.to_string())
            }
        }
    }
}
impl From<DBError> for ServiceError {
    fn from(db_error: DBError) -> Self {
        match db_error {
            // User facing
            DBError::UserEmailAlreadyExistsError(email) => {
                ServiceError::UserEmailAlreadyExistsError(email)
            }

            // Internal, to log
            DBError::NetworkDeserializeError(err) => {
                ServiceError::InternalServerError(err.to_string())
            }
            DBError::OutboxEventTypeDeserializeError(err) => {
                ServiceError::InternalServerError(err.to_string())
            }

            // At startup, won't happen when running
            DBError::ConnectorError(_) => {
                ServiceError::InternalServerError("Internal Server Error".to_string())
            }
            DBError::MigrationError(_) => {
                ServiceError::InternalServerError("Internal Server Error".to_string())
            }
        }
    }
}
impl From<ClientError> for ServiceError {
    fn from(client_error: ClientError) -> Self {
        match client_error {
            // Internal, to log
            ClientError::HttpError(err) => ServiceError::InternalServerError(err.to_string()),
            ClientError::Unauthorized => {
                ServiceError::InternalServerError("CG Key is outdated".to_string())
            }
            ClientError::RateLimited => {
                ServiceError::InternalServerError("CG rate limited".to_string())
            }
            ClientError::NotFound => {
                ServiceError::InternalServerError("CG url not found".to_string())
            }
            ClientError::Unexpected(err) => ServiceError::InternalServerError(err.to_string()),

            // Doesn't get bubbled up
            err @ ClientError::MissingAssetRateError(_) => {
                ServiceError::InternalServerError(err.to_string())
            }
        }
    }
}
