use crate::model::ids::{AssetId, HoldingId, UserId};
use crate::model::{AssetAllocation, AssetHoldings, PortfolioValueAt, UserHolding};
use crate::repository::error::DBError;
use crate::repository::model::{AssetAllocationDTO, HoldingDTO};
use crate::repository::traits::PortfolioRepository;
use chrono::{DateTime, Utc};
use itertools::Itertools;
use rust_decimal::Decimal;
use sqlx::{PgPool, PgTransaction};
use std::collections::HashMap;
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
        user_id: UserId,
        asset_id: AssetId,
        percentage: Decimal,
        now: DateTime<Utc>,
    ) -> Result<(), DBError> {
        sqlx::query!(
            "INSERT INTO expected_portfolio_allocations (user_id, asset_id, percentage, updated_at)
             VALUES ($1, $2, $3, $4)
             ON CONFLICT (user_id, asset_id)
             DO UPDATE SET percentage = $3, updated_at = $4",
            user_id.0,
            asset_id.0,
            percentage,
            now
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn delete_expected_asset_allocation(
        &self,
        user_id: UserId,
        asset_id: AssetId,
    ) -> Result<(), DBError> {
        sqlx::query!(
            "DELETE FROM expected_portfolio_allocations
             WHERE user_id = $1
             AND asset_id = $2",
            user_id.0,
            asset_id.0,
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn get_expected_asset_allocations(
        &self,
        user_id: UserId,
    ) -> Result<Vec<AssetAllocation>, DBError> {
        let result = sqlx::query_as!(
            AssetAllocationDTO,
            "SELECT assets.id, assets.symbol, assets.name, assets.network, assets.contract_address, alloc.percentage as allocation
             FROM expected_portfolio_allocations as alloc
             INNER JOIN assets ON alloc.asset_id = assets.id
             WHERE alloc.user_id = $1
             ORDER BY alloc.percentage DESC, assets.symbol",
            user_id.0,
        ).fetch_all(&self.pool).await?
            .into_iter()
            .map(TryInto::try_into)
            .collect::<Result<Vec<AssetAllocation>, DBError>>()?;
        Ok(result)
    }

    async fn insert_holding(
        &self,
        user_id: UserId,
        asset_id: AssetId,
        amount: Decimal,
        description: Option<String>,
        now: DateTime<Utc>,
    ) -> Result<HoldingId, DBError> {
        let id = sqlx::query_scalar!(
            "INSERT INTO current_holdings (id, user_id, asset_id, amount, description, updated_at)
             VALUES (gen_random_uuid(), $1, $2, $3, $4, $5)
             RETURNING id",
            user_id.0,
            asset_id.0,
            amount,
            description,
            now
        )
        .fetch_one(&self.pool)
        .await
        .map(From::from)?;
        Ok(id)
    }

    async fn update_holding(
        &self,
        holding_id: HoldingId,
        user_id: UserId,
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
            holding_id.0,
            user_id.0,
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn delete_holding(&self, holding_id: HoldingId, user_id: UserId) -> Result<(), DBError> {
        sqlx::query!(
            "DELETE FROM current_holdings
             WHERE id = $1
             AND user_id = $2",
            holding_id.0,
            user_id.0,
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn get_holdings(&self, user_id: UserId) -> Result<Vec<AssetHoldings>, DBError> {
        let holdings = sqlx::query_as!(
            HoldingDTO,
            "WITH latest_rates AS (
                SELECT DISTINCT ON (asset_id)
                    asset_id,
                    rate_usd
                FROM rates
                ORDER BY asset_id, rate_at DESC
            )
            SELECT holding.id, assets.id as asset_id, assets.symbol, assets.name, assets.network, assets.contract_address, holding.amount, holding.description, latest_rates.rate_usd
            FROM current_holdings as holding
            INNER JOIN assets ON assets.id = holding.asset_id
            INNER JOIN latest_rates ON latest_rates.asset_id = holding.asset_id
            WHERE holding.user_id = $1",
            user_id.0
        ).fetch_all(&self.pool)
            .await?
            .into_iter().into_group_map_by(|asset| asset.asset_id)
            .into_values().map(TryFrom::try_from)
            .collect::<Result<Vec<AssetHoldings>, DBError>>()?;
        Ok(holdings)
    }

    async fn get_all_users_holdings(&self) -> Result<HashMap<UserId, Vec<UserHolding>>, DBError> {
        let user_holdings = sqlx::query_as!(
            UserHolding,
            "SELECT id, user_id, asset_id, amount, description FROM current_holdings"
        )
        .fetch_all(&self.pool)
        .await?
        .into_iter()
        .into_group_map_by(|holding| holding.user_id);
        Ok(user_holdings)
    }

    async fn insert_portfolio_snapshots(
        &self,
        tx: &mut PgTransaction<'_>,
        user_snapshots: Vec<(&UserId, Decimal)>,
        at: DateTime<Utc>,
    ) -> Result<(), DBError> {
        let (user_ids, snapshots_value_usd): (Vec<Uuid>, Vec<Decimal>) = user_snapshots
            .iter()
            .map(|(user, value)| (user.0, *value))
            .unzip();
        sqlx::query!(
            "INSERT INTO portfolio_snapshots (user_id, value_usd, at)
             SELECT * FROM UNNEST($1::uuid[], $2::numeric[], $3::timestamptz[])",
            &user_ids as &[Uuid],
            &snapshots_value_usd as &[Decimal],
            &vec![at; user_ids.len()] as &[DateTime<Utc>]
        )
        .execute(tx.as_mut())
        .await?;
        Ok(())
    }

    async fn get_historical_portfolio_values(
        &self,
        user_id: UserId,
    ) -> Result<Vec<PortfolioValueAt>, DBError> {
        let values = sqlx::query_as!(
            PortfolioValueAt,
            "SELECT value_usd, at
             FROM portfolio_snapshots
             WHERE user_id = $1
             ORDER BY at DESC",
            user_id.0
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(values)
    }
}
