use crate::api::handlers::assets;
use crate::appstate::AppState;
use axum::{routing::get, Router};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/assets", get(assets::get_all_assets))
        .with_state(state)
}