use crate::appstate::AppState;
use crate::model::error::AppError;

pub async fn fetch_rates_and_persist(app: AppState) -> Result<(), AppError> {
    println!("Rates Job: Fetching rates");
    let res = app.services.rates_service.fetch_rates_and_persist().await;
    println!("Rates Job: Finished fetching rates");
    res
}
