mod client;
mod model;

use crate::client::LiveCGClient;
use crate::client::error::ClientError;
use crate::model::Network;
use crate::model::contract::Contract;
use client::CGClient;
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), ClientError> {

    let live_client: LiveCGClient = LiveCGClient::new(
        "https://api.coingecko.com/api/v3".into(),
        std::env::var("CG_KEY").expect("CG_KEY must be set"),
    );

    let response = live_client
        .get_simple_price(&["bitcoin", "ethereum"], &["usd", "eur"])
        .await?;

    println!("{}", response);

    let contracts = [Contract::from(
        "6p6xgHyF7AeE6TZkSmFsko444wqoP15icUSqi2jfGiPN",
    )];
    let response = live_client
        .get_prices_from_network(Network::Solana, &contracts)
        .await?;

    println!("{}", response);

    Ok(())
}
