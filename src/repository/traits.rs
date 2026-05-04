use crate::model::ids::{AssetId, HoldingId, OutboxEventId, UserId};
use crate::model::{
    Asset, AssetAllocation, AssetHoldings, AssetRate, Contract, Network, OutboxEvent,
    PortfolioValueAt, Rate, Symbol, User, UserHolding,
};
use crate::repository::error::DBError;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use sqlx::PgTransaction;
use std::collections::HashMap;
use std::future::Future;

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
        rates: Vec<AssetRate>,
        now: DateTime<Utc>,
    ) -> impl Future<Output = Result<(), DBError>>;
    fn get_latest_asset_rates_at(
        &self,
        at: DateTime<Utc>,
    ) -> impl Future<Output = Result<HashMap<AssetId, Rate>, DBError>>;
}

pub trait OutboxRepository: Clone {
    fn insert_rates_inserted(
        &self,
        tx: &mut PgTransaction,
        now: DateTime<Utc>,
    ) -> impl Future<Output = Result<(), DBError>>;
    fn get_pending_rates_persisted_events(
        &self,
    ) -> impl Future<Output = Result<Vec<OutboxEvent>, DBError>>;
    fn set_pending_snapshot_as_handled(
        &self,
        tx: &mut PgTransaction<'_>,
        id: OutboxEventId,
        now: DateTime<Utc>,
    ) -> impl Future<Output = Result<(), DBError>>;
}

pub trait PortfolioRepository: Clone {
    fn upsert_expected_asset_allocation(
        &self,
        user_id: UserId,
        asset_id: AssetId,
        percentage: Decimal,
        now: DateTime<Utc>,
    ) -> impl Future<Output = Result<(), DBError>>;
    fn delete_expected_asset_allocation(
        &self,
        user_id: UserId,
        asset_id: AssetId,
    ) -> impl Future<Output = Result<(), DBError>>;
    fn get_expected_asset_allocations(
        &self,
        user_id: UserId,
    ) -> impl Future<Output = Result<Vec<AssetAllocation>, DBError>>;

    fn insert_holding(
        &self,
        user_id: UserId,
        asset_id: AssetId,
        amount: Decimal,
        description: Option<String>,
        now: DateTime<Utc>,
    ) -> impl Future<Output = Result<HoldingId, DBError>>;
    fn update_holding(
        &self,
        holding_id: HoldingId,
        user_id: UserId,
        amount: Decimal,
        description: Option<String>,
        now: DateTime<Utc>,
    ) -> impl Future<Output = Result<(), DBError>>;
    fn delete_holding(
        &self,
        holding_id: HoldingId,
        user_id: UserId,
    ) -> impl Future<Output = Result<(), DBError>>;
    fn get_holdings(
        &self,
        user_id: UserId,
    ) -> impl Future<Output = Result<Vec<AssetHoldings>, DBError>>;
    fn get_all_users_holdings(
        &self,
    ) -> impl Future<Output = Result<HashMap<UserId, Vec<UserHolding>>, DBError>>;
    fn insert_portfolio_snapshots(
        &self,
        tx: &mut PgTransaction<'_>,
        user_snapshots: Vec<(&UserId, Decimal)>,
        at: DateTime<Utc>,
    ) -> impl Future<Output = Result<(), DBError>>;
    fn get_historical_portfolio_values(
        &self,
        user_id: UserId,
    ) -> impl Future<Output = Result<Vec<PortfolioValueAt>, DBError>>;
}
