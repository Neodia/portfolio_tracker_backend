use crate::client::CGClient;
use crate::client::error::ClientError;
use crate::client::model::{BlockchainAssetRate, GetRatesFromNetworkResponse};
use crate::model::{Asset, AssetRate, Contract, Network};
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
    pub async fn fetch_all_rates_and_persist(&self) -> Result<(), ServiceError> {
        let assets = self.repositories.asset.get_all_assets().await?;
        self.fetch_asset_rates_and_persist(assets).await?;
        Ok(())
    }
    pub async fn fetch_asset_rates_and_persist(
        &self,
        assets: Vec<Asset>,
    ) -> Result<(), ServiceError> {
        let assets_per_network: HashMap<Network, Vec<&Asset>> =
            assets.iter().into_group_map_by(|asset| asset.network);

        let rates_per_asset_f = assets_per_network
            .into_iter()
            .map(|(network, assets)| Self::fetch_rates(&self.client, network, assets));

        let all_token_rates: HashMap<_, _> = try_join_all(rates_per_asset_f)
            .await?
            .into_iter()
            .flatten()
            .collect();

        let (asset_rates, rate_mapping_errors): (Vec<AssetRate>, Vec<Asset>) = assets
            .into_iter()
            .map(|asset| Self::get_rate_for_asset(asset, &all_token_rates))
            .partition_map(|result| match result {
                Ok(rate) => itertools::Either::Left(rate),
                Err(e) => itertools::Either::Right(e),
            });

        // Logs assets for which we didn't find rates
        for asset_without_rate in rate_mapping_errors {
            tracing::warn!(asset=?asset_without_rate, "Error finding rate for asset");
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

    async fn fetch_rates(
        client: &C,
        network: Network,
        assets: Vec<&Asset>,
    ) -> Result<HashMap<(Network, Contract), BlockchainAssetRate>, ClientError> {
        let map_response_to_hashmap = |rates: GetRatesFromNetworkResponse| -> HashMap<(Network, Contract), BlockchainAssetRate> {
            rates
                .rates
                .into_iter()
                .map(|rate| ((network, rate.contract.clone()), rate))
                .collect::<HashMap<_, _>>()
        };

        let contracts: Vec<Contract> = assets.iter().map(|a| a.contract_address.clone()).collect();
        client
            .get_rates_from_network(network, contracts)
            .await
            .map(map_response_to_hashmap)
    }

    fn get_rate_for_asset(
        asset: Asset,
        rates: &HashMap<(Network, Contract), BlockchainAssetRate>,
    ) -> Result<AssetRate, Asset> {
        let key = (asset.network, asset.contract_address.clone());
        let rate = rates.get(&key);
        let rate = rate.ok_or(asset.clone())?;

        Ok(AssetRate::new(asset, rate.rate))
    }
}
