use crate::model::{Contract, Symbol};
use uuid::Uuid;

pub struct BlockchainAssetDTO {
    pub id: Uuid,
    pub symbol: Symbol,
    pub name: String,
    pub network: String,
    pub contract_address: Contract,
}
