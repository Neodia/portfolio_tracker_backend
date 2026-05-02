use crate::service::error::ServiceError;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    // Internal, to log
    #[error("{0}")]
    InternalServerError(String),

    // User facing
    #[error("Bad request {0}")]
    BadRequest(String),
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Email/password doesn't match")]
    UserNotFound,
    #[error("User already registered")]
    UserAlreadyRegistered,
}
impl From<ServiceError> for ApiError {
    fn from(service_error: ServiceError) -> Self {
        match service_error {
            // Internal, to log
            ServiceError::InternalServerError(err) => {
                ApiError::InternalServerError(err.to_string())
            }

            // User facing
            ServiceError::Unauthorized => ApiError::Unauthorized,
            ServiceError::UserNotFound(_) => ApiError::UserNotFound,
            ServiceError::PasswordDoesntMatch => ApiError::UserNotFound,
            ServiceError::UserEmailAlreadyExistsError(_) => ApiError::UserAlreadyRegistered,
        }
    }
}
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        if let ApiError::InternalServerError(err) = &self {
            tracing::error!("{}", err);
        }

        let (status, error_code, description) = match self {
            ApiError::InternalServerError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "INTERNAL_SERVER_ERROR",
                "Internal server error".to_string(),
            ),
            ApiError::BadRequest(_) => (StatusCode::BAD_REQUEST, "BAD_REQUEST", self.to_string()),
            ApiError::Unauthorized => (StatusCode::UNAUTHORIZED, "UNAUTHORIZED", self.to_string()),
            ApiError::UserNotFound => (StatusCode::UNAUTHORIZED, "USER_NOT_FOUND", self.to_string()),
            ApiError::UserAlreadyRegistered => (
                StatusCode::CONFLICT,
                "CONFLICT",
                self.to_string(),
            ),
        };

        (
            status,
            Json(serde_json::json!({
                "error_code": error_code,
                "description": description,
            })),
        )
            .into_response()
    }
}
