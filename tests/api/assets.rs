use crate::common::{AssetFixture, TestApp};
use axum::body::Body;
use axum::http::{Request, StatusCode, header};
use serde_json::json;
use tower::ServiceExt;
use portfolio_tracker_backend::model::Asset;
// Behavior testing is in /service

#[tokio::test]
async fn insert_asset_request_properly_decoded() {
    let testapp = TestApp::new().await;   
    let Asset { id: _, symbol, name, network, contract_address } = AssetFixture::jitosol_test_asset();

    let expected_request = json!({
        "symbol": symbol.0.as_str(),
        "name": name.as_str(),
        "network": network.to_id(),
        "contract_address": contract_address.0.as_str(),
    }); 
    let token = testapp.with_auth_user().await;

    let response = testapp
        .router
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/assets")
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, format!("Bearer {}", token.0))
                .body(Body::from(expected_request.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
