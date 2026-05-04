use crate::common::DBFixture;
use chrono::Utc;
use portfolio_tracker_backend::repository::OutboxRepository;
use portfolio_tracker_backend::repository::live::LiveOutboxRepository;

#[tokio::test]
async fn outbox_flow_works() {
    let db = DBFixture::new().await;
    let repo = LiveOutboxRepository::new_from_pool(db.pool.clone());
    let now = Utc::now();

    let mut tx = db.pool.begin().await.unwrap();
    repo.insert_rates_inserted(&mut tx, now).await.unwrap();
    tx.commit().await.unwrap();

    let pending_events_after_insert = repo.get_pending_rates_persisted_events().await.unwrap();
    assert_eq!(pending_events_after_insert.len(), 1);
    let pending_event = &pending_events_after_insert[0];

    let mut tx = db.pool.begin().await.unwrap();
    repo.set_pending_snapshot_as_handled(&mut tx, pending_event.id, now)
        .await
        .unwrap();
    tx.commit().await.unwrap();
    let pending_events_after_handling = repo.get_pending_rates_persisted_events().await.unwrap();
    assert_eq!(pending_events_after_handling.len(), 0);
}
