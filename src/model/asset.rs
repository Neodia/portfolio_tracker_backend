use crate::model::ids::AssetId;
use crate::model::{Contract, Network, Symbol};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Hash, Deserialize, Clone, Serialize)]
pub struct Asset {
    pub id: AssetId,
    pub symbol: Symbol,
    pub name: String,
    pub network: Network,
    pub contract_address: Contract,
}

impl Asset {
    pub fn new(
        id: AssetId,
        symbol: Symbol,
        name: String,
        network: Network,
        contract_address: Contract,
    ) -> Self {
        Self {
            id,
            symbol,
            name,
            network,
            contract_address,
        }
    }

    pub fn is_same_asset(&self, other: &Self) -> bool {
        // id intentionally excluded
        self.symbol == other.symbol
            && self.network == other.network
            && self.contract_address == other.contract_address
    }
}

impl Display for Asset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {}({})",
            self.symbol, self.network, self.contract_address
        )
    }
}
