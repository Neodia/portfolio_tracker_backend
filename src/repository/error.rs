use thiserror::Error;

#[derive(Error, Debug)]
pub enum DBError {
    #[error("Database Error: {0}")]
    ConnectorError(#[from] sqlx::Error),

    #[error("Network {0} could not be deserialized")]
    NetworkDeserializeError(String),

    #[error("User email {0} already exists")]
    UserEmailAlreadyExistsError(String),
}
