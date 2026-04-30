use crate::common::TestApp;
use itertools::Itertools;
use portfolio_tracker_backend::model::{Contract, Network, Symbol};
use wiremock::MockServer;

#[tokio::test]
async fn insert_and_get_asset_should_work() {
    let network = Network::Solana;
    let trump_symbol = Symbol::new("TRUMP");
    let trump_name = "OFFICIAL TRUMP";
    let trump_contract = Contract("6p6xgHyF7AeE6TZkSmFsko444wqoP15icUSqi2jfGiPN".into());

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
            trump_symbol.clone(),
            trump_name.to_string(),
            network,
            trump_contract.clone(),
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
        (
            trump_symbol,
            trump_name.to_string(),
            network,
            trump_contract,
        )
    );
}
