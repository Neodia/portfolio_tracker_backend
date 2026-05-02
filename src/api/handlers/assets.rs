use crate::api::error::ApiError;
use crate::api::model::AddAssetRequest;
use crate::api::ValidatedJson;
use crate::appstate::AppState;
use crate::model::Asset;
use axum::extract::State;
use axum::Json;

pub async fn get_all_assets(
    State(state): State<AppState>,
) -> Result<Json<Vec<Asset>>, ApiError> {
    Ok(Json(
        state
            .services
            .asset_service
            .get_all_assets()
            .await?
    ))
}

pub async fn insert_asset(
    State(state): State<AppState>,
    ValidatedJson(req): ValidatedJson<AddAssetRequest>,
) -> Result<Json<()>, ApiError> {
    state
        .services
        .asset_service
        .insert_asset(req.symbol, req.name, req.network, req.contract_address)
        .await?;
    Ok(Json(()))
}
