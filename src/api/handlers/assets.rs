use crate::api::model::AssetResponse;
use crate::appstate::AppState;
use crate::model::error::AppError;
use axum::Json;
use axum::extract::State;

pub async fn get_all_assets(
    State(state): State<AppState>,
) -> Result<Json<Vec<AssetResponse>>, AppError> {
    Ok(Json(
        state
            .services
            .asset_service
            .get_all_assets()
            .await?
            .into_iter()
            .map(AssetResponse::from)
            .collect(),
    ))
}
