use crate::common::DBFixture;
use portfolio_tracker_backend::model::{Asset, Contract, Network, Symbol};
use portfolio_tracker_backend::repository::AssetRepository;
use portfolio_tracker_backend::repository::live::LiveAssetRepository;
use uuid::Uuid;

#[tokio::test]
async fn get_all_assets_returns_data_after_inserting() {
    let db = DBFixture::new().await;

    let symbol = Symbol::new("BTC");
    let name = "Bitcoin";
    let network = Network::Bitcoin;
    let contract: Contract = "Native".into();

    let repo = LiveAssetRepository::new_from_pool(db.pool.clone());
    repo.insert_asset(symbol.clone(), name.to_string(), network, contract.clone())
        .await
        .unwrap();
    let assets = repo.get_all_assets().await.unwrap();

    assert_eq!(
        assets,
        vec!(Asset::new(
            Uuid::nil(),
            symbol,
            name.to_string(),
            Network::Bitcoin,
            contract,
        ))
    );
}
