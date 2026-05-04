use crate::client::live::LiveCGClient;
use crate::model::ids::{AssetId, HoldingId, UserId};
use crate::model::{
    Asset, AssetAllocation, AssetHoldings, AssetHoldingsWithDrift, HoldingWithAllocation,
    OutboxEvent, PortfolioHoldings, PortfolioResponse, UserHolding,
};
use crate::repository::error::DBError;
use crate::repository::traits::PortfolioRepository;
use crate::repository::{OutboxRepository, RateRepository, Repositories};
use crate::service::error::ServiceError;
use crate::service::model::SnapshotsComputationResult;
use crate::service::rates::RatesService;
use chrono::{DateTime, Utc};
use futures::future::try_join_all;
use itertools::Itertools;
use rust_decimal::Decimal;
use rust_decimal::prelude::Zero;
use std::collections::HashMap;

#[derive(Clone)]
pub struct PortfolioService {
    repositories: Repositories,
    rates_service: RatesService<LiveCGClient>,
}
impl PortfolioService {
    pub fn new(repositories: Repositories, rates_service: RatesService<LiveCGClient>) -> Self {
        Self {
            repositories,
            rates_service,
        }
    }
    pub async fn upsert_expected_asset_allocation(
        &self,
        user_id: UserId,
        asset_id: AssetId,
        percentage: Decimal,
    ) -> Result<(), ServiceError> {
        let now = Utc::now();
        self.repositories
            .portfolio
            .upsert_expected_asset_allocation(user_id, asset_id, percentage, now)
            .await?;
        Ok(())
    }
    pub async fn delete_expected_asset_allocation(
        &self,
        user_id: UserId,
        asset_id: AssetId,
    ) -> Result<(), ServiceError> {
        self.repositories
            .portfolio
            .delete_expected_asset_allocation(user_id, asset_id)
            .await?;
        Ok(())
    }

    pub async fn insert_holding(
        &self,
        user_id: UserId,
        asset_id: AssetId,
        amount: Decimal,
        description: Option<String>,
    ) -> Result<HoldingId, ServiceError> {
        let now = Utc::now();
        let id = self
            .repositories
            .portfolio
            .insert_holding(user_id, asset_id, amount, description, now)
            .await?;
        Ok(id)
    }
    pub async fn update_holding(
        &self,
        user_id: UserId,
        holding_id: HoldingId,
        amount: Decimal,
        description: Option<String>,
    ) -> Result<(), ServiceError> {
        let now = Utc::now();
        self.repositories
            .portfolio
            .update_holding(holding_id, user_id, amount, description, now)
            .await?;
        Ok(())
    }
    pub async fn delete_holding(
        &self,
        user_id: UserId,
        holding_id: HoldingId,
    ) -> Result<(), ServiceError> {
        self.repositories
            .portfolio
            .delete_holding(holding_id, user_id)
            .await?;
        Ok(())
    }

    pub async fn get_portfolio(&self, user_id: UserId) -> Result<PortfolioResponse, ServiceError> {
        let (expected_allocations, holdings) = tokio::try_join!(
            self.repositories
                .portfolio
                .get_expected_asset_allocations(user_id),
            self.repositories.portfolio.get_holdings(user_id),
        )?;

        let expected_allocations_map: HashMap<AssetId, &AssetAllocation> = expected_allocations
            .iter()
            .map(|alloc| (alloc.asset.id, alloc))
            .collect();

        let total_portfolio_value_usd: Decimal =
            holdings.iter().map(|holding| holding.total_value_usd).sum();

        let asset_holdings_with_drift = holdings
            .into_iter()
            .map(|holding| {
                PortfolioService::compute_drift_for_holding(
                    holding,
                    &expected_allocations_map,
                    total_portfolio_value_usd,
                )
            })
            .sorted_by_key(|holding| -holding.total_value_usd) // Sorted from biggest holding to lowest
            .collect();

        let user_holdings =
            PortfolioHoldings::new(asset_holdings_with_drift, total_portfolio_value_usd);

        let historical_portfolio_values = self
            .repositories
            .portfolio
            .get_historical_portfolio_values(user_id)
            .await?;

        Ok(PortfolioResponse::new(
            expected_allocations,
            user_holdings,
            historical_portfolio_values,
        ))
    }

    pub async fn refresh_portfolio(
        &self,
        user_id: UserId,
    ) -> Result<PortfolioResponse, ServiceError> {
        let holdings = self.repositories.portfolio.get_holdings(user_id).await?;
        let holdings_assets: Vec<Asset> = holdings
            .into_iter()
            .map(|holding| holding.asset_rate.asset)
            .collect();

        self.rates_service
            .fetch_asset_rates_and_persist(holdings_assets)
            .await?;

        self.get_portfolio(user_id).await
    }

