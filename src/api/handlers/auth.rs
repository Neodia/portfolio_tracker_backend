use crate::api::error::ApiError;
use crate::api::model::{LoginRequest, RegisterRequest, TokenResponse};
use crate::api::ValidatedJson;
use crate::appstate::AppState;
use axum::extract::State;
use axum::Json;

pub async fn register(
    State(state): State<AppState>,
    ValidatedJson(req): ValidatedJson<RegisterRequest>,
) -> Result<Json<TokenResponse>, ApiError> {
    let token = state
        .services
        .user_service
        .register(req.email.as_str(), req.password.as_str())
        .await?;

    Ok(Json(TokenResponse::new(token.0)))
}

pub async fn login(
    State(state): State<AppState>,
    ValidatedJson(req): ValidatedJson<LoginRequest>,
) -> Result<Json<TokenResponse>, ApiError> {
    let user = state
        .services
        .user_service
        .login(req.email.as_str(), req.password.as_str())
        .await?;

    Ok(Json(TokenResponse::new(user.0)))
}
