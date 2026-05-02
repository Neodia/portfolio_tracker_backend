use rust_decimal::Decimal;
use crate::model::AssetRate;
use crate::model::ids::HoldingId;

#[derive(PartialEq, Debug)]
pub struct Holding {
    pub id: HoldingId,
    pub amount: Decimal,
    pub value_usd: Decimal,
    pub description: Option<String>,
}
impl Holding {
    pub fn new(id: HoldingId, amount: Decimal, value_usd: Decimal, description: Option<String>) -> Self {
        Self {
            id,
            amount,
            value_usd,
            description,
        }
    }
}
#[derive(PartialEq, Debug)]
pub struct AssetHoldings {
    pub asset_rate: AssetRate,
    pub total_value_usd: Decimal,
    pub holdings: Vec<Holding>,
}
impl AssetHoldings {
    pub fn new(asset_rate: AssetRate, total_value_usd: Decimal, holdings: Vec<Holding>) -> Self {
        Self {
            asset_rate,
            total_value_usd,
            holdings,
        }
    }
}