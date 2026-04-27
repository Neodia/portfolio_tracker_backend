use crate::model::{Asset, User};
use crate::repository::error::DBError;
use std::future::Future;

pub trait AssetRepository: Clone {
    fn get_all_assets(&self) -> impl Future<Output = Result<Vec<Asset>, DBError>>;
}

pub trait UserRepository: Clone {
    fn insert_user(
        &self,
        email: &str,
        password_hash: &str,
    ) -> impl Future<Output = Result<User, DBError>>;
    fn get_user(&self, email: &str) -> impl Future<Output = Result<Option<User>, DBError>>;
}
