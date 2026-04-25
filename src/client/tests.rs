use super::util::*;
use crate::client::cg_model::{CGTokenAttribute, CGTokenData};
use crate::client::mapper::map_cg_to_domain;
use crate::model::{BlockchainAsset, Contract, Network, Symbol};
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;

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
    let contract = Contract("0x123".to_string());
    let network = Network::Bitcoin;
    let price_usd = Decimal::from_f64(75_000f64).unwrap();
    assert_eq!(
        map_cg_to_domain(
            CGTokenData {
                attributes: CGTokenAttribute {
                    symbol: symbol.clone(),
                    address: contract.clone(),
                    price_usd
                }
            },
            network.clone()
        ),
        (BlockchainAsset::new(symbol, network, contract), price_usd)
    )
}
