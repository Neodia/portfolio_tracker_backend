use std::time::Duration;
use dotenvy::dotenv;
use portfolio_tracker_backend::api::router::create_router;
use portfolio_tracker_backend::appstate::AppState;
use portfolio_tracker_backend::model::error::AppError;
use portfolio_tracker_backend::jobs;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    dotenv().ok();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let cg_url = std::env::var("CG_URL").expect("CG_KEY must be set");
    let cg_key = std::env::var("CG_KEY").expect("CG_KEY must be set");
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let state = AppState::new(
        db_url,
        cg_url,
        cg_key,
        jwt_secret,
    ).await?;

    let rates_fetching_job_state = state.clone();

    // Spawns rates-fetching job
    tokio::spawn(async move {
        let mut ticker = tokio::time::interval(Duration::from_mins(60));
        loop {
            ticker.tick().await;
            if let Err(e) = jobs::rates::fetch_rates_and_persist(rates_fetching_job_state.clone()).await {
                eprintln!("Rate fetching error: {e}");
            }
        }
    });

    let app = create_router(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
