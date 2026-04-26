use portfolio_tracker_backend::model::{Asset, Contract, Network};
use portfolio_tracker_backend::repository::live::AssetRepository;
use portfolio_tracker_backend::repository::Repository;
use uuid::Uuid;

mod common;

#[tokio::test]
async fn get_all_assets_returns_data() {
    let db = common::DBFixture::new().await;

    let symbol = "BTC";
    let name = "Bitcoin";
    let network = Network::Bitcoin;
    let contract: Contract = "Native".into();
    db.insert_assert(symbol, name, network.to_id(), contract.0.as_str()).await;

    let repo = AssetRepository::new_from_pool(db.pool);
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
