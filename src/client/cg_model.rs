use crate::client::model::GetPricesFromNetworkResponse;
use crate::model::Symbol;
use crate::model::contract::Contract;
use crate::model::token_on_chain::TokenOnChain;
use rust_decimal::Decimal;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct CGTokenAttribute {
    address: Contract,
    symbol: Symbol,
    price_usd: Decimal,
}

#[derive(Deserialize)]
pub struct CGTokenData {
    attributes: CGTokenAttribute,
}

impl From<CGTokenData> for (TokenOnChain, Decimal) {
    fn from(value: CGTokenData) -> Self {
        let token_on_chain = TokenOnChain::new(value.attributes.symbol, value.attributes.address);
        (token_on_chain, value.attributes.price_usd)
    }
}

#[derive(Deserialize)]
pub struct CGGetPricesFromNetworkResponse {
    data: Vec<CGTokenData>,
}

impl From<CGGetPricesFromNetworkResponse> for GetPricesFromNetworkResponse {
    fn from(value: CGGetPricesFromNetworkResponse) -> Self {
        GetPricesFromNetworkResponse {
            prices: value
                .data
                .into_iter()
                .map(|data| data.into())
                .collect::<HashMap<TokenOnChain, Decimal>>(),
        }
    }
}
