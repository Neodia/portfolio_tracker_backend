use crate::model::Asset;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Hash, Deserialize, Clone, Serialize)]
pub struct AssetRate {
    pub asset: Asset,
    pub rate_usd: Decimal,
}

impl AssetRate {
    pub fn new(asset: Asset, rate_usd: Decimal) -> Self {
        Self { asset, rate_usd }
    }
}
