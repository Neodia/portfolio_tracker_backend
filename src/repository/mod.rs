pub mod error;
pub mod live;
pub mod repository;

pub use repository::AssetRepository;
pub use repository::UserRepository;

mod mapper;
mod model;

use crate::repository::error::DBError;
use crate::repository::live::{LiveAssetRepository, LiveUserRepository};
use sqlx::PgPool;

#[derive(Clone)]
pub struct Repositories {
    pub asset: LiveAssetRepository,
    pub user: LiveUserRepository,
}

impl Repositories {
    pub async fn connect(database_url: &str) -> Result<Self, DBError> {
        let pg_pool = PgPool::connect(database_url).await?;
        Ok(Self {
            asset: LiveAssetRepository::new_from_pool(pg_pool.clone()),
            user: LiveUserRepository::new_from_pool(pg_pool),
        })
    }
}
