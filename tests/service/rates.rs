use crate::common::TestApp;
use itertools::Itertools;
use portfolio_tracker_backend::model::{Contract, Network, Symbol};
use serde_json::json;
use wiremock::matchers::{method, path, path_regex};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn fetch_rates_inserts_rates_and_outbox_event() {
    let network = Network::Solana;
    let trump_symbol = Symbol::new("TRUMP");
    let trump_name = "OFFICIAL TRUMP";
    let trump_contract = Contract("6p6xgHyF7AeE6TZkSmFsko444wqoP15icUSqi2jfGiPN".into());
    let soracat_symbol = Symbol::new("SORACAT");
    let soracat_name = "SORACAT";
    let soracat_contract = Contract("2g4LS3y2myPe6vj9wTvoBE1wKqxvhnZPoZA9QU9upump".into());

    let mock_server = MockServer::start().await;

    let body = std::fs::read_to_string("tests/fixtures/get_prices_from_network_200.json")
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
        trump_name,
        network.to_id(),
        trump_contract.0.as_str(),
    )
    .await;
    db.insert_asset(
        soracat_symbol.0.as_str(),
        soracat_name,
        network.to_id(),
        soracat_contract.0.as_str(),
    )
    .await;

    let result = state.services.rates_service.fetch_rates_and_persist().await;
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
async fn fetch_rates_handles_missing_price_gracefully() {
    let mock_server = MockServer::start().await;

    // CG returns null price
    Mock::given(method("GET"))
        .and(path_regex("/onchain/networks/solana/tokens/multi/.*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "data": [{
                "attributes": {
                    "address": "DeadContractAddress123",
                    "symbol": "DEADCOIN",
                    "name": "Dead Coin",
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
    db.insert_asset("DEADCOIN", "Dead Coin", "solana", "DeadContractAddress123")
        .await;

    let result = state.services.rates_service.fetch_rates_and_persist().await;
    assert!(result.is_ok());

    let rates = sqlx::query!("SELECT * FROM rates")
        .fetch_all(&db.pool)
        .await
        .unwrap();
    assert_eq!(rates.len(), 0);
}
