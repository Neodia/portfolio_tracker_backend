use crate::common::{AssetFixture, TestApp};
use itertools::Itertools;
use portfolio_tracker_backend::model::Asset;
use serde_json::json;
use wiremock::matchers::{method, path, path_regex};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn fetch_rates_inserts_rates_and_outbox_event() {
    let Asset {
        id: _,
        symbol: trump_symbol,
        name: trump_name,
        network: _,
        contract_address: trump_contract,
    } = AssetFixture::trump_test_asset();
    let Asset {
        id: _,
        symbol: soracat_symbol,
        name: soracat_name,
        network,
        contract_address: soracat_contract,
    } = AssetFixture::soracat_test_asset();

    let mock_server = MockServer::start().await;

    let body = std::fs::read_to_string("tests/fixtures/get_rates_from_network_200.json")
        .expect("fixture file not found");
    Mock::given(method("GET"))
        .and(path(format!(
            "/onchain/networks/solana/tokens/multi/{},{}",
            trump_contract.clone(),
            soracat_contract.clone()
        )))
        .respond_with(ResponseTemplate::new(200).set_body_raw(body, "application/json"))
        .mount(&mock_server)
        .await;

    let TestApp {
        appstate: state,
        router: _,
        db,
    } = TestApp::with_mock_cg_uri(&mock_server.uri()).await;

    db.insert_asset(
        trump_symbol.0.as_str(),
        trump_name.as_str(),
        network.to_id(),
        trump_contract.0.as_str(),
    )
    .await;
    db.insert_asset(
        soracat_symbol.0.as_str(),
        soracat_name.as_str(),
        network.to_id(),
        soracat_contract.0.as_str(),
    )
    .await;

    let result = state.services.rates_service.fetch_all_rates_and_persist().await;
    assert!(result.is_ok());

    let rates = sqlx::query!("SELECT * FROM rates")
        .fetch_all(&db.pool)
        .await
        .unwrap();
    assert_eq!(
        rates
            .iter()
            .map(|record| record.rate_usd.to_string())
            .sorted()
            .collect_vec(),
        &["0.000006746080385", "7.7593694175"]
    );

    let outbox = sqlx::query!("SELECT * FROM outbox WHERE handled_at IS NULL")
        .fetch_all(&db.pool)
        .await
        .unwrap();
    assert_eq!(outbox.len(), 1);
    assert_eq!(outbox[0].event_type, "RatesPersisted");
}

#[tokio::test]
async fn fetch_rates_handles_missing_rate_gracefully() {
    let mock_server = MockServer::start().await;
    let Asset {
        id: _,
        symbol,
        name,
        network,
        contract_address,
    } = AssetFixture::jitosol_test_asset();

    // CG returns null rate
    Mock::given(method("GET"))
        .and(path_regex("/onchain/networks/solana/tokens/multi/.*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "data": [{
                "attributes": {
                    "address": contract_address.0.as_str(),
                    "symbol": symbol.0.as_str(),
                    "name": name.as_str(),
                    "price_usd": null
                }
            }]
        })))
        .mount(&mock_server)
        .await;

    let TestApp {
        appstate: state,
        router: _,
        db,
    } = TestApp::with_mock_cg_uri(&mock_server.uri()).await;
    db.insert_asset(
        symbol.0.as_str(),
        name.as_str(),
        network.to_id(),
        contract_address.0.as_str(),
    )
    .await;

    let result = state.services.rates_service.fetch_all_rates_and_persist().await;
    assert!(result.is_ok());

    let rates = sqlx::query!("SELECT * FROM rates")
        .fetch_all(&db.pool)
        .await
        .unwrap();
    assert_eq!(rates.len(), 0);
}
