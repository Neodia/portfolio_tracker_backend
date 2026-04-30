use crate::client::CGClient;
use crate::client::error::ClientError;
use crate::client::model::{BlockchainAssetPrice, GetPricesFromNetworkResponse};
use crate::model::error::AppError;
use crate::model::{Asset, AssetPrice, Contract, Network};
use crate::repository::{AssetRepository, OutboxRepository, RateRepository, Repositories};
use crate::service::error::ServiceError;
use chrono::Utc;
use futures::future::try_join_all;
use itertools::Itertools;
use std::collections::HashMap;

#[derive(Clone)]
pub struct RatesService<C: CGClient> {
    repositories: Repositories,
    client: C,
}

impl<C: CGClient> RatesService<C> {
    pub fn new(repositories: Repositories, client: C) -> Self {
        Self {
            repositories,
            client,
        }
    }
    pub async fn fetch_rates_and_persist(&self) -> Result<(), AppError> {
        let assets = self.repositories.asset.get_all_assets().await?;

        let assets_per_network: HashMap<Network, Vec<&Asset>> =
            assets.iter().into_group_map_by(|asset| asset.network);

        let prices_per_asset_f = assets_per_network
            .into_iter()
            .map(|(network, assets)| Self::fetch_prices(&self.client, network, assets));

        let all_token_prices: HashMap<_, _> = try_join_all(prices_per_asset_f)
            .await?
            .into_iter()
            .flatten()
            .collect();

        let (asset_rates, price_mapping_errors): (Vec<AssetPrice>, Vec<ServiceError>) = assets
            .into_iter()
            .map(|asset| Self::get_price_for_asset(asset, &all_token_prices))
            .partition_map(|result| match result {
                Ok(rate) => itertools::Either::Left(rate),
                Err(e) => itertools::Either::Right(e),
            });

        // Logs business errors. Other errors get bubbled up
        for error in price_mapping_errors {
            tracing::warn!(warn=?error, "Error finding price for asset");
        }

        let now = Utc::now();
        // Inserting in the outbox table should be inside the same TX to ensure once delivery
        let mut tx = self.repositories.begin_transaction().await?;
        self.repositories
            .rate
            .insert_rates(&mut tx, asset_rates, now)
            .await?;
        self.repositories
            .outbox
            .insert_rates_inserted(&mut tx, now)
            .await?;
        self.repositories.commit_transaction(tx).await?;

        Ok(())
    }

    async fn fetch_prices(
        client: &C,
        network: Network,
        assets: Vec<&Asset>,
    ) -> Result<HashMap<(Network, Contract), BlockchainAssetPrice>, ClientError> {
        let map_response_to_hashmap = |prices: GetPricesFromNetworkResponse| -> HashMap<(Network, Contract), BlockchainAssetPrice> {
            prices
                .prices
                .into_iter()
                .map(|price| ((network, price.contract.clone()), price))
                .collect::<HashMap<_, _>>()
        };

        let contracts: Vec<Contract> = assets.iter().map(|a| a.contract_address.clone()).collect();
        client
            .get_prices_from_network(network, contracts)
            .await
            .map(map_response_to_hashmap)
    }

    fn get_price_for_asset(
        asset: Asset,
        prices: &HashMap<(Network, Contract), BlockchainAssetPrice>,
    ) -> Result<AssetPrice, ServiceError> {
        let key = (asset.network, asset.contract_address.clone());
        let price = prices.get(&key);
        let price = price.ok_or(ServiceError::MissingAssetPriceError(asset.clone()))?;

        Ok(AssetPrice::new(asset, price.price))
    }
}
