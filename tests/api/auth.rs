use crate::common::DBFixture;
use axum::Router;
use axum::body::Body;
use axum::http::{Request, StatusCode, header};
use portfolio_tracker_backend::api::router::create_router;
use portfolio_tracker_backend::appstate::AppState;
use serde_json::json;
use tower::ServiceExt;

async fn setup_app() -> (Router, DBFixture) {
    let db = DBFixture::new().await;
    (
        create_router(AppState::with_pool(
            db.pool.clone(),
            "CG_URL".into(),
            "CG_KEY".into(),
            "test_secret".to_string(),
        )),
        db,
    )
}

#[tokio::test]
async fn register_returns_200_with_token() {
    let (app, _db) = setup_app().await;

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
async fn register_duplicate_email_returns_conflict() {
    let (app, _db) = setup_app().await;

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
    let (app, _db) = setup_app().await;

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
    let (app, _db) = setup_app().await;

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
