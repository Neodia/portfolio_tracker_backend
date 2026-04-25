use crate::model::BlockchainAsset;
use crate::repository::DBError;

pub(crate) trait Repository {
    async fn get_all_assets(&self) -> Result<Vec<BlockchainAsset>, DBError>;
}
