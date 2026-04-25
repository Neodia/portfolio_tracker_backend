use crate::client::util::map_cg_to_domain;
use crate::client::model::GetPricesFromNetworkResponse;
use crate::model::contract::Contract;
use crate::model::{BlockchainAsset, Network, Symbol};
use rust_decimal::Decimal;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct CGTokenAttribute {
    pub address: Contract,
    pub symbol: Symbol,
    pub price_usd: Decimal,
}

#[derive(Deserialize)]
pub struct CGTokenData {
    pub attributes: CGTokenAttribute,
}

#[derive(Deserialize)]
pub struct CGGetPricesFromNetworkResponse {
    data: Vec<CGTokenData>,
}

impl CGGetPricesFromNetworkResponse {
    pub fn into_domain(self, network: Network) -> GetPricesFromNetworkResponse {
        GetPricesFromNetworkResponse {
            prices: self
                .data
                .into_iter()
                .map(|cg_token_data| map_cg_to_domain(cg_token_data, network.clone()))
                .collect::<HashMap<BlockchainAsset, Decimal>>(),
        }
    }
}
