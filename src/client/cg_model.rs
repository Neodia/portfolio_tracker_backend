use crate::client::error::ClientError;
use crate::client::mapper::map_cg_to_domain;
use crate::client::model::{BlockchainAssetRate, GetRatesFromNetworkResponse};
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
    pub price_usd: Option<Decimal>, // Think dead assets: They exist but rate is so low it's considered null
}

#[derive(Deserialize, Debug, Clone)]
pub struct CGTokenData {
    pub attributes: CGTokenAttribute,
}

#[derive(Deserialize)]
pub struct CGGetRatesFromNetworkResponse {
    data: Vec<CGTokenData>,
}

impl WithEnrichment<Network, GetRatesFromNetworkResponse> for CGGetRatesFromNetworkResponse {
    fn into_domain(self, network: Network) -> GetRatesFromNetworkResponse {
        let (rates, errors): (Vec<BlockchainAssetRate>, Vec<ClientError>) =
            self.data.into_iter().partition_map(|cg_token_data| {
                match map_cg_to_domain(cg_token_data.clone(), network) {
                    Some(data) => Left(data),
                    None => Right(ClientError::MissingAssetRateError(cg_token_data)),
                }
            });

        for error in errors {
            tracing::warn!(warn = ?error, "Error getting rates from network");
        }

        GetRatesFromNetworkResponse { rates }
    }
}
