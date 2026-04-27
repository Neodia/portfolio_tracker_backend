use crate::repository::live::AssetRepository;
use crate::service::asset::AssetService;
use crate::service::Services;

#[derive(Clone)]
pub struct AppState {
    pub services: Services,
}

impl AppState {
    pub fn new(repo: AssetRepository) -> Self {
        Self {
            services: Services {
                asset_service: AssetService::new(repo),
            },
        }
    }
}