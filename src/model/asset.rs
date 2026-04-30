use crate::model::{Contract, Network, Symbol};
use serde::Deserialize;
use std::fmt::Display;
use uuid::Uuid;

#[derive(Debug, Eq, Hash, Deserialize, Clone)]
pub struct Asset {
    pub id: Uuid,
    pub symbol: Symbol,
    pub name: String,
    pub network: Network,
    pub contract_address: Contract,
}

impl Asset {
    pub fn new(
        id: Uuid,
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
}

impl PartialEq for Asset {
    fn eq(&self, other: &Self) -> bool {
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
