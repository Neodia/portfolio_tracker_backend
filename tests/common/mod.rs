use testcontainers::runners::AsyncRunner;
use testcontainers_modules::postgres::Postgres;
use sqlx::PgPool;

pub struct DBFixture {
    pub pool: PgPool,
    _container: testcontainers::ContainerAsync<Postgres>,  // keep alive
}

impl DBFixture {
    pub async fn new() -> Self {
        let container = Postgres::default().start().await.unwrap();

        let url = format!(
            "postgres://postgres:postgres@{}:{}/postgres",
            container.get_host().await.unwrap(),
            container.get_host_port_ipv4(5432).await.unwrap()
        );

        let pool = PgPool::connect(&url).await.unwrap();
        sqlx::migrate!().run(&pool).await.unwrap();

        Self { pool, _container: container }
    }

    pub async fn insert_assert(&self, ticker: &str, chain: &str, contract: &str) {
        sqlx::query!(
            "INSERT INTO assets (ticker, chain, contract_address) VALUES ($1, $2, $3)",
            ticker, chain, contract
        )
            .execute(&self.pool)
            .await
            .unwrap();
    }
}