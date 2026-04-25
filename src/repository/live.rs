use crate::model::BlockchainAsset;
use crate::repository::DBError;
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
}

impl Repository for AssetRepository {
    async fn get_all_assets(&self) -> Result<Vec<BlockchainAsset>, DBError> {
        let result = sqlx::query_as!(
            BlockchainAssetDTO,
            "SELECT ticker, chain, contract_address FROM assets",
        )
        .fetch_all(&self.pool)
        .await?
        .into_iter()
        .map(BlockchainAsset::try_from)
        .collect::<Result<Vec<BlockchainAsset>, DBError>>()?;

        Ok(result)
    }
}
