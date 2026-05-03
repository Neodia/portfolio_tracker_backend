use crate::model::ids::{AssetId, HoldingId, UserId};
use rust_decimal::Decimal;

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct UserHolding {
    pub id: HoldingId,
    pub user_id: UserId,
    pub asset_id: AssetId,
    pub amount: Decimal,
    pub description: Option<String>,
}
impl UserHolding {
    pub fn new(
        id: HoldingId,
        user_id: UserId,
        asset_id: AssetId,
        amount: Decimal,
        description: Option<String>,
    ) -> Self {
        Self {
            id,
            user_id,
            asset_id,
            amount,
            description,
        }
    }
}
