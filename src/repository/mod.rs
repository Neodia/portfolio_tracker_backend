pub mod error;
pub mod live;
pub mod repository;

pub use repository::AssetRepository;
pub use repository::OutboxRepository;
pub use repository::RateRepository;
pub use repository::UserRepository;

mod mapper;
mod model;

use crate::repository::error::DBError;
use crate::repository::live::{
    LiveAssetRepository, LiveOutboxRepository, LiveRateRepository, LiveUserRepository,
};
use sqlx::{PgPool, PgTransaction};

#[derive(Clone)]
pub struct Repositories {
    pool: PgPool,
    pub asset: LiveAssetRepository,
    pub user: LiveUserRepository,
    pub rate: LiveRateRepository,
    pub outbox: LiveOutboxRepository,
}

impl Repositories {
    pub async fn connect(database_url: String) -> Result<Self, DBError> {
        let pg_pool = PgPool::connect(database_url.as_str()).await?;
        Ok(Repositories::from(pg_pool))
    }
    pub fn from(pool: PgPool) -> Self {
        Self {
            pool: pool.clone(),
            asset: LiveAssetRepository::new_from_pool(pool.clone()),
            user: LiveUserRepository::new_from_pool(pool.clone()),
            rate: LiveRateRepository::default(),
            outbox: LiveOutboxRepository::default(),
        }
    }
    pub async fn begin_transaction(&self) -> Result<PgTransaction<'_>, DBError> {
        self.pool.begin().await.map_err(DBError::from)
    }

    pub async fn commit_transaction(&self, tx: PgTransaction<'_>) -> Result<(), DBError> {
        tx.commit().await.map_err(DBError::from)
    }
}
