use crate::model::{Contract, Symbol};

#[derive(sqlx::FromRow)]
pub struct BlockchainAssetDTO {
    pub ticker: Symbol,
    pub chain: String,
    pub contract_address: Contract,
}
