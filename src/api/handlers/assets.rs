use crate::api::ValidatedJson;
use crate::api::model::{AddAssetRequest, AssetResponse};
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

pub async fn insert_asset(
    State(state): State<AppState>,
    ValidatedJson(req): ValidatedJson<AddAssetRequest>,
) -> Result<Json<()>, AppError> {
    state
        .services
        .asset_service
        .insert_asset(req.symbol, req.name, req.network, req.contract_address)
        .await?;
    Ok(Json(()))
}
