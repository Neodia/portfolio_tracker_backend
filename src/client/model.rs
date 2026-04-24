use super::util::vec_to_csv_format;
use crate::model::Symbol;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use crate::model::token_on_chain::TokenOnChain;

#[derive(Serialize)]
pub struct GetSimplePriceRequest {
    #[serde(serialize_with = "vec_to_csv_format")]
    ids: Vec<String>,
    #[serde(serialize_with = "vec_to_csv_format")]
    vs_currencies: Vec<String>,
}

impl GetSimplePriceRequest {
    pub fn new(ids: Vec<impl Into<String>>, vs_currencies: Vec<impl Into<String>>) -> Self {
        Self {
            ids: ids.into_iter().map(Into::into).collect(),
            vs_currencies: vs_currencies.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct GetSimplePriceResponse(pub HashMap<Symbol, HashMap<Symbol, Decimal>>);

impl Display for GetSimplePriceResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (coin, prices) in &self.0 {
            writeln!(f, "{coin}:")?;
            for (currency, price) in prices {
                writeln!(f, "  {currency}: {price}", currency = currency.0)?;
            }
        }
        Ok(())
    }
}

#[derive(Deserialize, Debug)]
pub struct GetPricesFromNetworkResponse {
    pub prices: HashMap<TokenOnChain, Decimal>,
}

impl Display for GetPricesFromNetworkResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (token, price) in &self.prices {
            writeln!(f, "{token}: {price}")?;
        }
        Ok(())
    }
}