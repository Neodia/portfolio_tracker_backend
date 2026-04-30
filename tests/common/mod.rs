use axum::Router;
use portfolio_tracker_backend::api::router::create_router;
use portfolio_tracker_backend::appstate::AppState;
use portfolio_tracker_backend::service::model::Token;
use sqlx::PgPool;
use testcontainers::ImageExt;
use testcontainers::runners::AsyncRunner;
use testcontainers_modules::postgres::Postgres;
use uuid::Uuid;

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
    ) -> Uuid {
        sqlx::query_scalar!(
            "INSERT INTO assets (id, symbol, name, network, contract_address) VALUES (gen_random_uuid(), $1, $2, $3, $4) RETURNING id",
            symbol,
            name,
            network,
            contract
        )
            .fetch_one(&self.pool)
        .await
        .unwrap()
    }
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
