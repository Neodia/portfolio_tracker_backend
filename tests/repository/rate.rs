use crate::common::{AssetFixture, DBFixture};
use chrono::Utc;
use portfolio_tracker_backend::model::{Asset, AssetPrice};
use portfolio_tracker_backend::repository::live::LiveRateRepository;
use portfolio_tracker_backend::repository::RateRepository;
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;

#[tokio::test]
async fn insert_rates_works() {
    let db = DBFixture::new().await;
    
    let Asset { id: _, symbol, name, network, contract_address } = AssetFixture::jitosol_test_asset();

    let asset_id = db
        .insert_asset(symbol.0.as_str(), name.as_str(), network.to_id(), contract_address.0.as_str())
        .await;

    let asset_price = AssetPrice::new(
        Asset::new(
            asset_id,
            symbol,
            name.to_string(),
            network,
            contract_address,
        ),
        Decimal::from_f64(75_000f64).unwrap(),
    );
    let now = Utc::now();

    let repo = LiveRateRepository::default();
    let mut tx = db.pool.begin().await.unwrap();
    repo.insert_rates(&mut tx, vec![asset_price.clone()], now)
        .await
        .unwrap();
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
