use crate::model::{Asset, AssetPrice, User};
use crate::repository::error::DBError;
use chrono::{DateTime, Utc};
use sqlx::PgTransaction;
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

pub trait RateRepository: Clone {
    fn insert_rates(
        &self,
        tx: &mut PgTransaction,
        rates: Vec<AssetPrice>,
        now: DateTime<Utc>,
    ) -> impl Future<Output = Result<(), DBError>>;
}

pub trait OutboxRepository: Clone {
    fn insert_rates_inserted(
        &self,
        tx: &mut PgTransaction,
        now: DateTime<Utc>,
    ) -> impl Future<Output = Result<(), DBError>>;
}
