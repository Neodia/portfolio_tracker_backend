use rust_decimal::Decimal;
use crate::model::ids::{AssetId, HoldingId, UserId};

pub struct UserHolding {
    pub id: HoldingId,
    pub user_id: UserId,
    pub asset_id: AssetId,
    pub amount: Decimal,
    pub description: Option<String>,
}