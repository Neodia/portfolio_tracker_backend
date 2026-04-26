use uuid::Uuid;
use crate::model::{Contract, Symbol};

pub struct BlockchainAssetDTO {
    pub id: Uuid,
    pub symbol: Symbol,
    pub name: String,
    pub network: String,
    pub contract_address: Contract,
}
