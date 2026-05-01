use portfolio_tracker_backend::client::CGClient;
use portfolio_tracker_backend::client::error::ClientError;
use portfolio_tracker_backend::client::live::LiveCGClient;
use portfolio_tracker_backend::client::model::BlockchainAssetPrice;
use portfolio_tracker_backend::model::*;
use reqwest::StatusCode;
use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};
use crate::common::AssetFixture;

#[tokio::test]
async fn get_prices_per_network_returns_mapped_response() {
    let mock_server = MockServer::start().await;
    let body = std::fs::read_to_string("tests/fixtures/get_prices_from_network_200.json")
        .expect("fixture file not found");
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

    Mock::given(method("GET"))
        .and(path(format!(
            "/onchain/networks/solana/tokens/multi/{},{}",
            trump_contract.clone(),
            soracat_contract.clone()
        )))
        .respond_with(ResponseTemplate::new(200).set_body_raw(body, "application/json"))
        .mount(&mock_server)
        .await;

    let client = LiveCGClient::new(mock_server.uri(), "fake_key".into());
    let response = client
        .get_prices_from_network(
            network,
            vec![trump_contract.clone(), soracat_contract.clone()],
        )
        .await
        .unwrap();

    assert_eq!(
        response.prices,
        vec!(
            BlockchainAssetPrice::new(
                trump_symbol,
                trump_name,
                network,
                trump_contract,
                Decimal::from_f64(7.7593694175f64).unwrap(),
            ),
            BlockchainAssetPrice::new(
                soracat_symbol,
                soracat_name,
                network,
                soracat_contract,
                Decimal::from_f64(0.000006746080385f64).unwrap(),
            ),
        )
    );
}

#[tokio::test]
async fn get_prices_per_network_http_fails() {
    let client = LiveCGClient::new("whatever_fake_server.com".into(), "fake_key".into());
    let response = client
        .get_prices_from_network(Network::Solana, vec!["whatever".into()])
        .await;
    assert!(matches!(response, Err(ClientError::HttpError(_))))
}

#[tokio::test]
async fn get_prices_per_network_unauthorized() {
    let mock_server = MockServer::start().await;

    let network = Network::Solana;
    let trump_contract = Contract("6p6xgHyF7AeE6TZkSmFsko444wqoP15icUSqi2jfGiPN".into());
    let soracat_contract = Contract("2g4LS3y2myPe6vj9wTvoBE1wKqxvhnZPoZA9QU9upump".into());

    Mock::given(method("GET"))
        .and(path(format!(
            "/onchain/networks/solana/tokens/multi/{},{}",
            trump_contract.clone(),
            soracat_contract.clone()
        )))
        .respond_with(ResponseTemplate::new(StatusCode::UNAUTHORIZED))
        .mount(&mock_server)
        .await;

    let client = LiveCGClient::new(mock_server.uri(), "fake_key".into());
    let response = client
        .get_prices_from_network(
            network,
            vec![trump_contract.clone(), soracat_contract.clone()],
        )
        .await;

    assert!(matches!(response, Err(ClientError::Unauthorized)));
}

#[tokio::test]
async fn get_prices_per_network_rate_limited() {
    let mock_server = MockServer::start().await;

    let network = Network::Solana;
    let trump_contract = Contract("6p6xgHyF7AeE6TZkSmFsko444wqoP15icUSqi2jfGiPN".into());
    let soracat_contract = Contract("2g4LS3y2myPe6vj9wTvoBE1wKqxvhnZPoZA9QU9upump".into());

    Mock::given(method("GET"))
        .and(path(format!(
            "/onchain/networks/solana/tokens/multi/{},{}",
            trump_contract.clone(),
            soracat_contract.clone()
        )))
        .respond_with(ResponseTemplate::new(StatusCode::TOO_MANY_REQUESTS))
        .mount(&mock_server)
        .await;

    let client = LiveCGClient::new(mock_server.uri(), "fake_key".into());
    let response = client
        .get_prices_from_network(
            network,
            vec![trump_contract.clone(), soracat_contract.clone()],
        )
        .await;

    assert!(matches!(response, Err(ClientError::RateLimited)));
}

#[tokio::test]
async fn get_prices_per_network_not_found() {
    let mock_server = MockServer::start().await;

    let network = Network::Solana;
    let trump_contract = Contract("6p6xgHyF7AeE6TZkSmFsko444wqoP15icUSqi2jfGiPN".into());
    let soracat_contract = Contract("2g4LS3y2myPe6vj9wTvoBE1wKqxvhnZPoZA9QU9upump".into());

    Mock::given(method("GET"))
        .and(path(format!(
            "/onchain/networks/solana/tokens/multi/{},{}",
            trump_contract.clone(),
            soracat_contract.clone()
        )))
        .respond_with(ResponseTemplate::new(StatusCode::NOT_FOUND))
        .mount(&mock_server)
        .await;

    let client = LiveCGClient::new(mock_server.uri(), "fake_key".into());
    let response = client
        .get_prices_from_network(
            network,
            vec![trump_contract.clone(), soracat_contract.clone()],
        )
        .await;

    assert!(matches!(response, Err(ClientError::NotFound)));
}

#[tokio::test]
async fn get_prices_per_network_unexpected() {
    let mock_server = MockServer::start().await;

    let network = Network::Solana;
    let trump_contract = Contract("6p6xgHyF7AeE6TZkSmFsko444wqoP15icUSqi2jfGiPN".into());
    let soracat_contract = Contract("2g4LS3y2myPe6vj9wTvoBE1wKqxvhnZPoZA9QU9upump".into());

    Mock::given(method("GET"))
        .and(path(format!(
            "/onchain/networks/solana/tokens/multi/{},{}",
            trump_contract.clone(),
            soracat_contract.clone()
        )))
        .respond_with(ResponseTemplate::new(StatusCode::INTERNAL_SERVER_ERROR))
        .mount(&mock_server)
        .await;

    let client = LiveCGClient::new(mock_server.uri(), "fake_key".into());
    let response = client
        .get_prices_from_network(
            network,
            vec![trump_contract.clone(), soracat_contract.clone()],
        )
        .await;

    assert!(matches!(response, Err(ClientError::Unexpected(500))));
}
