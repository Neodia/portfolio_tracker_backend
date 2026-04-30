use crate::model::error::{AppError, BusinessError};
use crate::repository::UserRepository;
use crate::repository::error::DBError;
use crate::service::model::Token;
use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
    pub async fn register(&self, email: &str, raw_password: &str) -> Result<Token, AppError> {
        let password_hash = hash_password(raw_password)?;
        let user = self
            .repository
            .insert_user(email, password_hash.as_str())
            .await
            .map_err(|err| match err {
                DBError::UserEmailAlreadyExistsError(_) => {
                    AppError::BusinessError(BusinessError::UserAlreadyExistsError)
                }
                e => AppError::DatabaseError(e),
            })?;

        let token = create_token(user.id, self.jwt_secret.as_str())?;
        Ok(token)
    }
    pub async fn login(&self, email: &str, raw_password: &str) -> Result<Token, AppError> {
        let user = self
            .repository
            .get_user(email)
            .await?
            .ok_or(BusinessError::UserNotFoundError)?;

        verify_password(raw_password, &user.password_hash)?;

        let token = create_token(user.id, self.jwt_secret.as_str())?;
        Ok(token)
    }
}

fn hash_password(password: &str) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(|err| AppError::PasswordError(err.to_string()))?
        .to_string();
    Ok(hash)
}

fn verify_password(password: &str, hash: &str) -> Result<(), AppError> {
    let parsed_hash =
        PasswordHash::new(hash).map_err(|err| AppError::PasswordError(err.to_string()))?;
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .map_err(|_| BusinessError::UserNotFoundError)?;
    Ok(())
}

#[derive(Serialize, Deserialize)]
struct Claims {
    pub sub: Uuid, // user_id
    pub exp: i64,  // expiry timestamp
    pub iat: i64,  // issued at
}

fn create_token(user_id: Uuid, secret: &str) -> Result<Token, AppError> {
    let now = Utc::now();
    let expiry = now + Duration::hours(24);

    let claims = Claims {
        sub: user_id,
        exp: expiry.timestamp(),
        iat: now.timestamp(),
    };

    let token = encode(
        &Header::default(), // HS256 algorithm
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )?;

    Ok(Token(token))
}
