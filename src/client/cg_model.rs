use crate::client::error::ClientError;
use crate::client::mapper::map_cg_to_domain;
use crate::client::model::{BlockchainAssetPrice, GetPricesFromNetworkResponse};
use crate::client::util::WithEnrichment;
use crate::model::Contract;
use crate::model::{Network, Symbol};
use itertools::Either::{Left, Right};
use itertools::Itertools;
use rust_decimal::Decimal;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct CGTokenAttribute {
    pub address: Contract,
    pub symbol: Symbol,
    pub name: String,
    pub price_usd: Option<Decimal>, // Think dead assets: They exist but price is so low it's considered null
}

#[derive(Deserialize, Debug, Clone)]
pub struct CGTokenData {
    pub attributes: CGTokenAttribute,
}

#[derive(Deserialize)]
pub struct CGGetPricesFromNetworkResponse {
    data: Vec<CGTokenData>,
}

impl WithEnrichment<Network, GetPricesFromNetworkResponse> for CGGetPricesFromNetworkResponse {
    fn into_domain(self, network: Network) -> GetPricesFromNetworkResponse {
        let (prices, errors): (Vec<BlockchainAssetPrice>, Vec<ClientError>) =
            self.data.into_iter().partition_map(|cg_token_data| {
                match map_cg_to_domain(cg_token_data.clone(), network) {
                    Some(data) => Left(data),
                    None => Right(ClientError::MissingAssetPriceError(cg_token_data)),
                }
            });

        for error in errors {
            tracing::warn!(warn = ?error, "Error getting prices from network");
        }

        GetPricesFromNetworkResponse { prices }
    }
}
