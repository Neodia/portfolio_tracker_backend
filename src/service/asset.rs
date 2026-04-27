use crate::model::Asset;
use crate::model::error::AppError;
use crate::repository::AssetRepository;

#[derive(Clone)]
pub struct AssetService<R: AssetRepository> {
    repository: R,
}

impl<R: AssetRepository> AssetService<R> {
    pub fn new(repository: R) -> Self { Self { repository } }
    pub async fn get_all_assets(&self) -> Result<Vec<Asset>, AppError> {
        Ok(self.repository.get_all_assets().await?)
    }
}