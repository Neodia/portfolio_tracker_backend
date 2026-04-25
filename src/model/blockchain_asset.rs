use crate::model::{Contract, Network, Symbol};
use std::fmt::Display;
use serde::Deserialize;

#[derive(Debug, PartialEq, Eq, Hash, Deserialize, Clone)]
pub struct BlockchainAsset {
    pub symbol: Symbol,
    pub network: Network,
    pub contract: Contract,
}

impl BlockchainAsset {
    pub fn new(symbol: Symbol, network: Network, contract: Contract) -> Self {
        Self {
            symbol,
            network,
            contract,
        }
    }
}

impl Display for BlockchainAsset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}({})", self.symbol, self.network, self.contract)
    }
}
