use crate::common::DBFixture;
use portfolio_tracker_backend::model::{Asset, Contract, Network};
use portfolio_tracker_backend::repository::AssetRepository;
use portfolio_tracker_backend::repository::live::LiveAssetRepository;
use uuid::Uuid;

#[tokio::test]
async fn get_all_assets_returns_data() {
    let db = DBFixture::new().await;

    let symbol = "BTC";
    let name = "Bitcoin";
    let network = Network::Bitcoin;
    let contract: Contract = "Native".into();
    db.insert_asset(symbol, name, network.to_id(), contract.0.as_str())
        .await;

    let repo = LiveAssetRepository::new_from_pool(db.pool.clone());
    let assets = repo.get_all_assets().await.unwrap();

    assert_eq!(
        assets,
        vec!(Asset::new(
            Uuid::nil(),
            symbol.to_string().into(),
            name.to_string(),
            Network::Bitcoin,
            contract,
        ))
    );
}
