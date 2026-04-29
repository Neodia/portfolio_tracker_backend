use crate::api::handlers::{assets, auth};
use crate::appstate::AppState;
use axum::Router;
use axum::routing::{get, post};
use tower_http::trace::TraceLayer;

pub fn create_router(state: AppState) -> Router {
    let public_routes = Router::new()
        .route("/register", post(auth::register))
        .route("/login", post(auth::login));

    let protected_routes = Router::new().route("/assets", get(assets::get_all_assets));

    Router::new()
        .nest("/auth", public_routes)
        .nest("/api", protected_routes)
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
