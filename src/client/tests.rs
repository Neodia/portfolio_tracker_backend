use super::util::*;
use crate::client::cg_model::{CGTokenAttribute, CGTokenData};
use crate::client::mapper::map_cg_to_domain;
use crate::client::model::BlockchainAssetRate;
use crate::model::{Contract, Network, Symbol};
use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;

#[test]
fn join_as_csv_joins_with_comma() {
    assert_eq!(join_as_csv(&["a", "b", "c"]), "a,b,c");
}

#[test]
fn join_as_csv_single_element() {
    assert_eq!(join_as_csv(&["a"]), "a");
}

#[test]
fn join_as_csv_empty() {
    assert_eq!(join_as_csv::<&str>(&[]), "");
}

#[test]
fn map_cg_to_domain_works() {
    let symbol = Symbol("BTC".to_string());
    let name = "Bitcoin".to_string();
    let contract = Contract("0x123".to_string());
    let network = Network::Bitcoin;
    let rate_usd = Decimal::from_f64(75_000f64).unwrap();
    assert_eq!(
        map_cg_to_domain(
            CGTokenData {
                attributes: CGTokenAttribute {
                    symbol: symbol.clone(),
                    name: name.clone(),
                    address: contract.clone(),
                    price_usd: Some(rate_usd.clone()),
                }
            },
            network
        ),
        Some(BlockchainAssetRate::new(
            symbol, name, network, contract, rate_usd
        ))
    )
}
