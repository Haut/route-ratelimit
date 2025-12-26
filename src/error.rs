//! Error types for rate limiting.

use std::time::Duration;
use thiserror::Error;

/// Errors that can occur during rate limiting.
#[derive(Debug, Error)]
pub enum RateLimitError {
    /// The request was rate limited and the configured behavior is to error.
    #[error("rate limit exceeded, retry after {0:?}")]
    RateLimited(Duration),
}

impl From<RateLimitError> for reqwest_middleware::Error {
    fn from(err: RateLimitError) -> Self {
        reqwest_middleware::Error::Middleware(err.into())
    }
}
