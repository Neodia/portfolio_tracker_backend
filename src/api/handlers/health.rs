use crate::appstate::AppState;
use crate::model::error::AppError;
use axum::extract::State;

pub async fn live_check() -> &'static str {
    "OK"
}

pub async fn readiness_check(State(state): State<AppState>) -> Result<String, AppError> {
    state.repositories.is_ready().await?;
    Ok("Ready".to_string())
}
