mod client;
mod model;
mod repository;

use crate::client::LiveCGClient;
use crate::model::contract::Contract;
use crate::model::{BlockchainAsset, Network};
use crate::repository::AssetRepository;
use crate::repository::repository::Repository;
use client::CGClient;
use dotenvy::dotenv;
use futures::future::try_join_all;
use itertools::Itertools;
use model::error::AppError;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    dotenv().ok();

    let live_client: LiveCGClient = LiveCGClient::new(
        "https://api.coingecko.com/api/v3".into(),
        std::env::var("CG_KEY").expect("CG_KEY must be set"),
    );

    let repo = AssetRepository::new(&std::env::var("DATABASE_URL")?).await?;
    let assets = repo.get_all_assets().await?;

    let assets_per_network: HashMap<Network, Vec<BlockchainAsset>> = assets
        .into_iter()
        .into_group_map_by(|asset| asset.network.clone());

    let prices_per_asset_f = assets_per_network.into_iter().map(|(network, assets)| {
        let contracts: Vec<Contract> = assets.iter().map(|a| a.contract.clone()).collect();
        live_client.get_prices_from_network(network, contracts)
    });

    let all_token_prices = try_join_all(prices_per_asset_f).await?;
    for token_prices_per_network in all_token_prices {
        for (token_on_chain, price) in token_prices_per_network.prices {
            println!(
                "{}({}): {}",
                token_on_chain.symbol,
                token_on_chain.contract,
                price
            );
        }
    }

    Ok(())
}
