use crate::model::error::AppError;
use crate::model::{
    AssetAllocation, AssetHoldings, AssetHoldingsWithDrift, HoldingWithAllocation,
    PortfolioHoldings, PortfolioResponse,
};
use crate::repository::traits::PortfolioRepository;
use crate::repository::Repositories;
use chrono::Utc;
use rust_decimal::prelude::Zero;
use rust_decimal::Decimal;
use std::collections::HashMap;
use itertools::Itertools;
use uuid::Uuid;

#[derive(Clone)]
pub struct PortfolioService {
    repositories: Repositories,
}
impl PortfolioService {
    pub fn new(repositories: Repositories) -> Self {
        Self { repositories }
    }
    pub async fn upsert_expected_asset_allocation(
        &self,
        user_id: Uuid,
        asset_id: Uuid,
        percentage: Decimal,
    ) -> Result<(), AppError> {
        let now = Utc::now();
        self.repositories
            .portfolio
            .upsert_expected_asset_allocation(user_id, asset_id, percentage, now)
            .await?;
        Ok(())
    }
    pub async fn delete_expected_asset_allocation(
        &self,
        user_id: Uuid,
        asset_id: Uuid,
    ) -> Result<(), AppError> {
        self.repositories
            .portfolio
            .delete_expected_asset_allocation(user_id, asset_id)
            .await?;
        Ok(())
    }

    pub async fn insert_holding(
        &self,
        user_id: Uuid,
        asset_id: Uuid,
        amount: Decimal,
        description: Option<String>,
    ) -> Result<(), AppError> {
        let now = Utc::now();
        self.repositories
            .portfolio
            .insert_holding(user_id, asset_id, amount, description, now)
            .await?;
        Ok(())
    }
    pub async fn update_holding(
        &self,
        user_id: Uuid,
        holding_id: Uuid,
        amount: Decimal,
        description: Option<String>,
    ) -> Result<(), AppError> {
        let now = Utc::now();
        self.repositories
            .portfolio
            .update_holding(holding_id, user_id, amount, description, now)
            .await?;
        Ok(())
    }
    pub async fn delete_holding(&self, user_id: Uuid, holding_id: Uuid) -> Result<(), AppError> {
        self.repositories
            .portfolio
            .delete_holding(user_id, holding_id)
            .await?;
        Ok(())
    }

    pub async fn get_portfolio(&self, user_id: Uuid) -> Result<PortfolioResponse, AppError> {
        let (expected_allocations, holdings) = tokio::try_join!(
            self.repositories
                .portfolio
                .get_expected_asset_allocations(user_id),
            self.repositories.portfolio.get_holdings(user_id),
        )?;

        let expected_allocations_map: HashMap<Uuid, &AssetAllocation> = expected_allocations
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

        Ok(PortfolioResponse::new(
            expected_allocations,
            user_holdings,
            vec![], // TODO: Get this from DB when historical portfolio job is done
        ))
    }

    fn compute_drift_for_holding(
        holdings: AssetHoldings,
        expected_allocations_map: &HashMap<Uuid, &AssetAllocation>,
        total_portfolio_value_usd: Decimal,
    ) -> AssetHoldingsWithDrift {
        let expected_allocation_opt = expected_allocations_map
            .get(&holdings.asset_price.asset.id)
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
            holdings.asset_price,
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
    use crate::model::{Asset, AssetPrice, Contract, Holding, Network, Symbol};
    use rust_decimal::prelude::One;

    // str for exact precision
    fn decimal(number: &str) -> Decimal {
        Decimal::from_str_exact(number).unwrap()
    }

    fn test_fn(
        total_portfolio_value_usd: Decimal,
        asset_price: Decimal,
        expected_drift: Decimal,
        expected_allocation_pct: Decimal,
        expected_holding_pct: Decimal,
    ) {
        let asset_uuid = Uuid::new_v4();
        let symbol = Symbol::new("JITOSOL");
        let name = "Jito Staked Sol";
        let network = Network::Solana;
        let contract = Contract::from("J1toso1uCk3RLmjorhTtrVwY9HJ7X8V9yYac6Y7kGCPn");
        let amount_of_asset = Decimal::one();

        let expected_allocation = AssetAllocation::new(
            asset_uuid,
            symbol.clone(),
            name.into(),
            network,
            contract.clone(),
            expected_allocation_pct,
        );
        let expected_allocations_map: HashMap<Uuid, &AssetAllocation> =
            HashMap::from([(asset_uuid, &expected_allocation)]);

        let holdings = AssetHoldings::new(
            AssetPrice::new(
                Asset::new(asset_uuid, symbol, name.into(), network, contract),
                asset_price,
            ),
            asset_price,
            vec![Holding::new(amount_of_asset, asset_price, None)],
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
        let asset_price = decimal("150.0"); // 15% of 1k
        let expected_drift = decimal("0.0");
        let expected_allocation_pct = decimal("0.15");
        let expected_holding_pct = decimal("0.15");

        test_fn(
            total_portfolio_value_usd,
            asset_price,
            expected_drift,
            expected_allocation_pct,
            expected_holding_pct,
        )
    }

    #[test]
    fn drift_is_negative_when_actual_lower_than_target() {
        let total_portfolio_value_usd = decimal("1_000.0");
        let asset_price = decimal("100.0"); // 10% of 1k
        let expected_drift = decimal("-0.05"); // 10% - 15%
        let expected_allocation_pct = decimal("0.15");
        let expected_holding_pct = decimal("0.1");

        test_fn(
            total_portfolio_value_usd,
            asset_price,
            expected_drift,
            expected_allocation_pct,
            expected_holding_pct,
        )
    }

    #[test]
    fn drift_is_positive_when_actual_higher_than_target() {
        let total_portfolio_value_usd = decimal("1_000.0");
        let asset_price = decimal("200.0"); // 20% of 1k
        let expected_drift = decimal("0.05"); // 20% - 15%
        let expected_allocation_pct = decimal("0.15");
        let expected_holding_pct = decimal("0.2");

        test_fn(
            total_portfolio_value_usd,
            asset_price,
            expected_drift,
            expected_allocation_pct,
            expected_holding_pct,
        )
    }

    #[test]
    fn drift_is_positive_when_no_target() {
        let total_portfolio_value_usd = decimal("1_000.0");
        let asset_price = decimal("200.0"); // 20% of 1k
        let expected_drift = decimal("0.2"); // 20% - 0%
        let expected_allocation_pct = decimal("0");
        let expected_holding_pct = decimal("0.2");
        
        let asset_uuid = Uuid::new_v4();
        let symbol = Symbol::new("JITOSOL");
        let name = "Jito Staked Sol";
        let network = Network::Solana;
        let contract = Contract::from("J1toso1uCk3RLmjorhTtrVwY9HJ7X8V9yYac6Y7kGCPn");
        let amount_of_asset = Decimal::one();
        
        let expected_allocations_map: HashMap<Uuid, &AssetAllocation> =
            HashMap::from([]);

        let holdings = AssetHoldings::new(
            AssetPrice::new(
                Asset::new(asset_uuid, symbol, name.into(), network, contract),
                asset_price,
            ),
            asset_price,
            vec![Holding::new(amount_of_asset, asset_price, None)],
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
