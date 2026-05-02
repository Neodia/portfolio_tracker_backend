use crate::client::live::LiveCGClient;
use crate::repository::live::{LiveAssetRepository, LiveUserRepository};
use crate::service::asset::AssetService;
use crate::service::portfolio::PortfolioService;
use crate::service::rates::RatesService;
use crate::service::user::UserService;

pub mod asset;

pub mod error;
pub mod model;
pub mod portfolio;
pub mod rates;
pub mod user;

#[derive(Clone)]
pub struct Services {
    pub asset_service: AssetService<LiveAssetRepository>,
    pub user_service: UserService<LiveUserRepository>,
    pub rates_service: RatesService<LiveCGClient>,
    pub portfolio_service: PortfolioService,
}
