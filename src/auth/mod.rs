use crate::model::Claims;
use crate::model::error::{AppError, BusinessError};
use crate::service::model::Token;
use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use axum::http::HeaderMap;
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use uuid::Uuid;

pub fn hash_password(password: &str) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(|err| AppError::PasswordError(err.to_string()))?
        .to_string();
    Ok(hash)
}

pub fn verify_password(password: &str, hash: &str) -> Result<(), AppError> {
    let parsed_hash =
        PasswordHash::new(hash).map_err(|err| AppError::PasswordError(err.to_string()))?;
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .map_err(|_| BusinessError::UserNotFoundError)?;
    Ok(())
}

pub fn create_token(user_id: Uuid, secret: &str) -> Result<Token, AppError> {
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

pub fn extract_bearer_token(headers: &HeaderMap) -> Result<&str, AppError> {
    headers
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .ok_or(AppError::BusinessError(
            crate::model::error::BusinessError::Unauthorized,
        ))
}

pub fn decode_claims(token: &str, jwt_secret: &str) -> Result<Claims, AppError> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| AppError::BusinessError(BusinessError::Unauthorized))?;
    Ok(token_data.claims)
}
