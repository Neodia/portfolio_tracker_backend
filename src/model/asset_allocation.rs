use crate::model::ids::AssetId;
use crate::model::{Asset, Contract, Network, Symbol};
use rust_decimal::Decimal;
use serde::Serialize;

#[derive(Serialize, PartialEq, Debug)]
pub struct AssetAllocation {
    pub asset: Asset,
    pub allocation_pct: Decimal, // Percentage
}
impl AssetAllocation {
    pub fn new(
        id: AssetId,
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