use crate::model::{Contract, Symbol};
use rust_decimal::Decimal;
use uuid::Uuid;

pub struct BlockchainAssetDTO {
    pub id: Uuid,
    pub symbol: Symbol,
    pub name: String,
    pub network: String,
    pub contract_address: Contract,
}

pub struct AssetAllocationDTO {
    pub id: Uuid,
    pub symbol: Symbol,
    pub name: String,
    pub network: String,
    pub contract_address: Contract,
    pub allocation: Decimal // Percentage
}

pub struct HoldingDTO {
    pub asset_id: Uuid,
    pub symbol: Symbol,
    pub name: String,
    pub network: String,
    pub contract_address: Contract,
    pub amount: Decimal,
    pub description: Option<String>,
    pub rate_usd: Decimal,
}