use crate::model::ids::{AssetId, HoldingId};
use crate::model::{Contract, Symbol};
use rust_decimal::Decimal;

pub struct BlockchainAssetDTO {
    pub id: AssetId,
    pub symbol: Symbol,
    pub name: String,
    pub network: String,
    pub contract_address: Contract,
}

pub struct AssetAllocationDTO {
    pub id: AssetId,
    pub symbol: Symbol,
    pub name: String,
    pub network: String,
    pub contract_address: Contract,
    pub allocation: Decimal // Percentage
}

pub struct HoldingDTO {
    pub id: HoldingId,
    pub asset_id: AssetId,
    pub symbol: Symbol,
    pub name: String,
    pub network: String,
    pub contract_address: Contract,
    pub amount: Decimal,
    pub description: Option<String>,
    pub rate_usd: Decimal,
}
