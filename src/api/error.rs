use crate::model::error::AppError;
use crate::model::error::BusinessError;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use itertools::Either::{Left, Right};

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let either = match self {
            AppError::DatabaseError(_) => Left((
                StatusCode::INTERNAL_SERVER_ERROR,
                "DATABASE_ERROR",
                self.to_string(),
            )),
            AppError::ClientError(_) => {
                Left((StatusCode::BAD_GATEWAY, "UPSTREAM_ERROR", self.to_string()))
            }
            // Shouldn't happen, this would only happen when creating the server break so HTTP calls won't even reach
            AppError::IOError(_) => Left((
                StatusCode::INTERNAL_SERVER_ERROR,
                "IO_ERROR",
                self.to_string(),
            )),
            AppError::VarError(_) => Left((
                StatusCode::INTERNAL_SERVER_ERROR,
                "CONFIG_ERROR",
                self.to_string(),
            )),
            AppError::PasswordError(_) => Left((
                StatusCode::INTERNAL_SERVER_ERROR,
                "PASSWORD_ERROR",
                self.to_string(),
            )),
            AppError::TokenCreationError(_) => Left((
                StatusCode::INTERNAL_SERVER_ERROR,
                "TOKEN_CREATION_ERROR",
                self.to_string(),
            )),
            AppError::BadRequestError(_) => Left((
                StatusCode::BAD_REQUEST,
                "BAD_REQUEST",
                self.to_string(),
            )),
            AppError::BusinessError(business_error) => Right(business_error.into_response()),
        };

        either.right_or_else(|(status, error_code, description)| {
            (
                status,
                Json(serde_json::json!({
                    "error_code": error_code,
                    "description": description,
                })),
            )
                .into_response()
        })
    }
}

impl IntoResponse for BusinessError {
    fn into_response(self) -> Response {
        let (status, error_code, description) = match self {
            BusinessError::UserNotFoundError => {
                (StatusCode::NOT_FOUND, "NOT_FOUND", self.to_string())
            }
            BusinessError::UserAlreadyExistsError => {
                (StatusCode::CONFLICT, "CONFLICT", self.to_string())
            }
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
