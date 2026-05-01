use crate::model::AssetHoldingsWithDrift;
use rust_decimal::Decimal;
use serde::Serialize;

#[derive(Serialize)]
pub struct PortfolioHoldings {
    pub holdings: Vec<AssetHoldingsWithDrift>,
    pub portfolio_value_usd: Decimal,
}
impl PortfolioHoldings {
    pub fn new(holdings: Vec<AssetHoldingsWithDrift>, portfolio_value_usd: Decimal) -> Self {
        Self {
            holdings,
            portfolio_value_usd,
        }
    }
}
