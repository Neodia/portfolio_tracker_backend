use crate::common::TestApp;
use axum::body::Body;
use axum::http::{Request, StatusCode, header};
use serde_json::json;
use tower::ServiceExt;

#[tokio::test]
async fn register_returns_200_with_token() {
    let TestApp {
        appstate: _,
        router: app,
        db: _db,
    } = TestApp::new().await;

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/auth/register")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    json!({
                        "email": "test@test.com",
                        "password": "password123"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert!(json["token"].as_str().is_some());
    assert_eq!(json["token_type"], "Bearer");
}

#[tokio::test]
async fn email_validation_returns_bad_request() {
    let TestApp {
        appstate: _,
        router: app,
        db: _db,
    } = TestApp::new().await;

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/auth/register")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    json!({
                        "email": "this ain't an email",
                        "password": "password123"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn password_validation_returns_bad_request() {
    let TestApp {
        appstate: _,
        router: app,
        db: _db,
    } = TestApp::new().await;

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/auth/register")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    json!({
                        "email": "test@test.com",
                        "password": "smol"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn register_duplicate_email_returns_conflict() {
    let TestApp {
        appstate: _,
        router: app,
        db: _db,
    } = TestApp::new().await;

    let registration_f = || {
        let app = app.clone();
        async move {
            app.oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/auth/register")
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(
                        json!({
                            "email": "test@test.com",
                            "password": "password123"
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap()
        }
    };

    assert_eq!(registration_f().await.status(), StatusCode::OK);
    // Second registration with same credentials fails
    assert_eq!(registration_f().await.status(), StatusCode::CONFLICT);
}

#[tokio::test]
async fn login_with_valid_credentials_returns_token() {
    let TestApp {
        appstate: _,
        router: app,
        db: _db,
    } = TestApp::new().await;

    // Register
    app.clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/auth/register")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    json!({
                        "email": "test@test.com",
                        "password": "password123"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    // Login
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/auth/login")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    json!({
                        "email": "test@test.com",
                        "password": "password123"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn login_with_wrong_password_returns_not_found() {
    let TestApp {
        appstate: _,
        router: app,
        db: _db,
    } = TestApp::new().await;

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/auth/login")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    json!({
                        "email": "not_existing_user@test.com",
                        "password": "wrongpassword"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
