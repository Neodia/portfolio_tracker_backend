use crate::api::auth::AuthenticatedUser;
use crate::api::handlers::{assets, auth, health, portfolio};
use crate::appstate::AppState;
use axum::Router;
use axum::middleware::from_extractor_with_state;
use axum::routing::{delete, get, patch, post, put};
use tower_http::trace::TraceLayer;

pub fn create_router(state: AppState) -> Router {
    let health_routes = Router::new()
        .route("/live", get(health::live_check))
        .route("/ready", get(health::readiness_check));

    let public_routes = Router::new()
        .route("/register", post(auth::register))
        .route("/login", post(auth::login));

    let asset_routes = Router::new()
        .route("/", get(assets::get_all_assets))
        .route("/", post(assets::insert_asset));
    let portfolio_routes = Router::new()
        .route(
            "/allocations/{asset_id}",
            put(portfolio::upsert_expected_asset_allocation),
        )
        .route(
            "/allocations/{asset_id}",
            delete(portfolio::delete_expected_asset_allocation),
        )
        .route("/holdings", post(portfolio::insert_holding))
        .route("/holdings/{holding_id}", patch(portfolio::update_holding))
        .route("/holdings/{holding_id}", delete(portfolio::delete_holding))
        .route("/", get(portfolio::get_portfolio));
    let protected_routes = Router::new()
        .nest("/assets", asset_routes)
        .nest("/portfolio", portfolio_routes)
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
