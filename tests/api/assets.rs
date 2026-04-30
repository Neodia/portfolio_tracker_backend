use crate::common::TestApp;
use axum::body::Body;
use axum::http::{Request, StatusCode, header};
use serde_json::json;
use tower::ServiceExt;

// Behavior testing is in /service

#[tokio::test]
async fn insert_asset_request_properly_decoded() {
    let testapp = TestApp::new().await;
    let expected_request = json!({
        "symbol": "TRUMP",
        "name": "OFFICIAL TRUMP",
        "network": "solana",
        "contract_address": "6p6xgHyF7AeE6TZkSmFsko444wqoP15icUSqi2jfGiPN"
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
