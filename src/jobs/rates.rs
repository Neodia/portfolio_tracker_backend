use crate::appstate::AppState;
use crate::model::error::AppError;

pub async fn fetch_rates_and_persist(app: AppState) -> Result<(), AppError> {
    tracing::info!(job = "Rates", "Fetching rates");
    let res = app.services.rates_service.fetch_rates_and_persist().await;
    tracing::info!(job = "Rates", "Finished fetching rates");
    res
}
