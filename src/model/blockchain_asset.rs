use crate::model::{Contract, Network, Symbol};

#[derive(Debug)]
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
