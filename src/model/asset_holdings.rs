use rust_decimal::Decimal;
use crate::model::AssetPrice;
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
    pub asset_price: AssetPrice,
    pub total_value_usd: Decimal,
    pub holdings: Vec<Holding>,
}
impl AssetHoldings {
    pub fn new(asset_price: AssetPrice, total_value_usd: Decimal, holdings: Vec<Holding>) -> Self {
        Self {
            asset_price,
            total_value_usd,
            holdings,
        }
    }
}