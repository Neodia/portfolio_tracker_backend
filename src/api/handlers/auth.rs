use crate::api::model::{LoginRequest, RegisterRequest, TokenResponse};
use crate::appstate::AppState;
use crate::model::error::AppError;
use axum::Json;
use axum::extract::State;

pub async fn register(
    State(state): State<AppState>,
    req: Json<RegisterRequest>,
) -> Result<Json<TokenResponse>, AppError> {
    let token = state
        .services
        .user_service
        .register(req.email.as_str(), req.password.as_str())
        .await?;

    Ok(Json(TokenResponse::new(token.0)))
}

pub async fn login(
    State(state): State<AppState>,
    req: Json<LoginRequest>,
) -> Result<Json<TokenResponse>, AppError> {
    let user = state
        .services
        .user_service
        .login(req.email.as_str(), req.password.as_str())
        .await?;

    Ok(Json(TokenResponse::new(user.0)))
}
