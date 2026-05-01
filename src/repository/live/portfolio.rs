use crate::model::{AssetAllocation, AssetHoldings};
use crate::repository::error::DBError;
use crate::repository::model::{AssetAllocationDTO, HoldingDTO};
use crate::repository::traits::PortfolioRepository;
use chrono::{DateTime, Utc};
use itertools::Itertools;
use rust_decimal::Decimal;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone)]
pub struct LivePortfolioRepository {
    pool: PgPool,
}

impl LivePortfolioRepository {
    pub fn new_from_pool(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl PortfolioRepository for LivePortfolioRepository {
    async fn upsert_expected_asset_allocation(
        &self,
        user_id: Uuid,
        asset_id: Uuid,
        percentage: Decimal,
        now: DateTime<Utc>,
    ) -> Result<(), DBError> {
        sqlx::query!(
            "INSERT INTO expected_portfolio_allocations (user_id, asset_id, percentage, updated_at)
             VALUES ($1, $2, $3, $4)
             ON CONFLICT (user_id, asset_id)
             DO UPDATE SET percentage = $3, updated_at = $4",
            user_id,
            asset_id,
            percentage,
            now
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn delete_expected_asset_allocation(
        &self,
        user_id: Uuid,
        asset_id: Uuid,
    ) -> Result<(), DBError> {
        sqlx::query!(
            "DELETE FROM expected_portfolio_allocations
             WHERE user_id = $1
             AND asset_id = $2",
            user_id,
            asset_id,
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn get_expected_asset_allocations(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<AssetAllocation>, DBError> {
        let result = sqlx::query_as!(
            AssetAllocationDTO,
            "SELECT assets.id, assets.symbol, assets.name, assets.network, assets.contract_address, alloc.percentage as allocation
             FROM expected_portfolio_allocations as alloc
             INNER JOIN assets ON alloc.asset_id = assets.id
             WHERE alloc.user_id = $1
             ORDER BY alloc.percentage DESC",
            user_id,
        ).fetch_all(&self.pool).await?
            .into_iter()
            .map(TryInto::try_into)
            .collect::<Result<Vec<AssetAllocation>, DBError>>()?;
        Ok(result)
    }

    async fn insert_holding(
        &self,
        user_id: Uuid,
        asset_id: Uuid,
        amount: Decimal,
        description: Option<String>,
        now: DateTime<Utc>,
    ) -> Result<(), DBError> {
        sqlx::query!(
            "INSERT INTO current_holdings (id, user_id, asset_id, amount, description, updated_at)
             VALUES (gen_random_uuid(), $1, $2, $3, $4, $5)",
            user_id,
            asset_id,
            amount,
            description,
            now
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn update_holding(
        &self,
        holding_id: Uuid,
        user_id: Uuid,
        amount: Decimal,
        description: Option<String>,
        now: DateTime<Utc>,
    ) -> Result<(), DBError> {
        sqlx::query!(
            "UPDATE current_holdings
             SET amount = $1, description = $2, updated_at = $3
             WHERE id = $4
             AND user_id = $5",
            amount,
            description,
            now,
            holding_id,
            user_id,
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn delete_holding(&self, holding_id: Uuid, user_id: Uuid) -> Result<(), DBError> {
        sqlx::query!(
            "DELETE FROM current_holdings
             WHERE id = $1
             AND user_id = $2",
            holding_id,
            user_id,
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn get_holdings(&self, user_id: Uuid) -> Result<Vec<AssetHoldings>, DBError> {
        let holdings = sqlx::query_as!(
            HoldingDTO,
            "WITH latest_rates AS (
                SELECT DISTINCT ON (asset_id)
                    asset_id,
                    rate_usd
                FROM rates
                ORDER BY asset_id, rate_at DESC
            )
            SELECT assets.id as asset_id, assets.symbol, assets.name, assets.network, assets.contract_address, holding.amount, holding.description, latest_rates.rate_usd
            FROM current_holdings as holding
            INNER JOIN assets ON assets.id = holding.asset_id
            INNER JOIN latest_rates ON latest_rates.asset_id = holding.asset_id
            WHERE holding.user_id = $1"    ,
            user_id
        ).fetch_all(&self.pool)
            .await?
            .into_iter().into_group_map_by(|asset| asset.asset_id)
            .into_values().map(TryFrom::try_from)
            .collect::<Result<Vec<AssetHoldings>, DBError>>()?;
        Ok(holdings)
    }
}
