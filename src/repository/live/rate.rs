use crate::model::ids::AssetId;
use crate::model::{AssetRate, Rate};
use crate::repository::RateRepository;
use crate::repository::error::DBError;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use sqlx::{PgPool, PgTransaction};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Clone)]
pub struct LiveRateRepository {
    pool: PgPool,
}
impl LiveRateRepository {
    pub fn new_from_pool(pool: PgPool) -> Self {
        Self { pool }
    }
}
impl RateRepository for LiveRateRepository {
    async fn insert_rates(
        &self,
        tx: &mut PgTransaction<'_>,
        rates: Vec<AssetRate>,
        now: DateTime<Utc>,
    ) -> Result<(), DBError> {
        let asset_ids: Vec<Uuid> = rates.iter().map(|r| r.asset.id.0).collect();
        let asset_rates: Vec<Decimal> = rates.iter().map(|r| r.rate_usd).collect();

        sqlx::query!(
            "INSERT INTO rates (asset_id, rate_usd, rate_at)
     SELECT * FROM UNNEST($1::uuid[], $2::numeric[], $3::timestamptz[])",
            &asset_ids as &[Uuid],
            &asset_rates as &[Decimal],
            &vec![now; asset_ids.len()] as &[DateTime<Utc>]
        )
        .execute(tx.as_mut())
        .await?;

        Ok(())
    }

    async fn get_latest_asset_rates_at(
        &self,
        at: DateTime<Utc>,
    ) -> Result<HashMap<AssetId, Rate>, DBError> {
        let rates: HashMap<AssetId, Rate> = sqlx::query_as!(
            Rate,
            "SELECT DISTINCT ON (asset_id) asset_id, rate_usd, rate_at as at
             FROM rates 
             WHERE rate_at <= $1 
             ORDER BY asset_id, rate_at DESC",
            at,
        )
        .fetch_all(&self.pool)
        .await?
        .into_iter()
        .map(|r| (r.asset_id, r))
        .collect();

        Ok(rates)
    }
}
