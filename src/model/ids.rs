use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize)]
pub struct UserId(pub Uuid);
impl From<Uuid> for UserId {
    fn from(id: Uuid) -> Self {
        Self(id)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize)]
pub struct AssetId(pub Uuid);
impl AssetId {
    pub fn new() -> Self { Self(Uuid::new_v4()) }
}
impl From<Uuid> for AssetId {
    fn from(id: Uuid) -> Self {
        Self(id)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize)]
pub struct HoldingId(pub Uuid);
impl From<Uuid> for HoldingId {
    fn from(id: Uuid) -> Self {
        Self(id)
    }
}
impl HoldingId {
    pub fn new() -> Self { Self(Uuid::new_v4()) }
}
