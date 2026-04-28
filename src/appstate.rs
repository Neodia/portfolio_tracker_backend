use crate::client::live::LiveCGClient;
use crate::model::error::AppError;
use crate::repository::Repositories;
use crate::service::Services;
use crate::service::asset::AssetService;
use crate::service::rates::RatesService;
use crate::service::user::UserService;
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub repositories: Repositories,
    pub services: Services,
}

impl AppState {
    pub async fn new(
        database_url: String,
        cg_url: String,
        cg_key: String,
        jwt_secret: String,
    ) -> Result<Self, AppError> {
        let live_cg_client: LiveCGClient =
            LiveCGClient::new(cg_url.to_string(), cg_key.to_string());
        let repositories = Repositories::connect(database_url).await?;
        Ok(Self::from(repositories, live_cg_client, jwt_secret))
    }
    pub fn with_pool(pool: PgPool, cg_url: String, cg_key: String, jwt_secret: String) -> Self {
        let live_cg_client: LiveCGClient =
            LiveCGClient::new(cg_url.to_string(), cg_key.to_string());
        let repositories = Repositories::from(pool);
        Self::from(repositories, live_cg_client, jwt_secret)
    }

    fn from(repositories: Repositories, cg_client: LiveCGClient, jwt_secret: String) -> Self {
        Self {
            repositories: repositories.clone(),
            services: Services {
                asset_service: AssetService::new(repositories.asset.clone()),
                user_service: UserService::new(repositories.user.clone(), jwt_secret),
                rates_service: RatesService::new(repositories.clone(), cg_client),
            },
        }
    }
}
