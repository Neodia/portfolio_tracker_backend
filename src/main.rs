mod client;
mod model;
mod repository;
mod api;
mod service;
mod appstate;

use crate::client::live::LiveCGClient;
use crate::model::Contract;
use crate::model::{Asset, Network};
use crate::repository::live::LiveAssetRepository;
use crate::repository::AssetRepository;
use client::CGClient;
use dotenvy::dotenv;
use futures::future::try_join_all;
use itertools::Itertools;
use crate::model::error::AppError;
use std::collections::HashMap;
use crate::api::router::create_router;
use crate::appstate::AppState;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    dotenv().ok();

    let repo = LiveAssetRepository::new(&std::env::var("DATABASE_URL")?).await?;

    let state = AppState::new(repo);

    let app = create_router(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}

// Not used, kept as an example for future code
async fn _pull_prices_from_cg() -> Result<(), AppError> {
    let live_client: LiveCGClient = LiveCGClient::new(
        "https://api.coingecko.com/api/v3".into(),
        std::env::var("CG_KEY").expect("CG_KEY must be set"),
    );

    let repo = LiveAssetRepository::new(&std::env::var("DATABASE_URL")?).await?;
    let assets = repo.get_all_assets().await?;

    let assets_per_network: HashMap<Network, Vec<Asset>> = assets
        .into_iter()
        .into_group_map_by(|asset| asset.network.clone());

    let prices_per_asset_f = assets_per_network.into_iter().map(|(network, assets)| {
        let contracts: Vec<Contract> = assets.iter().map(|a| a.contract_address.clone()).collect();
        live_client.get_prices_from_network(network, contracts)
    });

    let all_token_prices = try_join_all(prices_per_asset_f).await?;
    for token_prices_per_network in all_token_prices {
        for blockchain_asset_price in token_prices_per_network.prices {
            println!(
                "{}({}): {}",
                blockchain_asset_price.symbol,
                blockchain_asset_price.contract,
                blockchain_asset_price.price,
            );
        }
    }

    Ok(())
}
