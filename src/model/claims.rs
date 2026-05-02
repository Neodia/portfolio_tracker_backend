use crate::model::ids::UserId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: UserId,
    pub exp: i64,
    pub iat: i64,
}
