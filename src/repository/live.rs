use crate::model::Asset;
use crate::model::User;
use crate::repository::AssetRepository;
use crate::repository::UserRepository;
use crate::repository::error::DBError;
use crate::repository::model::BlockchainAssetDTO;
use sqlx::PgPool;

#[derive(Clone)]
pub struct LiveAssetRepository {
    pool: PgPool,
}

impl LiveAssetRepository {
    pub fn new_from_pool(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl AssetRepository for LiveAssetRepository {
    async fn get_all_assets(&self) -> Result<Vec<Asset>, DBError> {
        let result = sqlx::query_as!(
            BlockchainAssetDTO,
            "SELECT id, symbol, name, network, contract_address FROM assets",
        )
        .fetch_all(&self.pool)
        .await?
        .into_iter()
        .map(TryInto::try_into)
        .collect::<Result<Vec<Asset>, DBError>>()?;

        Ok(result)
    }
}

#[derive(Clone)]
pub struct LiveUserRepository {
    pool: PgPool,
}

impl LiveUserRepository {
    pub fn new_from_pool(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl UserRepository for LiveUserRepository {
    async fn insert_user(&self, email: &str, password_hash: &str) -> Result<User, DBError> {
        let db_response = sqlx::query_as!(
            User,
            "INSERT INTO users (id, email, password_hash, created_at) VALUES (gen_random_uuid(), $1, $2, now())
            RETURNING id, email, password_hash, created_at",
            email,
            password_hash
        )
        .fetch_one(&self.pool)
        .await;

        db_response.map_err(|err| match err {
            sqlx::Error::Database(e) if e.constraint() == Some("users_email_key") => {
                DBError::UserEmailAlreadyExistsError(email.to_string())
            }
            e => DBError::from(e),
        })
    }

    async fn get_user(&self, email: &str) -> Result<Option<User>, DBError> {
        let user = sqlx::query_as!(
            User,
            "SELECT id, email, password_hash, created_at from users WHERE email = $1",
            email
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }
}
