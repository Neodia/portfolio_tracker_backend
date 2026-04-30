use crate::common::DBFixture;
use chrono::Utc;
use portfolio_tracker_backend::model::{Asset, AssetPrice, Contract, Network, Symbol};
use portfolio_tracker_backend::repository::live::LiveRateRepository;
use portfolio_tracker_backend::repository::RateRepository;
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;

#[tokio::test]
async fn get_all_assets_returns_data() {
    let db = DBFixture::new().await;

    let symbol = "BTC";
    let name = "Bitcoin";
    let network = Network::Bitcoin;
    let contract: Contract = "Native".into();
    let asset_id = db.insert_asset(symbol, name, network.to_id(), contract.0.as_str())
        .await;

    let asset_price = AssetPrice::new(
        Asset::new(asset_id, Symbol::new(symbol), name.to_string(), network, contract), Decimal::from_f64(75_000f64).unwrap(),
    );
    let now = Utc::now();

    let repo = LiveRateRepository::default();
    let mut tx = db.pool.begin().await.unwrap();
    let assets = repo.insert_rates(&mut tx, vec!(asset_price.clone()), now).await.unwrap();
    tx.commit().await.unwrap();
    
    let resp = sqlx::query!("SELECT * FROM rates;")
        .fetch_one(&db.pool)
        .await
        .unwrap();

    assert_eq!(
        (resp.asset_id, resp.rate_usd, resp.rate_at),
        (asset_id, asset_price.price_usd, now)
    );
}
