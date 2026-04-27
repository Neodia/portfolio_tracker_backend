use crate::repository::Repositories;
use crate::service::Services;
use crate::service::asset::AssetService;
use crate::service::user::UserService;

#[derive(Clone)]
pub struct AppState {
    pub services: Services,
}

impl AppState {
    pub fn new(repositories: Repositories, jwt_secret: String) -> Self {
        Self {
            services: Services {
                asset_service: AssetService::new(repositories.asset),
                user_service: UserService::new(repositories.user, jwt_secret),
            },
        }
    }
}
