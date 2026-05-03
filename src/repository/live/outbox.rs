use crate::model::ids::OutboxEventId;
use crate::model::{OutboxEvent, OutboxEventType};
use crate::repository::OutboxRepository;
use crate::repository::error::DBError;
use chrono::{DateTime, Utc};
use sqlx::{PgPool, PgTransaction};

#[derive(Clone)]
pub struct LiveOutboxRepository {
    pool: PgPool,
}
impl LiveOutboxRepository {
    pub fn new_from_pool(pool: PgPool) -> Self {
        Self { pool }
    }
}
impl OutboxRepository for LiveOutboxRepository {
    async fn insert_rates_inserted(
        &self,
        tx: &mut PgTransaction<'_>,
        now: DateTime<Utc>,
    ) -> Result<(), DBError> {
        sqlx::query!(
            "INSERT INTO outbox (id, event_type, created_at, handled_at) VALUES (gen_random_uuid(), $1, $2, NULL)",
            OutboxEventType::RatesPersisted.to_string(),
            now,
        ).execute(tx.as_mut())
            .await?;

        Ok(())
    }

    async fn get_pending_rates_persisted_events(&self) -> Result<Vec<OutboxEvent>, DBError> {
        let events = sqlx::query_as!(
            OutboxEvent,
            "SELECT id, event_type, created_at, handled_at FROM outbox WHERE handled_at IS NULL"
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(events)
    }

    async fn set_pending_snapshot_as_handled(
        &self,
        tx: &mut PgTransaction<'_>,
        id: OutboxEventId,
        now: DateTime<Utc>,
    ) -> Result<(), DBError> {
        sqlx::query!("UPDATE outbox SET handled_at = $1 WHERE id = $2", now, id.0,)
            .execute(tx.as_mut())
            .await?;
        Ok(())
    }
}

impl From<String> for OutboxEventType {
    fn from(_str: String) -> Self {
        // This is ok because there's only 1 type for now. Change this with a proper DTO if adding net event types
        OutboxEventType::RatesPersisted
    }
}
