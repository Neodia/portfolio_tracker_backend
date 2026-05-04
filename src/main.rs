use portfolio_tracker_backend::api::router::create_router;
use portfolio_tracker_backend::appconfig::AppConfig;
use portfolio_tracker_backend::appstate::AppState;
use portfolio_tracker_backend::jobs;
use portfolio_tracker_backend::model::error::AppError;
use std::time::Duration;
use tokio::signal;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let config = AppConfig::load()?;

    init_tracing(config.json_logs);

    let state = AppState::new(
        config.database_url,
        config.cg_url,
        config.cg_key,
        config.jwt_secret,
    )
    .await?;

    // Spawns rates-fetching job
    let rates_fetching_job_state = state.clone();
    tokio::spawn(async move {
        let mut ticker = tokio::time::interval(Duration::from_mins(60));
        loop {
            ticker.tick().await;
            if let Err(e) =
                jobs::rates::fetch_rates_and_persist(rates_fetching_job_state.clone()).await
            {
                tracing::error!(job="Rate", error=?e,"Rate fetching error");
            }
        }
    });

    // Spawns portfolio snapshot job
    let portfolio_snapshot_job_state = state.clone();
    tokio::spawn(async move {
        let mut ticker = tokio::time::interval(Duration::from_mins(60));
        loop {
            ticker.tick().await;
            if let Err(e) = jobs::portfolio_snapshots::compute_pending_snapshots(
                portfolio_snapshot_job_state.clone(),
            )
            .await
            {
                tracing::error!(job="Portfolio Snapshots", error=?e,"Snapshots computation error");
            }
        }
    });

    let app = create_router(state);

    tracing::info!(port = 3000, "Server starting");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

fn init_tracing(json_logs: bool) {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    if json_logs {
        tracing_subscriber::fmt()
            .json()
            .with_env_filter(filter)
            .init();
    } else {
        tracing_subscriber::fmt().with_env_filter(filter).init();
    }
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => tracing::info!("Received Ctrl+C, shutting down"),
        _ = terminate => tracing::info!("Received SIGTERM, shutting down"),
    }
}
