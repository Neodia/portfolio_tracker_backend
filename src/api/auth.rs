use crate::appstate::AppState;
use crate::auth::{decode_claims, extract_bearer_token};
use crate::model::error::AppError;
use crate::model::ids::UserId;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;

pub struct AuthenticatedUser {
    pub id: UserId,
}

impl FromRequestParts<AppState> for AuthenticatedUser {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let token = extract_bearer_token(&parts.headers)?;
        let claims = decode_claims(token, state.jwt_secret.as_str())?;
        Ok(AuthenticatedUser { id: claims.sub })
    }
}
