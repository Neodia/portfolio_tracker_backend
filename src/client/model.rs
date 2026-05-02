use crate::model::{Contract, Network, Symbol};
use rust_decimal::Decimal;
use serde::Deserialize;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Eq, Hash, Deserialize, Clone)]
pub struct BlockchainAssetRate {
    pub symbol: Symbol,
    pub name: String,
    pub network: Network,
    pub contract: Contract,
    pub rate: Decimal,
}

impl BlockchainAssetRate {
    pub fn new(
        symbol: Symbol,
        name: String,
        network: Network,
        contract: Contract,
        rate: Decimal,
    ) -> Self {
        Self {
            symbol,
            name,
            network,
            contract,
            rate,
        }
    }
}
#[derive(Deserialize, Debug)]
pub struct GetRatesFromNetworkResponse {
    pub rates: Vec<BlockchainAssetRate>,
}

impl Display for GetRatesFromNetworkResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for asset_rate in &self.rates {
            writeln!(
                f,
                "{} {}({}): {}",
                asset_rate.symbol, asset_rate.network, asset_rate.contract, asset_rate.rate
            )?;
        }
        Ok(())
    }
}
