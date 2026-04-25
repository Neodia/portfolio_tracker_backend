use crate::model::BlockchainAsset;
use rust_decimal::Decimal;
use serde::Deserialize;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

#[derive(Deserialize, Debug)]
pub struct GetPricesFromNetworkResponse {
    pub prices: HashMap<BlockchainAsset, Decimal>,
}

impl Display for GetPricesFromNetworkResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (token, price) in &self.prices {
            writeln!(f, "{token}: {price}")?;
        }
        Ok(())
    }
}
