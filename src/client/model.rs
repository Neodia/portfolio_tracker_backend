use crate::model::{Contract, Network, Symbol};
use rust_decimal::Decimal;
use serde::Deserialize;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Eq, Hash, Deserialize, Clone)]
pub struct BlockchainAssetPrice {
    pub symbol: Symbol,
    pub name: String,
    pub network: Network,
    pub contract: Contract,
    pub price: Decimal,
}

impl BlockchainAssetPrice {
    pub fn new(
        symbol: Symbol,
        name: String,
        network: Network,
        contract: Contract,
        price: Decimal,
    ) -> Self {
        Self {
            symbol,
            name,
            network,
            contract,
            price,
        }
    }
}
#[derive(Deserialize, Debug)]
pub struct GetPricesFromNetworkResponse {
    pub prices: Vec<BlockchainAssetPrice>,
}

impl Display for GetPricesFromNetworkResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for asset_price in &self.prices {
            writeln!(
                f,
                "{} {}({}): {}",
                asset_price.symbol, asset_price.network, asset_price.contract, asset_price.price
            )?;
        }
        Ok(())
    }
}
