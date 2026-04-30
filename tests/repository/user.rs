use crate::common::DBFixture;
use portfolio_tracker_backend::repository::error::DBError;
use portfolio_tracker_backend::repository::live::LiveUserRepository;
use portfolio_tracker_backend::repository::UserRepository;

#[tokio::test]
async fn creating_user_works() {
    let db = DBFixture::new().await;

    let repo = LiveUserRepository::new_from_pool(db.pool.clone());
    let user_email = "User@Email.com";
    repo.insert_user(user_email, "Hashed Password")
        .await
        .unwrap();
    let user = repo.get_user(user_email).await.unwrap().unwrap();

    assert_eq!(user.email, user_email.to_string(),);
}

#[tokio::test]
async fn creating_duplicate_user_returns_error() {
    let db = DBFixture::new().await;

    let repo = LiveUserRepository::new_from_pool(db.pool.clone());
    let user_email = "User@Email.com";
    repo.insert_user(user_email, "Hashed Password")
        .await
        .unwrap();
    let resp = repo.insert_user(user_email, "Hashed Password").await;

    assert!(matches!(
        resp.unwrap_err(),
        DBError::UserEmailAlreadyExistsError(_)
    ));
}
