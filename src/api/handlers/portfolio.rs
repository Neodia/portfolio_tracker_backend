use crate::api::ValidatedJson;
use crate::api::auth::AuthenticatedUser;
use crate::api::error::ApiError;
use crate::api::model::{AddExpectedAllocationRequest, AddHoldingRequest, UpdateHoldingRequest};
use crate::appstate::AppState;
use crate::model::PortfolioResponse;
use crate::model::ids::{AssetId, HoldingId};
use axum::Json;
use axum::extract::{Path, State};

pub async fn upsert_expected_asset_allocation(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(asset_id): Path<AssetId>,
    ValidatedJson(req): ValidatedJson<AddExpectedAllocationRequest>,
) -> Result<Json<()>, ApiError> {
    let response = state
        .services
        .portfolio_service
        .upsert_expected_asset_allocation(user.id, asset_id, req.expected_allocation_pct)
        .await?;
    Ok(Json(response))
}
pub async fn delete_expected_asset_allocation(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(asset_id): Path<AssetId>,
) -> Result<Json<()>, ApiError> {
    let response = state
        .services
        .portfolio_service
        .delete_expected_asset_allocation(user.id, asset_id)
        .await?;
    Ok(Json(response))
}
pub async fn insert_holding(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(req): ValidatedJson<AddHoldingRequest>,
) -> Result<Json<()>, ApiError> {
    state
        .services
        .portfolio_service
        .insert_holding(user.id, req.asset_id, req.amount, req.description)
        .await?;
    Ok(Json(()))
}
pub async fn update_holding(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(holding_id): Path<HoldingId>,
    ValidatedJson(req): ValidatedJson<UpdateHoldingRequest>,
) -> Result<Json<()>, ApiError> {
    let response = state
        .services
        .portfolio_service
        .update_holding(user.id, holding_id, req.amount, req.description)
        .await?;
    Ok(Json(response))
}
pub async fn delete_holding(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(holding_id): Path<HoldingId>,
) -> Result<Json<()>, ApiError> {
    let response = state
        .services
        .portfolio_service
        .delete_holding(user.id, holding_id)
        .await?;
    Ok(Json(response))
}

pub async fn get_portfolio(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> Result<Json<PortfolioResponse>, ApiError> {
    let portfolio_response = state
        .services
        .portfolio_service
        .get_portfolio(user.id)
        .await?;
    Ok(Json(portfolio_response))
}

pub async fn refresh_portfolio(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> Result<Json<PortfolioResponse>, ApiError> {
    let portfolio_response = state
        .services
        .portfolio_service
        .refresh_portfolio(user.id)
        .await?;
    Ok(Json(portfolio_response))
}
