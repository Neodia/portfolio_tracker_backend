use crate::model::Asset;
use rust_decimal::Decimal;
use serde::Deserialize;

#[derive(Debug, PartialEq, Eq, Hash, Deserialize, Clone)]
pub struct AssetPrice {
    pub asset: Asset,
    pub price_usd: Decimal,
}

impl AssetPrice {
    pub fn new(asset: Asset, price_usd: Decimal) -> Self {
        Self { asset, price_usd }
    }
}
