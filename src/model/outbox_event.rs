use crate::model::ids::OutboxEventId;
use chrono::{DateTime, Utc};

#[derive(strum::Display)]
pub enum OutboxEventType {
    RatesPersisted,
}
impl OutboxEventType {
    pub fn opt_from_str(str: &str) -> Option<OutboxEventType> {
        match str {
            "RatesPersisted" => Some(OutboxEventType::RatesPersisted),
            _ => None,
        }
    }
}

pub struct OutboxEvent {
    pub id: OutboxEventId,
    pub event_type: OutboxEventType,
    pub created_at: DateTime<Utc>,
    pub handled_at: Option<DateTime<Utc>>,
}
