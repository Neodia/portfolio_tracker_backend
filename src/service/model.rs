pub struct SnapshotsComputationResult {
    pub number_of_users: usize,
    pub number_of_snapshots_events: usize,
}
impl SnapshotsComputationResult {
    pub fn new(number_of_users: usize, number_of_snapshots_events: usize) -> Self {
        Self {
            number_of_users,
            number_of_snapshots_events,
        }
    }
}
