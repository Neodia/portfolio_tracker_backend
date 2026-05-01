use crate::common::{AssetFixture, DBFixture};
use portfolio_tracker_backend::model::Asset;
use portfolio_tracker_backend::repository::live::LiveAssetRepository;
use portfolio_tracker_backend::repository::AssetRepository;
use uuid::Uuid;

#[tokio::test]
async fn get_all_assets_returns_data_after_inserting() {
    let db = DBFixture::new().await;

    let Asset { id: _, symbol, name, network, contract_address } = AssetFixture::jitosol_test_asset();

    let repo = LiveAssetRepository::new_from_pool(db.pool.clone());
    repo.insert_asset(symbol.clone(), name.to_string(), network, contract_address.clone())
        .await
        .unwrap();
    let assets = repo.get_all_assets().await.unwrap();

    assert_eq!(
        assets,
        vec!(Asset::new(
            Uuid::nil(),
            symbol,
            name.to_string(),
            network,
            contract_address,
        ))
    );
}
