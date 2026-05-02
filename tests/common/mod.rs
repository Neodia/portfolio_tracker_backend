use axum::Router;
use portfolio_tracker_backend::api::router::create_router;
use portfolio_tracker_backend::appstate::AppState;
use portfolio_tracker_backend::model::ids::{AssetId, HoldingId, UserId};
use portfolio_tracker_backend::model::{Asset, Contract, Network, Symbol};
use portfolio_tracker_backend::service::model::Token;
use rust_decimal::Decimal;
use sqlx::PgPool;
use testcontainers::runners::AsyncRunner;
use testcontainers::ImageExt;
use testcontainers_modules::postgres::Postgres;

pub trait IntoDecimal {
    fn d(self) -> Decimal;
}

impl IntoDecimal for &str {
    fn d(self) -> Decimal {
        Decimal::from_str_exact(self).unwrap()
    }
}

pub struct DBFixture {
    pub pool: PgPool,
    _container: testcontainers::ContainerAsync<Postgres>, // keep alive
}

impl DBFixture {
    pub async fn new() -> Self {
        let container = Postgres::default().with_tag("16").start().await.unwrap();

        let url = format!(
            "postgres://postgres:postgres@{}:{}/postgres",
            container.get_host().await.unwrap(),
            container.get_host_port_ipv4(5432).await.unwrap()
        );

        let pool = PgPool::connect(&url).await.unwrap();
        sqlx::migrate!().run(&pool).await.unwrap();

        Self {
            pool,
            _container: container,
        }
    }

    pub async fn insert_asset(
        &self,
        symbol: &str,
        name: &str,
        network: &str,
        contract: &str,
    ) -> AssetId {
        sqlx::query_scalar!(
            "INSERT INTO assets (id, symbol, name, network, contract_address) VALUES (gen_random_uuid(), $1, $2, $3, $4) RETURNING id",
            symbol,
            name,
            network,
            contract
        )
            .fetch_one(&self.pool)
            .await
            .map(From::from)
            .unwrap()
    }
    pub async fn with_test_user(&self) -> UserId {
        sqlx::query_scalar!(
            "INSERT INTO users (id, email, password_hash, created_at) VALUES (gen_random_uuid(), 'test@test.com', 'Test User', now()) RETURNING id")
            .fetch_one(&self.pool)
            .await
            .map(From::from)
            .unwrap()
    }
    pub async fn with_test_asset(&self, asset: &Asset) -> AssetId {
        sqlx::query_scalar!(
            "INSERT INTO assets (id, symbol, name, network, contract_address) VALUES ($1, $2, $3, $4, $5) RETURNING id", asset.id.0, asset.symbol.0.as_str(), asset.name.as_str(), asset.network.to_id(), asset.contract_address.0.as_str())
            .fetch_one(&self.pool)
            .await
            .map(From::from)
            .unwrap()
    }

    pub async fn get_user_holdings(&self, user_id: UserId) -> Vec<HoldingDTO> {
        sqlx::query_as!(
            HoldingDTO,
            "SELECT id, asset_id, amount, description FROM current_holdings WHERE user_id = $1",
            user_id.0
        )
        .fetch_all(&self.pool)
        .await
        .unwrap()
    }
}
#[derive(Debug)]
pub struct HoldingDTO {
    pub id: HoldingId,
    pub asset_id: AssetId,
    pub amount: Decimal,
    pub description: Option<String>,
}
pub struct TestApp {
    pub appstate: AppState,
    pub router: Router,
    pub db: DBFixture,
}
impl TestApp {
    pub async fn new() -> Self {
        let db = DBFixture::new().await;
        let appstate = AppState::with_pool(
            db.pool.clone(),
            "CG_URL".into(),
            "CG_KEY".into(),
            "test_secret".to_string(),
        );
        let router = create_router(appstate.clone());
        Self {
            appstate,
            router,
            db,
        }
    }
    pub async fn with_mock_cg_uri(mock_cg_uri: &str) -> Self {
        let db = DBFixture::new().await;
        let appstate = AppState::with_pool(
            db.pool.clone(),
            mock_cg_uri.into(),
            "CG_KEY".into(),
            "test_secret".to_string(),
        );
        let router = create_router(appstate.clone());
        Self {
            appstate,
            router,
            db,
        }
    }
    pub async fn with_auth_user(&self) -> Token {
        self.appstate
            .services
            .user_service
            .register("test@email.com", "password1234")
            .await
            .unwrap()
    }
}

pub struct AssetFixture;
impl AssetFixture {
    pub fn jitosol_test_asset() -> Asset {
        Asset::new(
            AssetId::new(),
            Symbol::new("JITOSOL"),
            "Jito Staked Sol".into(),
            Network::Solana,
            Contract::from("J1toso1uCk3RLmjorhTtrVwY9HJ7X8V9yYac6Y7kGCPn"),
        )
    }
    pub fn weth_test_asset() -> Asset {
        Asset::new(
            AssetId::new(),
            Symbol::new("WETH"),
            "Wrapped Ethereum".into(),
            Network::Ethereum,
            Contract::from("0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2"),
        )
    }
    pub fn usdc_test_asset() -> Asset {
        Asset::new(
            AssetId::new(),
            Symbol::new("USDC"),
            "USDC".into(),
            Network::Base,
            Contract::from("0x833589fcd6edb6e08f4c7c32d4f71b54bda02913"),
        )
    }

    // Next two for the CG response example
    pub fn trump_test_asset() -> Asset {
        Asset::new(
            AssetId::new(),
            Symbol::new("TRUMP"),
            "OFFICIAL TRUMP".into(),
            Network::Solana,
            Contract::from("6p6xgHyF7AeE6TZkSmFsko444wqoP15icUSqi2jfGiPN"),
        )
    }
    pub fn soracat_test_asset() -> Asset {
        Asset::new(
            AssetId::new(),
            Symbol::new("SORACAT"),
            "SORACAT".into(),
            Network::Solana,
            Contract::from("2g4LS3y2myPe6vj9wTvoBE1wKqxvhnZPoZA9QU9upump"),
        )
    }
}
