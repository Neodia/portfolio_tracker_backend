use chrono::{DateTime, Utc};
use crate::model::ids::OutboxEventId;

#[derive(strum::Display)]
pub enum OutboxEventType {
    RatesPersisted,
}

pub struct OutboxEvent {
    pub id: OutboxEventId,
    pub event_type: OutboxEventType,
    pub created_at: DateTime<Utc>,
    pub handled_at: Option<DateTime<Utc>>,
}