use crate::repository::live::LiveAssetRepository;
use crate::service::asset::AssetService;
use crate::service::Services;

#[derive(Clone)]
pub struct AppState {
    pub services: Services,
}

impl AppState {
    pub fn new(repo: LiveAssetRepository) -> Self {
        Self {
            services: Services {
                asset_service: AssetService::new(repo),
            },
        }
    }
}