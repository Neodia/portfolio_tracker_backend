use crate::model::Asset;
use thiserror::Error;

#[derive(Debug, Error)]
pub(super) enum ServiceError {
    #[error("Missing asset price for: {0}")]
    MissingAssetPriceError(Asset),
}