    pub async fn compute_pending_snapshots(
        &self,
    ) -> Result<SnapshotsComputationResult, ServiceError> {
        let events = self
            .repositories
            .outbox
            .get_pending_rates_persisted_events()
            .await?;

        let user_holdings = self.repositories.portfolio.get_all_users_holdings().await?;

        let now = Utc::now();
        let handle_events_f: Vec<_> = events
            .iter()
            .map(async |event| {
                self.calculate_portfolio_snapshots_at_event_date(event, &user_holdings, now)
                    .await
            })
            .collect();

        try_join_all(handle_events_f).await?;

        Ok(SnapshotsComputationResult::new(
            user_holdings.keys().len(),
            events.len(),
        ))
    }

    async fn calculate_portfolio_snapshots_at_event_date(
        &self,
        event: &OutboxEvent,
        user_holdings: &HashMap<UserId, Vec<UserHolding>>,
        now: DateTime<Utc>,
    ) -> Result<(), DBError> {
        let latest_rates_at_event_date = self
            .repositories
            .rate
            .get_latest_asset_rates_at(event.created_at)
            .await?;

        let users_portfolio_value_at: Vec<_> = user_holdings
            .iter()
            .map(|(user_id, user_holdings)| {
                let user_portfolio_value = user_holdings
                    .iter()
                    .map(|holding| {
                        let asset_rate_at_event_date_opt = latest_rates_at_event_date
                            .get(&holding.asset_id)
                            .map(|rate| rate.rate_usd);

                        if asset_rate_at_event_date_opt.is_none() {
                            tracing::warn!(
                                asset_id=%holding.asset_id,
                                at_date=%event.created_at,
                                "Missing asset rate during portfolio snapshots. Defaulting to 0. If the asset did have a price at that point but it wasn't persisted, run the backfill job and recompute the portfolios.",
                            )
                        }

                        let asset_rate_at_event_date = asset_rate_at_event_date_opt.unwrap_or(Decimal::zero());
                        holding.amount * asset_rate_at_event_date
                    })
                    .sum();
                (user_id, user_portfolio_value)
            })
            .collect();

        let mut tx = self.repositories.begin_transaction().await?;
        self.repositories
            .portfolio
            .insert_portfolio_snapshots(&mut tx, users_portfolio_value_at, event.created_at)
            .await?;
        self.repositories
            .outbox
            .set_pending_snapshot_as_handled(&mut tx, event.id, now)
            .await?;
        tx.commit().await?;
        Ok(())
    }

