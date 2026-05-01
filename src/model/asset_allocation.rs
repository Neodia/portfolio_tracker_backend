use rust_decimal::Decimal;
use serde::Serialize;
use uuid::Uuid;
use crate::model::{Asset, Contract, Network, Symbol};

#[derive(Serialize)]
pub struct AssetAllocation {
    pub asset: Asset,
    pub allocation_pct: Decimal, // Percentage
}
impl AssetAllocation {
    pub fn new(
        id: Uuid,
        symbol: Symbol,
        name: String,
        network: Network,
        contract: Contract,
        allocation_pct: Decimal,
    ) -> Self {
        Self {
            asset: Asset::new(id, symbol, name, network, contract),
            allocation_pct,
        }
    }
}