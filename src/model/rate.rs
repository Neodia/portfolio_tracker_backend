use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use crate::model::ids::AssetId;

pub struct Rate {
    pub asset_id: AssetId,
    pub rate_usd: Decimal,
    pub at: DateTime<Utc>,
}