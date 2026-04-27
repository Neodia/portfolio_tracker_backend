use crate::model::{Asset, Contract, Network, Symbol};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Clone, Debug)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
}
#[derive(Deserialize, Clone, Debug)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}
#[derive(Serialize)]
pub struct TokenResponse {
    pub token: String,
    pub token_type: String, // always "Bearer"
}
impl TokenResponse {
    pub fn new(token: String) -> Self {
        Self {
            token,
            token_type: "Bearer".to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Deserialize, Clone, Serialize)]
pub struct NetworkResponse {
    pub id: String,
    pub display_name: String,
}

impl From<Network> for NetworkResponse {
    fn from(network: Network) -> Self {
        Self {
            id: network.to_id().to_string(),
            display_name: network.to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Deserialize, Clone, Serialize)]
pub struct AssetResponse {
    pub id: Uuid,
    pub symbol: Symbol,
    pub name: String,
    pub network: NetworkResponse,
    pub contract_address: Contract,
}

impl From<Asset> for AssetResponse {
    fn from(asset: Asset) -> Self {
        Self {
            id: asset.id,
            symbol: asset.symbol,
            name: asset.name,
            network: asset.network.into(),
            contract_address: asset.contract_address,
        }
    }
}
