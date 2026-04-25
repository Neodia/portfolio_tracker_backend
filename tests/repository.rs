use portfolio_tracker_backend::model::{BlockchainAsset, Network};
use portfolio_tracker_backend::repository::AssetRepository;
use portfolio_tracker_backend::repository::Repository;

mod common;

#[tokio::test]
async fn get_all_assets_returns_data() {
    let db = common::DBFixture::new().await;
    db.insert_assert("BTC", "bitcoin", "Native").await;

    let repo = AssetRepository::new_from_pool(db.pool);
    let assets = repo.get_all_assets().await.unwrap();

    assert_eq!(
        assets,
        vec!(BlockchainAsset::new(
            "BTC".to_string().into(),
            Network::Bitcoin,
            "Native".into()
        ))
    );
}
