use crate::common::{AssetFixture, TestApp};
use itertools::Itertools;
use portfolio_tracker_backend::model::Asset;
use wiremock::MockServer;

#[tokio::test]
async fn insert_and_get_asset_should_work() {
    let Asset {
        id: _,
        symbol,
        name,
        network,
        contract_address,
    } = AssetFixture::jitosol_test_asset();

    let mock_server = MockServer::start().await;
    let TestApp {
        appstate: state,
        router: _,
        db: _db,
    } = TestApp::with_mock_cg_uri(&mock_server.uri()).await;

    state
        .services
        .asset_service
        .insert_asset(
            symbol.clone(),
            name.to_string(),
            network,
            contract_address.clone(),
        )
        .await
        .unwrap();
    let result = state.services.asset_service.get_all_assets().await.unwrap();

    assert_eq!(
        result
            .into_iter()
            .map(|asset| (
                asset.symbol,
                asset.name,
                asset.network,
                asset.contract_address
            ))
            .collect_vec()[0],
        (symbol, name.to_string(), network, contract_address)
    );
}
