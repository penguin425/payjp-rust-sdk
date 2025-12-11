//! Error types for PAY.JP API interactions.

use serde::{Deserialize, Serialize};
use std::fmt;

/// The main error type for PAY.JP operations.
#[derive(Debug, thiserror::Error)]
pub enum PayjpError {
    /// API error returned by PAY.JP.
    #[error("API error: {0}")]
    Api(#[from] ApiError),

    /// Card-related error.
    #[error("Card error: {0}")]
    Card(CardError),

    /// Authentication error (invalid API key, etc.).
    #[error("Authentication error: {0}")]
    Auth(String),

    /// Rate limit exceeded (HTTP 429).
    #[error("Rate limit exceeded")]
    RateLimit,

    /// Network or HTTP client error.
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    /// JSON serialization/deserialization error.
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// Invalid request (missing required parameters, etc.).
    #[error("Invalid request: {0}")]
    InvalidRequest(String),

    /// URL parsing error.
    #[error("URL error: {0}")]
    Url(#[from] url::ParseError),
}

/// API error details returned by PAY.JP.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiError {
    /// HTTP status code.
    pub status: u16,

    /// Error type (e.g., "card_error", "invalid_request_error").
    #[serde(rename = "type")]
    pub error_type: String,

    /// Human-readable error message.
    pub message: String,

    /// Specific error code (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,

    /// Parameter that caused the error (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub param: Option<String>,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}] {}: {}",
            self.status, self.error_type, self.message
        )?;
        if let Some(code) = &self.code {
            write!(f, " (code: {})", code)?;
        }
        if let Some(param) = &self.param {
            write!(f, " (param: {})", param)?;
        }
        Ok(())
    }
}

impl std::error::Error for ApiError {}

/// Card-specific error details.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardError {
    /// Error code.
    pub code: String,

    /// Error message.
    pub message: String,

    /// Parameter that caused the error (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub param: Option<String>,
}

impl fmt::Display for CardError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.code, self.message)?;
        if let Some(param) = &self.param {
            write!(f, " (param: {})", param)?;
        }
        Ok(())
    }
}

impl std::error::Error for CardError {}

/// Error response wrapper from PAY.JP API.
#[derive(Debug, Deserialize)]
pub(crate) struct ErrorResponse {
    pub error: ApiError,
}

/// Result type alias for PAY.JP operations.
pub type PayjpResult<T> = Result<T, PayjpError>;
