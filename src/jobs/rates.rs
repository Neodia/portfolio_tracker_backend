use crate::appstate::AppState;
use crate::service::error::ServiceError;

pub async fn fetch_rates_and_persist(app: AppState) -> Result<(), ServiceError> {
    tracing::info!(job = "Rates", "Fetching rates");
    let res = app
        .services
        .rates_service
        .fetch_all_rates_and_persist()
        .await;
    tracing::info!(job = "Rates", "Finished fetching rates");
    res
}