    fn compute_drift_for_holding(
        holdings: AssetHoldings,
        expected_allocations_map: &HashMap<AssetId, &AssetAllocation>,
        total_portfolio_value_usd: Decimal,
    ) -> AssetHoldingsWithDrift {
        let expected_allocation_opt = expected_allocations_map
            .get(&holdings.asset_rate.asset.id)
            .map(|alloc| alloc.allocation_pct);
        let expected_allocation_pct = expected_allocation_opt.unwrap_or(Decimal::zero());

        let holdings_with_allocation = holdings
            .holdings
            .iter()
            .map(|holding| {
                let current_allocation = holding
                    .value_usd
                    .checked_div(total_portfolio_value_usd)
                    .unwrap_or(Decimal::zero());
                HoldingWithAllocation::new(
                    holding.id,
                    holding.amount,
                    holding.value_usd,
                    holding.description.clone(),
                    current_allocation,
                )
            })
            .collect::<Vec<_>>();

        let total_allocation_pct = holdings_with_allocation
            .iter()
            .map(|holding| holding.current_allocation_pct)
            .sum();
        let drift = total_allocation_pct - expected_allocation_pct;

        AssetHoldingsWithDrift::new(
            holdings.asset_rate,
            holdings.total_value_usd,
            holdings_with_allocation,
            total_allocation_pct,
            expected_allocation_pct,
            drift,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::ids::HoldingId;
    use crate::model::{Asset, AssetRate, Contract, Holding, Network, Symbol};
    use rust_decimal::prelude::One;

    // str for exact precision
    fn decimal(number: &str) -> Decimal {
        Decimal::from_str_exact(number).unwrap()
    }

    fn test_fn(
        total_portfolio_value_usd: Decimal,
        asset_rate: Decimal,
        expected_drift: Decimal,
        expected_allocation_pct: Decimal,
        expected_holding_pct: Decimal,
    ) {
        let asset_id = AssetId::new();
        let symbol = Symbol::new("JITOSOL");
        let name = "Jito Staked Sol";
        let network = Network::Solana;
        let contract = Contract::from("J1toso1uCk3RLmjorhTtrVwY9HJ7X8V9yYac6Y7kGCPn");
        let amount_of_asset = Decimal::one();

        let expected_allocation = AssetAllocation::new(
            asset_id,
            symbol.clone(),
            name.into(),
            network,
            contract.clone(),
            expected_allocation_pct,
        );
        let expected_allocations_map: HashMap<AssetId, &AssetAllocation> =
            HashMap::from([(asset_id, &expected_allocation)]);

        let holding_id = HoldingId::new();
        let holdings = AssetHoldings::new(
            AssetRate::new(
                Asset::new(asset_id, symbol, name.into(), network, contract),
                asset_rate,
            ),
            asset_rate,
            vec![Holding::new(holding_id, amount_of_asset, asset_rate, None)],
        );

        let allocation_with_drift = PortfolioService::compute_drift_for_holding(
            holdings,
            &expected_allocations_map,
            total_portfolio_value_usd,
        );
        assert_eq!(allocation_with_drift.drift_pct, expected_drift);
        assert_eq!(
            allocation_with_drift.expected_allocation_pct,
            expected_allocation_pct
        );
        assert_eq!(
            allocation_with_drift.total_allocation_pct,
            expected_holding_pct
        );
    }

    #[test]
    fn drift_is_zero_when_actual_equals_target() {
        let total_portfolio_value_usd = decimal("1_000.0");
        let asset_rate = decimal("150.0"); // 15% of 1k
        let expected_drift = decimal("0.0");
        let expected_allocation_pct = decimal("0.15");
        let expected_holding_pct = decimal("0.15");

        test_fn(
            total_portfolio_value_usd,
            asset_rate,
            expected_drift,
            expected_allocation_pct,
            expected_holding_pct,
        )
    }

    #[test]
    fn drift_is_negative_when_actual_lower_than_target() {
        let total_portfolio_value_usd = decimal("1_000.0");
        let asset_rate = decimal("100.0"); // 10% of 1k
        let expected_drift = decimal("-0.05"); // 10% - 15%
        let expected_allocation_pct = decimal("0.15");
        let expected_holding_pct = decimal("0.1");

        test_fn(
            total_portfolio_value_usd,
            asset_rate,
            expected_drift,
            expected_allocation_pct,
            expected_holding_pct,
        )
    }

    #[test]
    fn drift_is_positive_when_actual_higher_than_target() {
        let total_portfolio_value_usd = decimal("1_000.0");
        let asset_rate = decimal("200.0"); // 20% of 1k
        let expected_drift = decimal("0.05"); // 20% - 15%
        let expected_allocation_pct = decimal("0.15");
        let expected_holding_pct = decimal("0.2");

        test_fn(
            total_portfolio_value_usd,
            asset_rate,
            expected_drift,
            expected_allocation_pct,
            expected_holding_pct,
        )
    }

    #[test]
    fn drift_is_positive_when_no_target() {
        let total_portfolio_value_usd = decimal("1_000.0");
        let asset_rate = decimal("200.0"); // 20% of 1k
        let expected_drift = decimal("0.2"); // 20% - 0%
        let expected_allocation_pct = decimal("0");
        let expected_holding_pct = decimal("0.2");

        let asset_id = AssetId::new();
        let symbol = Symbol::new("JITOSOL");
        let name = "Jito Staked Sol";
        let network = Network::Solana;
        let contract = Contract::from("J1toso1uCk3RLmjorhTtrVwY9HJ7X8V9yYac6Y7kGCPn");
        let amount_of_asset = Decimal::one();

        let expected_allocations_map: HashMap<AssetId, &AssetAllocation> = HashMap::from([]);

        let holding_id = HoldingId::new();
        let holdings = AssetHoldings::new(
            AssetRate::new(
                Asset::new(asset_id, symbol, name.into(), network, contract),
                asset_rate,
            ),
            asset_rate,
            vec![Holding::new(holding_id, amount_of_asset, asset_rate, None)],
        );

        let allocation_with_drift = PortfolioService::compute_drift_for_holding(
            holdings,
            &expected_allocations_map,
            total_portfolio_value_usd,
        );
        assert_eq!(allocation_with_drift.drift_pct, expected_drift);
        assert_eq!(
            allocation_with_drift.expected_allocation_pct,
            expected_allocation_pct
        );
        assert_eq!(
            allocation_with_drift.total_allocation_pct,
            expected_holding_pct
        );
    }
}
