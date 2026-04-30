use crate::model::{Asset, Contract, Network, Symbol};
use crate::repository::AssetRepository;
use crate::repository::error::DBError;
use crate::repository::model::BlockchainAssetDTO;
use sqlx::PgPool;

#[derive(Clone)]
pub struct LiveAssetRepository {
    pool: PgPool,
}

impl LiveAssetRepository {
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

    async fn insert_asset(
        &self,
        symbol: Symbol,
        name: String,
        network: Network,
        contract: Contract,
    ) -> Result<(), DBError> {
        sqlx::query!(
            "INSERT INTO assets (id, symbol, name, network, contract_address) VALUES (gen_random_uuid(), $1, $2, $3, $4)",
            symbol.0,
            name,
            network.to_id(),
            contract.0
        )
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
