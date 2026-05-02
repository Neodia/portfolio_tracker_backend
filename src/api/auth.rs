use crate::api::error::ApiError;
use crate::appstate::AppState;
use crate::auth::{decode_claims, extract_bearer_token};
use crate::model::ids::UserId;
use crate::service::error::ServiceError;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;

pub struct AuthenticatedUser {
    pub id: UserId,
}

impl FromRequestParts<AppState> for AuthenticatedUser {
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let token = extract_bearer_token(&parts.headers).map_err(ServiceError::from)?;
        let claims = decode_claims(token, state.jwt_secret.as_str()).map_err(ServiceError::from)?;
        Ok(AuthenticatedUser { id: claims.sub })
    }
}
