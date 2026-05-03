use crate::appstate::AppState;
use crate::service::error::ServiceError;

pub async fn compute_pending_snapshots(app: AppState) -> Result<(), ServiceError> {
    tracing::info!(job = "Portfolio Snapshots", "Computing pending snapshots");
    let res = app
        .services
        .portfolio_service
        .compute_pending_snapshots()
        .await?;
    tracing::info!(
        job = "Portfolio Snapshots",
        nb_users = res.number_of_users,
        nb_snapshot_events = res.number_of_snapshots_events,
        total_snapshots = res.number_of_snapshots_events * res.number_of_users,
        "Finished computing snapshots",
    );
    Ok(())
}
