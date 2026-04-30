use crate::model::error::AppError;
use axum::extract::{FromRequest, Request};
use axum::Json;
use serde::de::DeserializeOwned;
use validator::Validate;

pub mod error;
pub mod handlers;
pub mod model;
pub mod router;
mod auth;

// Taken from: https://docs.rs/axum/latest/axum/extract/trait.FromRequest.html
pub struct ValidatedJson<T>(pub T);
impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state)
            .await
            .map_err(|e| AppError::BadRequestError(e.to_string()))?;
        value
            .validate()
            .map_err(|e| AppError::BadRequestError(e.to_string()))?;
        Ok(ValidatedJson(value))
    }
}
