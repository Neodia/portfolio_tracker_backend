use crate::model::ids::OutboxEventId;
use crate::model::{OutboxEvent, OutboxEventType};
use crate::repository::OutboxRepository;
use crate::repository::error::DBError;
use crate::repository::model::OutboxEventDTO;
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
            OutboxEventDTO,
            "SELECT id, event_type, created_at, handled_at FROM outbox WHERE handled_at IS NULL"
        )
        .fetch_all(&self.pool)
        .await?
        .into_iter()
        .map(TryFrom::try_from)
        .collect::<Result<Vec<OutboxEvent>, _>>()?;
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
impl TryFrom<OutboxEventDTO> for OutboxEvent {
    type Error = DBError;

    fn try_from(dto: OutboxEventDTO) -> Result<Self, Self::Error> {
        let event_type = OutboxEventType::from_str(&dto.event_type)
            .ok_or_else(|| DBError::OutboxEventTypeDeserializeError(dto.event_type))?;
        Ok(Self {
            id: dto.id,
            event_type,
            created_at: dto.created_at,
            handled_at: dto.handled_at,
        })
    }
}
