use crate::repository::live::{LiveAssetRepository, LiveUserRepository};
use crate::service::asset::AssetService;
use crate::service::user::UserService;

pub mod asset;

pub mod model;
pub mod user;

#[derive(Clone)]
pub struct Services {
    pub asset_service: AssetService<LiveAssetRepository>,
    pub user_service: UserService<LiveUserRepository>,
}
