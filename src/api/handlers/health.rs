use crate::api::error::ApiError;
use crate::appstate::AppState;
use crate::service::error::ServiceError;
use axum::extract::State;

pub async fn live_check() -> &'static str {
    "OK"
}

pub async fn readiness_check(State(state): State<AppState>) -> Result<String, ApiError> {
    state
        .repositories
        .is_ready()
        .await
        .map_err(ServiceError::from)?;
    Ok("Ready".to_string())
}
