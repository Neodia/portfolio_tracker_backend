use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use crate::model::error::AppError;

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_code, description) = match &self {
            AppError::DatabaseError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "DATABASE_ERROR",
                self.to_string(),
            ),
            AppError::ClientError(_) => {
                (StatusCode::BAD_GATEWAY, "UPSTREAM_ERROR", self.to_string())
            }
            // Shouldn't happen, this would only happen when creating the server break so HTTP calls won't even reach
            AppError::IOError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "IO_ERROR",
                self.to_string(),
            ),
            AppError::VarError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "CONFIG_ERROR",
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
