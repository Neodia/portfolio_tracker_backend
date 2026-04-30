use sqlx::PgPool;
use testcontainers::runners::AsyncRunner;
use testcontainers::ImageExt;
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
