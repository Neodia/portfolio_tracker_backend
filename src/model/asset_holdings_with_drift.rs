use rust_decimal::Decimal;
use serde::Serialize;
use crate::model::AssetRate;
use crate::model::ids::HoldingId;

#[derive(Serialize, PartialEq, Debug)]
pub struct HoldingWithAllocation {
    pub id: HoldingId,
    pub amount: Decimal,
    pub value_usd: Decimal,
    pub description: Option<String>,
    pub current_allocation_pct: Decimal,
}
impl HoldingWithAllocation {
    pub fn new(
        id: HoldingId,
        amount: Decimal,
        value_usd: Decimal,
        description: Option<String>,
        current_allocation_pct: Decimal,
    ) -> Self {
        Self {
            id,
            amount,
            value_usd,
            description,
            current_allocation_pct,
        }
    }
}
#[derive(Serialize, PartialEq, Debug)]
pub struct AssetHoldingsWithDrift {
    pub asset_rate: AssetRate,
    pub total_value_usd: Decimal,
    pub holdings: Vec<HoldingWithAllocation>,
    pub total_allocation_pct: Decimal,
    pub expected_allocation_pct: Decimal,
    pub drift_pct: Decimal,
}
impl AssetHoldingsWithDrift {
    pub fn new(
        asset_rate: AssetRate,
        total_value_usd: Decimal,
        holdings: Vec<HoldingWithAllocation>,
        total_allocation_pct: Decimal,
        expected_allocation_pct: Decimal,
        drift_pct: Decimal,
    ) -> Self {
        Self {
            asset_rate,
            total_value_usd,
            holdings,
            total_allocation_pct,
            expected_allocation_pct,
            drift_pct,
        }
    }
}