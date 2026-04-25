use crate::model::BlockchainAsset;
use crate::repository::DBError;
use std::future::Future;


pub trait Repository {
    fn get_all_assets(&self) -> impl Future<Output = Result<Vec<BlockchainAsset>, DBError>>;
}
