

use std::fmt::Display;
use rust_decimal::Decimal;
use crate::client::cg_model::CGTokenData;
use crate::model::{BlockchainAsset, Network};

pub fn join_as_csv<T: Display>(v: &[T]) -> String {
    v.iter()
        .map(|c| c.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

pub fn map_cg_to_domain(
    cg_token_data: CGTokenData,
    network: Network,
) -> (BlockchainAsset, Decimal) {
    (
        BlockchainAsset::new(
            cg_token_data.attributes.symbol,
            network,
            cg_token_data.attributes.address,
        ),
        cg_token_data.attributes.price_usd,
    )
}