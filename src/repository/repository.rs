use crate::model::Asset;
use crate::repository::error::DBError;
use std::future::Future;


pub trait Repository {
    fn get_all_assets(&self) -> impl Future<Output = Result<Vec<Asset>, DBError>>;
}
