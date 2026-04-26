use crate::model::Asset;
use crate::repository::error::DBError;
use crate::repository::model::BlockchainAssetDTO;
use crate::repository::repository::Repository;
use sqlx::PgPool;

pub struct AssetRepository {
    pool: PgPool,
}

impl AssetRepository {
    pub async fn new(url: &str) -> Result<AssetRepository, DBError> {
        let pool = PgPool::connect(url).await?;
        Ok(Self { pool })
    }

    pub fn new_from_pool(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl Repository for AssetRepository {
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
