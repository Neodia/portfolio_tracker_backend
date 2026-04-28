use crate::client::live::LiveCGClient;
use crate::repository::live::{
    LiveAssetRepository, LiveOutboxRepository, LiveRateRepository, LiveUserRepository,
};
use crate::service::asset::AssetService;
use crate::service::rates::RatesService;
use crate::service::user::UserService;

pub mod asset;

pub mod model;
pub mod rates;
pub mod user;

#[derive(Clone)]
pub struct Services {
    pub asset_service: AssetService<LiveAssetRepository>,
    pub user_service: UserService<LiveUserRepository>,
    pub rates_service: RatesService<LiveCGClient>,
}
