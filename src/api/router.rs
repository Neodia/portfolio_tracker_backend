use crate::api::auth::AuthenticatedUser;
use crate::api::handlers::{assets, auth, health};
use crate::appstate::AppState;
use axum::Router;
use axum::middleware::from_extractor_with_state;
use axum::routing::{get, post};
use tower_http::trace::TraceLayer;

pub fn create_router(state: AppState) -> Router {
    let health_routes = Router::new()
        .route("/live", get(health::live_check))
        .route("/ready", get(health::readiness_check));

    let public_routes = Router::new()
        .route("/register", post(auth::register))
        .route("/login", post(auth::login));

    let protected_routes = Router::new()
        .route("/assets", get(assets::get_all_assets))
        .route("/assets", post(assets::insert_asset))
        .route_layer(from_extractor_with_state::<AuthenticatedUser, AppState>(
            state.clone(),
        ));

    Router::new()
        .nest("/health", health_routes)
        .nest("/auth", public_routes)
        .nest("/api", protected_routes)
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
