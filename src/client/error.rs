use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("ClientError: unauthorized, check your API key")]
    Unauthorized,

    #[error("ClientError: rate limited")]
    RateLimited,

    #[error("ClientError: not found")]
    NotFound,

    #[error("ClientError: unexpected error {0}")]
    Unexpected(u16),
}
