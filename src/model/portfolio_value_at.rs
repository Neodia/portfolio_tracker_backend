use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::Serialize;

#[derive(Serialize)]
pub struct PortfolioValueAt {
    pub value_usd: Decimal,
    pub at: DateTime<Utc>,
}
