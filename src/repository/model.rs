use uuid::Uuid;
use crate::model::{Contract, Symbol};

#[derive(sqlx::FromRow)]
pub struct BlockchainAssetDTO {
    pub _id: Uuid,
    pub symbol: Symbol,
    pub name: String,
    pub network: String,
    pub contract_address: Contract,
}
