use crate::auth::model::Token;
use crate::auth::{create_token, hash_password, verify_password};
use crate::repository::UserRepository;
use crate::service::error::ServiceError;

#[derive(Clone)]
pub struct UserService<R: UserRepository> {
    repository: R,
    jwt_secret: String,
}

impl<R: UserRepository> UserService<R> {
    pub fn new(repository: R, jwt_secret: String) -> Self {
        Self {
            repository,
            jwt_secret,
        }
    }
    pub async fn register(&self, email: &str, raw_password: &str) -> Result<Token, ServiceError> {
        let password_hash = hash_password(raw_password)?;
        let user = self
            .repository
            .insert_user(email, password_hash.as_str())
            .await?;

        let token = create_token(user.id, self.jwt_secret.as_str())?;
        Ok(token)
    }
    pub async fn login(&self, email: &str, raw_password: &str) -> Result<Token, ServiceError> {
        let user = self
            .repository
            .get_user(email)
            .await?
            .ok_or(ServiceError::UserNotFound(email.to_string()))?;

        verify_password(raw_password, &user.password_hash)?;

        let token = create_token(user.id, self.jwt_secret.as_str())?;
        Ok(token)
    }
}
