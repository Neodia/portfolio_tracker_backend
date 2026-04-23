mod client;

use crate::client::LiveCGClient;
use client::CGClient;
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = Client::new();

    let live_client: LiveCGClient = LiveCGClient::new(
        "https://api.coingecko.com/api/v3".into(),
        std::env::var("CG_KEY").expect("CG_KEY must be set"),
        client,
    );

    let response = live_client
        .get_simple_price(&["bitcoin", "ethereum"], &["usd", "eur"])
        .await?;

    for (coin, prices) in &response.0 {
        println!("{coin}:");
        for (currency, price) in prices {
            println!("  {currency}: {price}");
        }
    }

    Ok(())
}
