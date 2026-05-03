use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::Serialize;

#[derive(Serialize, PartialEq, Debug)]
pub struct PortfolioValueAt {
    pub value_usd: Decimal,
    pub at: DateTime<Utc>,
}
impl PortfolioValueAt {
    pub fn new(value_usd: Decimal, at: DateTime<Utc>) -> Self {
        Self { value_usd, at }
    }
}