use crate::model::{Contract, Network, Symbol};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Clone, Debug, Validate)]
pub struct RegisterRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
}
#[derive(Deserialize, Clone, Debug, Validate)]
pub struct LoginRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    #[validate(length(min = 8, message = "Wrong password"))]
    // Anything below the min require password length will be wrong anyway
    pub password: String,
}
#[derive(Serialize)]
pub struct TokenResponse {
    pub token: String,
    pub token_type: String, // always "Bearer"
}
impl TokenResponse {
    pub fn new(token: String) -> Self {
        Self {
            token,
            token_type: "Bearer".to_string(),
        }
    }
}

#[derive(Deserialize, Clone, Debug, Validate)]
pub struct AddAssetRequest {
    pub symbol: Symbol,
    pub name: String,
    pub network: Network,
    pub contract_address: Contract,
}

#[derive(Deserialize, Clone, Debug, Validate)]
pub struct AddExpectedAllocationRequest {
    pub expected_allocation_pct: Decimal,
}
#[derive(Deserialize, Clone, Debug, Validate)]
pub struct AddHoldingRequest {
    pub asset_id: Uuid,
    pub amount: Decimal,
    pub description: Option<String>,
}
#[derive(Deserialize, Clone, Debug, Validate)]
pub struct UpdateHoldingRequest {
    pub amount: Decimal,
    pub description: Option<String>,
}