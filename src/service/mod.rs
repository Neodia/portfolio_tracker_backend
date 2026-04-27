use crate::repository::live::LiveAssetRepository;
use crate::service::asset::AssetService;

pub mod asset;



#[derive(Clone)]
pub struct Services {
    pub asset_service: AssetService<LiveAssetRepository>,
}
