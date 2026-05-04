use thiserror::Error;

#[derive(Error, Debug)]
pub enum DBError {
    #[error("Database Error: {0}")]
    ConnectorError(#[from] sqlx::Error),

    #[error("Database Error: {0}")]
    MigrationError(#[from] sqlx::migrate::MigrateError),

    #[error("Network {0} could not be deserialized")]
    NetworkDeserializeError(String),
    #[error("OutboxEventType {0} could not be deserialized")]
    OutboxEventTypeDeserializeError(String),

    #[error("User email {0} already exists")]
    UserEmailAlreadyExistsError(String),
}
