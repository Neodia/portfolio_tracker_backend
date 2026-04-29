use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use sqlx::PgTransaction;
use uuid::Uuid;
use crate::model::AssetPrice;
use crate::repository::error::DBError;
use crate::repository::RateRepository;

#[derive(Clone, Default)]
pub struct LiveRateRepository;
impl RateRepository for LiveRateRepository {
    async fn insert_rates(
        &self,
        tx: &mut PgTransaction<'_>,
        rates: Vec<AssetPrice>,
        now: DateTime<Utc>,
    ) -> Result<(), DBError> {
        let asset_ids: Vec<Uuid> = rates.iter().map(|r| r.asset.id).collect();
        let asset_rates: Vec<Decimal> = rates.iter().map(|r| r.price_usd).collect();

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
}