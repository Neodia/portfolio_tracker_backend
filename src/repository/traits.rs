use crate::model::{Asset, AssetAllocation, AssetHoldings, AssetPrice, Contract, Network, Symbol, User};
use crate::repository::error::DBError;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use sqlx::PgTransaction;
use std::future::Future;
use uuid::Uuid;

pub trait AssetRepository: Clone {
    fn get_all_assets(&self) -> impl Future<Output = Result<Vec<Asset>, DBError>>;
    fn insert_asset(
        &self,
        symbol: Symbol,
        name: String,
        network: Network,
        contract: Contract,
    ) -> impl Future<Output = Result<(), DBError>>;
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

pub trait PortfolioRepository: Clone {
    fn upsert_expected_asset_allocation(
        &self,
        user_id: Uuid,
        asset_id: Uuid,
        percentage: Decimal,
        now: DateTime<Utc>,
    ) -> impl Future<Output = Result<(), DBError>>;
    fn delete_expected_asset_allocation(
        &self,
        user_id: Uuid,
        asset_id: Uuid,
    ) -> impl Future<Output = Result<(), DBError>>;
    fn get_expected_asset_allocations(
        &self,
        user_id: Uuid,
    ) -> impl Future<Output = Result<Vec<AssetAllocation>, DBError>>;

    fn insert_holding(
        &self,
        user_id: Uuid,
        asset_id: Uuid,
        amount: Decimal,
        description: Option<String>,
        now: DateTime<Utc>,
    ) -> impl Future<Output = Result<(), DBError>>;
    fn update_holding(
        &self,
        holding_id: Uuid,
        user_id: Uuid,
        amount: Decimal,
        description: Option<String>,
        now: DateTime<Utc>,
    ) -> impl Future<Output = Result<(), DBError>>;
    fn delete_holding(
        &self,
        holding_id: Uuid,
        user_id: Uuid,
    ) -> impl Future<Output = Result<(), DBError>>;
    fn get_holdings(&self, user_id: Uuid) -> impl Future<Output = Result<Vec<AssetHoldings>, DBError>>;
}
