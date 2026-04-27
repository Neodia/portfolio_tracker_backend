use crate::model::Asset;
use crate::repository::error::DBError;
use crate::repository::model::BlockchainAssetDTO;
use crate::repository::repository::AssetRepository;
use sqlx::PgPool;

#[derive(Clone)]
pub struct LiveAssetRepository {
    pool: PgPool,
}

impl LiveAssetRepository {
    pub async fn new(url: &str) -> Result<LiveAssetRepository, DBError> {
        let pool = PgPool::connect(url).await?;
        Ok(Self { pool })
    }

    pub fn new_from_pool(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl AssetRepository for LiveAssetRepository {
    async fn get_all_assets(&self) -> Result<Vec<Asset>, DBError> {
        let result = sqlx::query_as!(
            BlockchainAssetDTO,
            "SELECT id, symbol, name, network, contract_address FROM assets",
        )
        .fetch_all(&self.pool)
        .await?
        .into_iter()
        .map(TryInto::try_into)
        .collect::<Result<Vec<Asset>, DBError>>()?;

        Ok(result)
    }
}
