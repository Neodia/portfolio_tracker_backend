use crate::repository::OutboxRepository;
use crate::repository::error::DBError;
use chrono::{DateTime, Utc};
use sqlx::PgTransaction;

#[derive(strum::Display)]
enum OutboxEvent {
    RatesPersisted,
}
#[derive(Clone, Default)]
pub struct LiveOutboxRepository;
impl OutboxRepository for LiveOutboxRepository {
    async fn insert_rates_inserted(
        &self,
        tx: &mut PgTransaction<'_>,
        now: DateTime<Utc>,
    ) -> Result<(), DBError> {
        sqlx::query!("INSERT INTO outbox (id, event_type, created_at, handled_at) VALUES (gen_random_uuid(), $1, $2, NULL)", OutboxEvent::RatesPersisted.to_string(), now)
            .execute(tx.as_mut())
            .await?;

        Ok(())
    }
}
