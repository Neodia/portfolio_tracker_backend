use crate::client::mapper::map_cg_to_domain;
use crate::client::model::{BlockchainAssetPrice, GetPricesFromNetworkResponse};
use crate::client::util::WithEnrichment;
use crate::model::Contract;
use crate::model::{Network, Symbol};
use rust_decimal::Decimal;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CGTokenAttribute {
    pub address: Contract,
    pub symbol: Symbol,
    pub name: String,
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

impl WithEnrichment<Network, GetPricesFromNetworkResponse> for CGGetPricesFromNetworkResponse {
    fn into_domain(self, network: Network) -> GetPricesFromNetworkResponse {
        GetPricesFromNetworkResponse {
            prices: self
                .data
                .into_iter()
                .map(|cg_token_data| map_cg_to_domain(cg_token_data, network.clone()))
                .collect::<Vec<BlockchainAssetPrice>>(),
        }
    }
}
